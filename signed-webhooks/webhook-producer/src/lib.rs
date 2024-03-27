use registrations::{
    delete_all_registrations, get_all_registrations, register_webhook, Registration,
};
use serde::Serialize;
use spin_sdk::http::{IntoResponse, Method, Params, Request, Response, Router};
use spin_sdk::http_component;
mod bindings;

use spin_sdk::sqlite::Connection;
use std::str;

use crate::bindings::fermyon::hmac::sign::sign;
mod registrations;

#[derive(Debug, Serialize)]
pub struct SamplePayload {
    pub event: String,
    pub data: String,
}

#[http_component]
fn handle_simple_http_api(req: Request) -> anyhow::Result<impl IntoResponse> {
    let mut router = Router::default();
    router.post_async("/registrations", register_webhook);
    router.get("/registrations", get_all_registrations);
    router.delete("/registrations", delete_all_registrations);
    router.post_async("/fire", demonstrate_firing);
    Ok(router.handle(req))
}

async fn demonstrate_firing(_: Request, _: Params) -> anyhow::Result<impl IntoResponse> {
    let con = Connection::open_default()?;
    let res = con.execute("SELECT URL, EVENT, KEY FROM REGISTRATIONS", &[])?;
    println!("PRODUCER: Loading all CONSUMERS from database");
    let registrations: Vec<Registration> = res
        .rows()
        .into_iter()
        .map(|row| Registration {
            url: row.get::<&str>("URL").map(|v| v.to_string()).unwrap(),
            event: row.get::<&str>("EVENT").map(|v| v.to_string()).unwrap(),
            signing_key: row.get::<&str>("KEY").map(|v| v.to_string()).unwrap(),
        })
        .collect();
    for reg in registrations.into_iter() {
        let payload = serde_json::to_vec(&SamplePayload {
            event: reg.event,
            data: reg.url.clone(),
        })?;

        let signature = sign(&payload, &reg.signing_key.as_bytes())
            .map(|by| String::from_utf8(by).unwrap())
            .unwrap();
        println!(
            "PRODUCER: Sending signed payload to CONSUMER {}",
            reg.url.clone()
        );
        let req = Request::builder()
            .method(Method::Post)
            .uri(reg.url.clone())
            .header("X-Signature", signature)
            .body(payload)
            .build();

        let response: Response = spin_sdk::http::send(req).await?;

        println!(
            "PRODUCER: CONSUMER {} responded with status {}",
            reg.url.clone(),
            response.status()
        );
    }

    Ok(Response::builder().status(200).body(()).build())
}

use std::thread::sleep;
use std::time::Duration;

use anyhow::Result;
use spin_sdk::http::{IntoResponse, Params, Request, Response, ResponseBuilder, Router};
use spin_sdk::http_component;
use spin_sdk::key_value::Store;
#[http_component]
fn api(req: Request) -> Result<impl IntoResponse> {
    let mut router = Router::default();

    router.get("/", regular_endpoint);
    router.get("/slow", slow_endpoint);
    router.get("/kv", kv_endpoint);
    router.get("/400", bad_request_endpoint);
    router.get("/404", not_found_endpoint);
    router.get("/500", internal_server_errror_endpoint);
    Ok(router.handle(req))
}

fn bad_request_endpoint(_: Request, _: Params) -> Result<impl IntoResponse> {
    Ok(Response::new(400, ()))
}

fn not_found_endpoint(_: Request, _: Params) -> Result<impl IntoResponse> {
    Ok(Response::new(404, ()))
}

fn internal_server_errror_endpoint(_: Request, _: Params) -> Result<impl IntoResponse> {
    Ok(Response::new(500, ()))
}

fn regular_endpoint(_: Request, _: Params) -> Result<impl IntoResponse> {
    Ok(Response::new(200, ()))
}

fn slow_endpoint(_: Request, _: Params) -> Result<impl IntoResponse> {
    sleep(Duration::from_secs(5));
    Ok(Response::new(200, ()))
}

fn kv_endpoint(_: Request, _: Params) -> Result<impl IntoResponse> {
    let store = Store::open_default()?;

    let value = match store.get("foo")? {
        Some(v) => {
            let bytes_array: [u8; 4] = v.try_into().expect("Vec<u8> must be exactly 4 bytes long");
            i32::from_le_bytes(bytes_array)
        }
        None => {
            let value = 1_i32;
            store.set("foo", &value.to_le_bytes())?;
            value
        }
    };
    Ok(ResponseBuilder::new(200)
        .header("Content-Type", "text/plain")
        .body(format!("{value}"))
        .build())
}

use anyhow::bail;
use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};
use spin_sdk::{
    http::{self, IntoResponse, Params, Request, Response},
    sqlite::{Connection, Value},
};

#[derive(Debug, Serialize)]
pub struct VerificationPayload {
    #[serde(rename = "keyData")]
    key_data: String,
}

#[derive(Debug, Deserialize)]
pub struct RegistrationRequestModel {
    pub url: String,
    pub event: String,
}

#[derive(Debug, Serialize)]
pub struct Registration {
    pub url: String,
    pub event: String,
    #[serde(rename = "signingKey")]
    pub signing_key: String,
}

impl Registration {
    pub fn new(url: String, event: String) -> Self {
        let key = generate_key();
        Registration {
            url,
            event,
            signing_key: key,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct RegistrationResponseModel {
    #[serde(rename = "signingKey")]
    pub signing_key: String,
}

fn generate_key() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(36)
        .map(char::from)
        .collect()
}

pub fn get_all_registrations(_: Request, _: Params) -> anyhow::Result<impl IntoResponse> {
    let con = Connection::open_default()?;
    let res = con.execute("SELECT URL, EVENT, KEY FROM REGISTRATIONS", &[])?;
    let registrations: Vec<Registration> = res
        .rows()
        .into_iter()
        .map(|row| Registration {
            url: row.get::<&str>("URL").map(|v| v.to_string()).unwrap(),
            event: row.get::<&str>("EVENT").map(|v| v.to_string()).unwrap(),
            signing_key: row.get::<&str>("KEY").map(|v| v.to_string()).unwrap(),
        })
        .collect();
    let payload = serde_json::to_vec(&registrations)?;
    Ok(Response::builder().status(200).body(payload).build())
}

pub async fn register_webhook(req: Request, _params: Params) -> anyhow::Result<impl IntoResponse> {
    let Ok(model) = serde_json::from_slice::<RegistrationRequestModel>(req.body()) else {
        bail!("Error while deserializing request payload");
    };

    let con = Connection::open_default()?;
    let registration = Registration::new(model.url, model.event);
    let parameters = [
        Value::Text(registration.url.clone()),
        Value::Text(registration.event.clone()),
        Value::Text(registration.signing_key.clone()),
    ];
    _ = con.execute(
        "INSERT INTO REGISTRATIONS (URL, EVENT, KEY) VALUES (?,?,?)",
        &parameters,
    )?;
    let verification_payload = serde_json::to_vec(&VerificationPayload {
        key_data: registration.signing_key.clone(),
    })?;
    println!(
        "PRODUCER: Invoking CONSUMER for verfication (handshake) along with random key data: {}",
        registration.signing_key.clone()
    );
    let verification_request = Request::builder()
        .method(http::Method::Post)
        .uri(create_handshake_url(registration.url.clone()))
        .header("Content-Type", "application/json")
        .body(verification_payload)
        .build();
    let verification_response: Response = http::send(verification_request).await?;
    match verification_response.status() {
        200 => {
            println!("PRODUCER: Handshake with PRODUCER was successful.");
            let payload = serde_json::to_vec(&registration)?;
            Ok(Response::builder().status(201).body(payload).build())
        }
        _ => {
            println!("PRODUCER: Handshake with PRODUCER failed.");
            println!(
                "Received {} from webhook consumer",
                verification_response.status()
            );
            Ok(Response::builder().status(500).body(()).build())
        }
    }
}

pub fn delete_all_registrations(_: Request, _: Params) -> anyhow::Result<impl IntoResponse> {
    let con = Connection::open_default()?;
    con.execute("DELETE FROM REGISTRATIONS", &[])?;
    Ok(Response::builder().status(200).body(()).build())
}

fn create_handshake_url(url: String) -> String {
    format!("{}?handshake=true", url)
}

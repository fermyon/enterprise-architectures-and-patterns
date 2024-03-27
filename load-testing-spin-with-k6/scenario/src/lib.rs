use serde::Serialize;
use spin_sdk::http::{IntoResponse, Params, Request, Response, Router};
use spin_sdk::http_component;

#[http_component]
fn handle_scenario(req: Request) -> anyhow::Result<impl IntoResponse> {
    let mut router = Router::default();
    router.get("/", return_json);
    router.get("/plain", return_text);
    Ok(router.handle(req))
}

fn return_json(_: Request, _: Params) -> anyhow::Result<impl IntoResponse> {
    let payload = serde_json::to_vec(&Sample::default())?;
    Ok(Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(payload)
        .build())
}

fn return_text(_: Request, _: Params) -> anyhow::Result<impl IntoResponse> {
    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body("Some static text value")
        .build())
}

#[derive(Debug, Serialize)]
pub struct Sample {
    value: String,
    key: i8,
    active: bool,
}

impl Default for Sample {
    fn default() -> Self {
        Self {
            value: "What is the meaning of life?".to_string(),
            key: 42,
            active: true,
        }
    }
}

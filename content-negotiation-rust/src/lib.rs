use content_negotiation::Negotiate;
use service::SampleService;
use spin_sdk::{
    http::{IntoResponse, Params, Request, Response, Router},
    http_component,
};

mod content_negotiation;
mod models;
mod service;
/// A simple Spin HTTP component.
#[http_component]
fn handle_content_negotiation_rust(req: Request) -> anyhow::Result<impl IntoResponse> {
    let mut router = Router::default();
    router.get("/data", handle_get_many);
    router.get("/data/:id", handle_get_single);
    router.get("*", handle_info);
    Ok(router.handle(req))
}

fn handle_info(_: Request, _: Params) -> anyhow::Result<impl IntoResponse> {
    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "text/plain")
        .body(
            r#"Please issue any of the following requests

GET /data           -> Returns a list of items
GET /data/:id       -> Returns a single item (you can either provide any string of any int as :id)

Supported content types (by specifying the Content-Type header):

- JSON          (application/json)
- YAML          (application/yaml)
- XML           (application/xml)
- Plain Text    (text/plain)
"#,
        )
        .build())
}
fn handle_get_many(req: Request, _: Params) -> anyhow::Result<impl IntoResponse> {
    let data = SampleService::get_data();

    Ok(Response::builder()
        .status(200)
        .negotiate(&req, &data)
        .build())
}

fn handle_get_single(req: Request, _: Params) -> anyhow::Result<impl IntoResponse> {
    let data = SampleService::get_single();

    Ok(Response::builder()
        .status(200)
        .negotiate(&req, &data)
        .build())
}

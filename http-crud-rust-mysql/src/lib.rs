use api::Api;
use spin_sdk::http::{IntoResponse, Request};
use spin_sdk::http_component;

mod api;
mod handlers;
mod models;
mod persistence;

#[http_component]
fn handle_crud(req: Request) -> anyhow::Result<impl IntoResponse> {
    let api = Api::default();
    api.handle(req)
}

use spin_sdk::http::{IntoResponse, Request, Router};
use spin_sdk::http_component;

mod config;
mod handlers;
mod models;
mod request_builder;
mod response_parser;

#[http_component]
fn handle_aggregate(req: Request) -> anyhow::Result<impl IntoResponse> {
    let mut router = Router::default();
    router.get_async("/aggregates/dashboard", handlers::get_dashboard);
    Ok(router.handle(req))
}

use spin_sdk::http::{IntoResponse, Request};
use spin_sdk::{http_component, http_router};

mod config;
mod handlers;
mod models;
mod service;

/// A simple Spin HTTP component.
#[http_component]
fn handle_api(req: Request) -> anyhow::Result<impl IntoResponse> {
    let router = http_router!(
        POST "/jobs" => handlers::start_job,
        GET  "/jobs/:id" => handlers::get_job_status,
        GET "/jobs" => handlers::get_status_of_all_jobs
    );

    Ok(router.handle(req))
}

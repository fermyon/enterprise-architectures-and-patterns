use spin_sdk::http::{IntoResponse, Request, Router};
use spin_sdk::http_component;

mod handlers;
mod models;

#[http_component]
fn http_api(req: Request) -> anyhow::Result<impl IntoResponse> {
    let mut router = Router::default();
    router.get("/tasks", handlers::get_all_tasks);
    router.post("/tasks", handlers::create_task);
    router.get("/tasks/:id", handlers::get_task_by_id);
    router.post("/tasks/toggle/:id", handlers::toggle_task_by_id);
    router.get("/stats", handlers::get_all_stats);
    Ok(router.handle(req))
}

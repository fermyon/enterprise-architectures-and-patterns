use std::fmt::Display;

use spin_sdk::{
    http::{IntoResponse, Request, Router},
    http_router,
};

use crate::handlers;

pub(crate) struct Api {
    router: Router,
}

impl Api {
    pub(crate) fn handle(&self, req: Request) -> anyhow::Result<impl IntoResponse> {
        Ok(self.router.handle(req))
    }
}

impl Display for Api {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.router)
    }
}

impl Default for Api {
    fn default() -> Self {
        let router = http_router!(
            GET "/items" => handlers::get_all,
            GET "/items/:id" => handlers::get_by_id,
            POST "/items" => handlers::create_item,
            PUT "/items/:id" => handlers::update_item_by_id,
            DELETE "/items" => handlers::delete_multiple_items,
            DELETE "/items/:id" => handlers::delete_by_id
        );
        Self { router: router }
    }
}

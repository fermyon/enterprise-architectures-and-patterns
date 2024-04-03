use cache::invalidate_cache;
use db::{get_single_from_database, update_single_item_in_database};
use model::{CacheKey, CacheKeyProvider, UpdateItemModel};
use spin_sdk::http::{IntoResponse, Params, Request, Response, Router};
use spin_sdk::http_component;

use crate::cache::{get_from_cache, store_in_cache};
use crate::db::get_all_from_database;
use crate::model::Item;

mod cache;
mod db;
mod model;

/// A simple Spin HTTP component.
#[http_component]
fn handle_caching_rust(req: Request) -> anyhow::Result<impl IntoResponse> {
    let mut router = Router::default();
    router.get("/items", get_all_items);
    router.get("/items/:id", get_item_by_id);
    router.put("/items/:id", update_item_by_id);
    router.delete("/invalidate-all", invalidate_all);
    Ok(router.handle(req))
}

fn invalidate_all(_: Request, _: Params) -> anyhow::Result<impl IntoResponse> {
    Ok(match cache::invalidate_all() {
        Ok(_) => Response::new(200, ()),
        Err(_) => Response::new(500, "Error invalidating cache"),
    })
}

fn get_all_items(_: Request, _: Params) -> anyhow::Result<impl IntoResponse> {
    // 1. Check if data is in cache, if so return from there
    let key = Item::get_cache_key_for_all();

    if let Some(cached) = get_from_cache(&key) {
        return Ok(Response::builder()
            .status(200)
            .header("Content-Type", "application/json")
            .header("X-Served-From-Cache", "true")
            .body(cached)
            .build());
    }
    // 2. If not in cache load from DB as you would normally do
    let data = get_all_from_database()?;
    let payload = serde_json::to_string(&data)?;

    // 3. Store in cache (this may fail because of different reasons (e.g connectivity))
    store_in_cache(&key, payload.clone());
    // Cache failed, respond but without cache headers
    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .body(payload)
        .build())
}

fn get_item_by_id(_: Request, params: Params) -> anyhow::Result<impl IntoResponse> {
    let Some(id) = params.get("id") else {
        return Ok(Response::new(400, "no identifier provided"));
    };
    let Ok(id) = uuid::Uuid::try_parse(id) else {
        return Ok(Response::new(
            400,
            "provided identifier is not a valid UUID",
        ));
    };

    let key = CacheKey::from((id, Item::get_cache_type()));
    // 1. Check if data is in cache, if so return from there
    if let Some(cached) = get_from_cache(&key) {
        return Ok(Response::builder()
            .status(200)
            .header("Content-Type", "application/json")
            .header("X-Served-From-Cache", "true")
            .body(cached)
            .build());
    };
    // 2. Load from database
    let data = get_single_from_database(id.to_string())?;
    let payload = serde_json::to_string(&data)?;
    // 3. Set in cache
    store_in_cache(&key, payload.clone());

    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .body(payload)
        .build())
}

fn update_item_by_id(req: Request, params: Params) -> anyhow::Result<impl IntoResponse> {
    let Some(id) = params.get("id") else {
        return Ok(Response::new(400, "no identifier provided"));
    };
    let Ok(id) = uuid::Uuid::try_parse(id) else {
        return Ok(Response::new(
            400,
            "provided identifier is not a valid UUID",
        ));
    };

    let Ok(model) = serde_json::from_slice::<UpdateItemModel>(&req.body()) else {
        return Ok(Response::new(400, "Invalid payload received"));
    };
    let key = CacheKey::from((id, Item::get_cache_type()));
    // 1. invalidated the cache
    let _ = invalidate_cache(&key);
    let _ = invalidate_cache(&Item::get_cache_key_for_all());
    // Although invalidation of the cache could be achieved by simply setting
    // the corresponding data in cache again, we explicitly invalidate it
    let updated = update_single_item_in_database(id.to_string(), model.name)?;
    let payload = serde_json::to_string(&updated)?;
    store_in_cache(&key, payload.clone());
    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .body(payload)
        .build())
}

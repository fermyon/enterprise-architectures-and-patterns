use spin_sdk::http::{
    conversions::TryIntoBody, responses::not_found, IntoResponse, Params, Request, Response,
};
use uuid::Uuid;

use crate::{
    models::{BatchDeleteModel, CreateItemModel, Item, ListOfItems, UpdateItemModel, Validate},
    persistence::{self, read_all_items, read_item_by_id},
};

pub(crate) fn get_all(_: Request, _: Params) -> anyhow::Result<impl IntoResponse> {
    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .body(ListOfItems::from(read_all_items()?).try_into_body()?)
        .build())
}

pub(crate) fn get_by_id(_: Request, params: Params) -> anyhow::Result<impl IntoResponse> {
    let Some(id) = params.get("id") else {
        return Ok(Response::new(400, ()));
    };
    match read_item_by_id(id)? {
        Some(item) => Ok(Response::builder()
            .status(200)
            .header("Content-Type", "application/json")
            .body(item.try_into_body()?)
            .build()),
        None => Ok(not_found()),
    }
}

pub(crate) fn create_item(req: Request, _: Params) -> anyhow::Result<impl IntoResponse> {
    let Ok(model) = serde_json::from_slice::<CreateItemModel>(req.body()) else {
        return Ok(Response::new(400, ()));
    };
    if !model.validate() {
        return Ok(Response::new(400, ()));
    }
    let item = Item::new(Uuid::new_v4().to_string(), model.name, model.active);
    persistence::create_item(&item)?;
    Ok(Response::builder()
        .status(201)
        .header("Content-Type", "application/json")
        .body(item.try_into_body()?)
        .build())
}

pub(crate) fn update_item_by_id(req: Request, _: Params) -> anyhow::Result<impl IntoResponse> {
    let Ok(model) = serde_json::from_slice::<UpdateItemModel>(req.body()) else {
        return Ok(Response::new(400, ()));
    };
    if !model.validate() {
        return Ok(Response::new(400, ()));
    }
    let item = Item::new(Uuid::new_v4().to_string(), model.name, model.active);
    persistence::update_item(&item)?;
    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .body(item.try_into_body()?)
        .build())
}

pub(crate) fn delete_multiple_items(req: Request, _: Params) -> anyhow::Result<impl IntoResponse> {
    let Ok(model) = serde_json::from_slice::<BatchDeleteModel>(req.body()) else {
        return Ok(Response::new(400, ()));
    };
    if !model.validate() {
        return Ok(Response::new(400, ()));
    }
    persistence::delete_multiple_items(model.ids)?;
    Ok(Response::new(204, ()))
}

pub(crate) fn delete_by_id(_: Request, params: Params) -> anyhow::Result<impl IntoResponse> {
    let Some(id) = params.get("id") else {
        return Ok(Response::new(400, ()));
    };
    persistence::delete_item_by_id(id)?;
    Ok(Response::new(204, ()))
}

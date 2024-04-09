use spin_sdk::http::{responses::not_found, HeaderValue, IntoResponse, Params, Request, Response};

use crate::{
    models::CreateJobModel,
    service::{create_job, read_job_status, read_job_status_all},
};

pub fn start_job(req: Request, _: Params) -> anyhow::Result<impl IntoResponse> {
    let Ok(model) = serde_json::from_slice::<CreateJobModel>(req.body()) else {
        return Ok(Response::new(400, ()));
    };
    let Ok(created) = create_job(model) else {
        return Ok(Response::new(500, ()));
    };
    let location_header_value =
        build_location_header_value(req.header("spin-full-url"), &created.id);

    Ok(Response::builder()
        .status(201)
        .header("Content-Type", "application/json")
        .header("Location", location_header_value)
        .body(created)
        .build())
}

pub fn get_job_status(_: Request, params: Params) -> anyhow::Result<impl IntoResponse> {
    let Some(id) = params.get("id") else {
        return Ok(Response::new(400, ()));
    };
    match read_job_status(id.to_string())? {
        Some(status) => Ok(Response::builder()
            .status(200)
            .header("Content-Type", "application/json")
            .body(status)
            .build()),
        None => Ok(not_found()),
    }
}

pub fn get_status_of_all_jobs(_: Request, _: Params) -> anyhow::Result<impl IntoResponse> {
    let status = read_job_status_all()?;
    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .body(status)
        .build())
}

fn build_location_header_value(url_header: Option<&HeaderValue>, id: &str) -> String {
    if url_header.is_none() {
        return format!("/{}", id);
    }
    let url = url_header.unwrap().as_str().unwrap_or_default();
    if url.ends_with("/") {
        return format!("{}{}", url, id);
    }
    format!("{}/{}", url, id)
}

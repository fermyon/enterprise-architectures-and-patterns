use anyhow::Result;
use helpers::{get_image_from_request_body, send_jpg, Dimension};
use photon_rs::channels::{remove_blue_channel, remove_green_channel, remove_red_channel};

use photon_rs::transform::{fliph, flipv};
use photon_rs::{filters, monochrome};
use spin_sdk::http::{IntoResponse, Params, Request, Response, ResponseBuilder, Router};
use spin_sdk::http_component;

mod helpers;

#[http_component]
fn image_manipulation(req: Request) -> anyhow::Result<impl IntoResponse> {
    let mut router = Router::default();
    router.post("/api/flip/:orientation", flip_image);
    router.post("/api/remove_channel/:channel", remove_color_channel);
    router.post("/api/resize/:width", resize_image);
    router.post("/api/colorize/sepia", transform_to_sepia);
    router.post("/api/colorize/grayscale", transform_to_grayscale);
    router.post("/api/filters/:filter", apply_filter);
    router.get("/api/filters", list_available_filters);
    router.post(
        "/api/stack/grayscale/:width",
        transform_to_grayscale_and_resize,
    );
    Ok(router.handle(req))
}

fn flip_image(req: Request, p: Params) -> Result<impl IntoResponse> {
    let mut img = get_image_from_request_body(&req.body())?;
    match p.get("orientation") {
        Some("vertical") => flipv(&mut img),
        Some("horizontal") => fliph(&mut img),
        _ => return Ok(Response::new(400, ())),
    };
    send_jpg(&img)
}

fn remove_color_channel(req: Request, p: Params) -> Result<impl IntoResponse> {
    // gated by the route constraint
    let channel = p.get("channel").unwrap();
    let mut img = get_image_from_request_body(&req.body())?;
    match channel {
        "red" => remove_red_channel(&mut img, 250_u8),
        "green" => remove_green_channel(&mut img, 250_u8),
        "blue" => remove_blue_channel(&mut img, 250_u8),
        _ => {
            return Ok(Response::new(
                400,
                "invalid channel requested. Valid values are (red|green|blue)",
            ))
        }
    };
    send_jpg(&img)
}

fn resize_image(req: Request, p: Params) -> Result<impl IntoResponse> {
    let mut img = get_image_from_request_body(&req.body())?;
    let dimension = Dimension::new(p, &img)?;
    println!("Resizing image to {}", dimension);
    let resized_image = photon_rs::transform::resize(
        &mut img,
        dimension.width,
        dimension.height,
        photon_rs::transform::SamplingFilter::Nearest,
    );
    send_jpg(&resized_image)
}

fn transform_to_sepia(req: Request, _: Params) -> Result<impl IntoResponse> {
    let mut img = get_image_from_request_body(&req.body())?;
    monochrome::sepia(&mut img);
    send_jpg(&img)
}

fn transform_to_grayscale(req: Request, _: Params) -> Result<impl IntoResponse> {
    let mut img = get_image_from_request_body(&req.body())?;
    monochrome::grayscale(&mut img);
    send_jpg(&img)
}

fn transform_to_grayscale_and_resize(req: Request, p: Params) -> Result<impl IntoResponse> {
    let mut img = get_image_from_request_body(&req.body())?;
    let dimension = Dimension::new(p, &img)?;

    println!("Resizing image to {}", dimension);
    let mut new_img = photon_rs::transform::resize(
        &mut img,
        dimension.width,
        dimension.height,
        photon_rs::transform::SamplingFilter::Nearest,
    );
    monochrome::grayscale(&mut new_img);
    send_jpg(&new_img)
}

fn apply_filter(req: Request, p: Params) -> Result<impl IntoResponse> {
    let filter = p
        .get("filter")
        .and_then(|f| Some(f.to_lowercase()))
        .unwrap();
    let mut img = get_image_from_request_body(&req.body())?;

    match filter.as_str() {
        "firenze" | "golden" | "lix" | "lofi" | "neue" | "obsidian" | "pastel_pink" | "ryo"
        | "cali" | "dramatic" | "duotone_horizon" | "duotone_lilac" | "duotone_ochre"
        | "duotone_violette" => filters::filter(&mut img, filter.as_str()),
        _ => {
            return Ok(Response::new(
                400,
                "Invalid filter provided. Check GET /api/filters",
            ))
        }
    }
    send_jpg(&img)
}

fn list_available_filters(_: Request, _: Params) -> Result<impl IntoResponse> {
    let filters = vec![
        "firenze",
        "golden",
        "lix",
        "lofi",
        "neue",
        "obsidian",
        "pastel_pink",
        "ryo",
        "cali",
        "dramatic",
        "duotone_horizon",
        "duotone_lilac",
        "duotone_ochre",
        "duotone_violette",
    ];
    Ok(ResponseBuilder::new(200)
        .header("content-type", "application/json")
        .body(serde_json::to_string_pretty(&filters)?)
        .build())
}

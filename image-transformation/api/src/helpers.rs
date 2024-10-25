use std::fmt::Display;

use anyhow::{Context, Result};
use photon_rs::{native::open_image_from_bytes, PhotonImage};
use spin_sdk::http::{Params, Response, ResponseBuilder};

pub(crate) fn get_image_from_request_body(body: &[u8]) -> Result<PhotonImage> {
    open_image_from_bytes(body).with_context(|| "error loading image from body")
}

pub(crate) fn send_jpg(img: &PhotonImage) -> Result<Response> {
    let bytes = img.get_bytes_jpeg(100);
    Ok(ResponseBuilder::new(200)
        .header("content-type", "image/jpeg")
        .body(bytes)
        .build())
}

pub(crate) struct Dimension {
    pub width: u32,
    pub height: u32,
}
impl Display for Dimension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}x{})", self.width, self.height)
    }
}
impl Dimension {
    pub(crate) fn new(params: Params, img: &PhotonImage) -> anyhow::Result<Self> {
        let width = params
            .get("width")
            .with_context(|| "width not found in params")?;
        let width: u32 = width.parse().with_context(|| "Invalid width provided")?;

        // calculate desired height while keeping aspect ratio
        let height = (img.get_height() * width) / img.get_width();
        Ok(Dimension { width, height })
    }
}

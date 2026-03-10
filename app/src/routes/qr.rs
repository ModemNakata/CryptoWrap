use axum::{Router, body::Bytes, extract::Query, response::IntoResponse, routing::get};
use qrcode::QrCode;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct QrParams {
    data: String,
    #[serde(default = "default_size")]
    size: Option<String>,
}

fn default_size() -> Option<String> {
    Some("200x200".to_string())
}

fn parse_size(size: &str) -> (u32, u32) {
    let parts: Vec<&str> = size.split('x').collect();
    if parts.len() == 2 {
        let width = parts[0].parse().unwrap_or(200);
        let height = parts[1].parse().unwrap_or(200);
        (width, height)
    } else {
        (200, 200)
    }
}

async fn generate_qr(Query(params): Query<QrParams>) -> impl IntoResponse {
    let (width, height) = params
        .size
        .as_ref()
        .map(|s| parse_size(s))
        .unwrap_or((200, 200));

    let code = QrCode::new(params.data.as_bytes()).unwrap();
    let image = code
        .render::<image::Luma<u8>>()
        .quiet_zone(false)
        .min_dimensions(width, height)
        .build();

    let mut buffer = Vec::new();
    let mut cursor = std::io::Cursor::new(&mut buffer);
    image::DynamicImage::ImageLuma8(image)
        .write_to(&mut cursor, image::ImageFormat::WebP)
        .unwrap();

    ([("content-type", "image/webp")], Bytes::from(buffer))
}

pub fn router() -> Router {
    Router::new().route("/api/v1/qr", get(generate_qr))
}

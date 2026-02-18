use axum::Json;
use openssl::rand::rand_bytes;
use serde::Serialize;
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};

#[derive(Serialize, ToSchema)]
pub struct TokenResponse {
    pub token: String,
    // pub token_type: String,
}

/// Generate a secure bearer token
///
/// Returns a newly generated cryptographically secure random bearer token.
#[utoipa::path(
    post,
    path = "/token",
    responses(
        (status = 200, description = "Token generated successfully", body = TokenResponse)
    )
)]
pub async fn generate_token() -> Json<TokenResponse> {
    let mut bytes = [0u8; 32];
    rand_bytes(&mut bytes).expect("Failed to generate secure random bytes");

    // Encode as hex for the token
    let token = hex::encode(&bytes);

    Json(TokenResponse {
        token,
        // token_type: "Bearer".to_string(),
    })
}

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new().routes(routes!(generate_token))
}

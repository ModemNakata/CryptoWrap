use crate::AppState;
use axum::Json;
use axum::extract::State;
use base64::Engine;
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
pub async fn generate_token(state: State<AppState>) -> Json<TokenResponse> {
    let token_prefix = &state.token_prefix;

    let mut bytes = [0u8; 90];
    rand_bytes(&mut bytes).expect("Failed to generate secure random bytes");

    // Encode as hex for the token
    let token = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(bytes);
    let token = format!("{}{}", token_prefix, token);

    Json(TokenResponse {
        token, //token prefixed_
               // token_type: "Bearer".to_string(),
    })
}

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(generate_token))
}

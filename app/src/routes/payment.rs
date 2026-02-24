use crate::AppState;
use crate::PAYMENT_TAG;
use crate::routes::auth_helper::extract_api_key;
use axum::{Json, extract::State, http::HeaderMap, http::StatusCode};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};
use uuid::Uuid;

#[derive(Serialize, ToSchema)]
pub struct InvoiceResponse {
    #[schema(value_type = String)]
    pub invoice_uuid: Uuid,
    pub wallet_address: String,
    pub payment_amount: String,
    pub currency: Currency,
}

#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "UPPERCASE")]
pub enum Currency {
    XMR,
    // BTC , // add later
}

#[derive(Deserialize, ToSchema)]
pub struct InvoiceRequest {
    pub amount: String, // 0.15
    pub currency: Currency, // XMR / xmr // cryptocurrency / e.g. coin ,.,.,.////
                        // pub network: Option<String>, // MONERO / Monero / monero
                        // ^ let's use lowercase only
}

/// Create an invoice
///
/// Returns an invoice UUID to check the specified payment amount.
#[utoipa::path(
    post,
    path = "/create_invoice",
    tag = PAYMENT_TAG,
    security(("api_key" = [])),
    responses(
        (status = 200, description = "Invoice created successfully", body = InvoiceResponse),
        (status = 401, description = "Token is missing or invalid"),
    )
)]
pub async fn create_invoice(
    // checkout / bill
    state: State<AppState>,
    headers: HeaderMap,
    Json(invoice_request): Json<InvoiceRequest>,
) -> Result<Json<InvoiceResponse>, StatusCode> {
    let _token_id = extract_api_key(&state, &headers)
        .await
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let mock_invoice = Uuid::new_v4();

    Ok(Json(InvoiceResponse {
        invoice_uuid: mock_invoice,
        wallet_address: "...".to_string(),
        payment_amount: "0.15".to_string(),
        currency: invoice_request.currency,
    }))
}

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(create_invoice))
}

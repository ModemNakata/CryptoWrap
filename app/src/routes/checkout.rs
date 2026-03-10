use askama::Template;
use askama_web::WebTemplate;
use axum::{Router, routing::get};

#[derive(Template, WebTemplate)]
#[template(path = "checkout.html")]
struct CheckoutTemplate;

async fn checkout() -> CheckoutTemplate {
    CheckoutTemplate
}

pub fn router() -> Router {
    Router::new().route("/checkout", get(checkout))
}
// TODO: allow this to be integrated into iframe, update nginx config
// add confirmations required and confirmations count
// when required > count = payment is completed for the user
// if no confirmations required is specified -> mempool means completed
// ^ logic for checkout tempalte

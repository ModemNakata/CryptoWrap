use askama::Template;
use askama_web::WebTemplate;
use axum::{Router, routing::get};

#[derive(Template, WebTemplate)]
#[template(path = "dashboard.html")]
struct DashboardTemplate {}

async fn dashboard() -> DashboardTemplate {
    DashboardTemplate {}
}

pub fn router() -> Router {
    Router::new().route("/", get(dashboard))
}

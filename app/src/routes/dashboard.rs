use askama::Template;
use askama_web::WebTemplate;
use axum::{Router, routing::get};

#[derive(Template, WebTemplate)]
#[template(path = "dashboard.html")]
struct DashboardTemplate {}

async fn dashboard() -> DashboardTemplate {
    DashboardTemplate {}
}

#[derive(Template, WebTemplate)]
#[template(path = "auth.html")]
struct AuthTemplate {}

async fn auth() -> AuthTemplate {
    AuthTemplate {}
}

pub fn router() -> Router {
    Router::new()
        .route("/", get(auth))
        .route("/dashboard", get(dashboard))
}

// check cookie with encrypted bearer token here
// if exists - check user - if valid - let to dashboard
// - if not - back to /, with optional clearing cookie
//
// add token encryption in auth.html for new logins

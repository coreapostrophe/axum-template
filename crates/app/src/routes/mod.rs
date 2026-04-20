use axum::{routing::get, Router};

mod health;

pub fn router() -> Router {
    Router::new().route("/health", get(health::handler))
}

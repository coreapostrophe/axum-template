use axum::Router;

pub mod status;

pub fn router() -> Router {
    Router::new().merge(status::router())
}
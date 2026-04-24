use axum::{http::StatusCode, routing::get, Json, Router};
use serde::Serialize;
use sqlx::PgPool;
use tower_http::{
    cors::{self, CorsLayer},
    trace::TraceLayer,
};

use super::domains::{self, services::ServiceCollection};

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
}

async fn health_check() -> (StatusCode, Json<HealthResponse>) {
    (StatusCode::OK, Json(HealthResponse { status: "healthy" }))
}

pub fn create_router(pg_pool: PgPool) -> Router {
    let services = ServiceCollection::new(pg_pool);

    Router::<ServiceCollection>::new()
        .route("/health", get(health_check))
        .nest("/api", domains::router())
        .layer(
            CorsLayer::new()
                .allow_origin(cors::Any)
                .allow_methods(cors::Any)
                .allow_headers(cors::Any),
        )
        .layer(TraceLayer::new_for_http())
        .with_state(services)
}

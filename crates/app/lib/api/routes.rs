use axum::{
    http::{header, HeaderValue, Method, StatusCode},
    routing::get,
    Json, Router,
};
use serde::Serialize;
use sqlx::PgPool;
use tower_http::{
    cors::{self, AllowOrigin, CorsLayer},
    trace::TraceLayer,
};
use tracing::warn;

use super::domains::{self, services::ServiceCollection};

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
}

async fn health_check() -> (StatusCode, Json<HealthResponse>) {
    (StatusCode::OK, Json(HealthResponse { status: "healthy" }))
}

const DEFAULT_ALLOWED_ORIGINS: [&str; 2] = ["http://localhost:3000", "http://127.0.0.1:3000"];

fn cors_allowed_origins() -> Vec<String> {
    let configured = std::env::var("APP_API__CORS_ALLOWED_ORIGINS")
        .ok()
        .map(|value| {
            value
                .split(',')
                .map(str::trim)
                .filter(|origin| !origin.is_empty())
                .map(ToOwned::to_owned)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    if configured.is_empty() {
        return DEFAULT_ALLOWED_ORIGINS
            .iter()
            .map(|origin| (*origin).to_owned())
            .collect();
    }

    configured
}

fn cors_layer() -> CorsLayer {
    let mut cors_layer = CorsLayer::new()
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION]);

    let allowed_origins = cors_allowed_origins();

    if allowed_origins.iter().any(|origin| origin == "*") {
        return cors_layer.allow_origin(cors::Any);
    }

    let header_values = allowed_origins
        .into_iter()
        .filter_map(|origin| match HeaderValue::from_str(&origin) {
            Ok(value) => Some(value),
            Err(_) => {
                warn!(origin = %origin, "ignoring invalid cors origin");
                None
            }
        })
        .collect::<Vec<_>>();

    if !header_values.is_empty() {
        cors_layer = cors_layer.allow_origin(AllowOrigin::list(header_values));
    }

    cors_layer
}

pub fn create_router(pg_pool: PgPool) -> Router {
    let services = ServiceCollection::new(pg_pool);

    Router::<ServiceCollection>::new()
        .route("/health", get(health_check))
        .nest("/api", domains::router())
        .layer(cors_layer())
        .layer(TraceLayer::new_for_http())
        .with_state(services)
}

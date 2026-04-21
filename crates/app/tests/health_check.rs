mod helpers;

use helpers::spawn_app;
use reqwest::{header::CONTENT_TYPE, StatusCode};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct HealthResponse {
    status: String,
}

#[tokio::test]
async fn health_check_returns_200_and_expected_payload() {
    let app = spawn_app().await;

    let response = app.get("/health").await;

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response
            .headers()
            .get(CONTENT_TYPE)
            .and_then(|value| value.to_str().ok()),
        Some("application/json")
    );

    let body = response
        .json::<HealthResponse>()
        .await
        .expect("failed to parse health response body");

    assert_eq!(body.status, "healthy");
}

#[tokio::test]
async fn unknown_route_returns_404() {
    let app = spawn_app().await;

    let response = app.get("/does-not-exist").await;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

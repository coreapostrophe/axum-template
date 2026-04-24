mod helpers;

use axum_applib::api::utils::response::{data::ApiDataResponse, error::ApiErrorResponse};
use helpers::spawn_app;
use reqwest::{header::LOCATION, StatusCode};
use serde::Deserialize;
use serde_json::json;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Todo {
    id: String,
    title: String,
    completed: bool,
}

#[tokio::test]
async fn todos_crud_flow_works() {
    let app = spawn_app().await;

    let create_response = app
        .post_json("/api/todos", &json!({ "title": "write tests" }))
        .await;
    assert_eq!(create_response.status(), StatusCode::CREATED);

    let location_header = create_response
        .headers()
        .get(LOCATION)
        .and_then(|value| value.to_str().ok())
        .map(ToOwned::to_owned)
        .expect("missing location header");

    let created = create_response
        .json::<ApiDataResponse<Todo>>()
        .await
        .expect("failed to parse create response");
    assert_eq!(location_header, format!("/api/todos/{}", created.data.id));

    let list_response = app.get("/api/todos").await;
    assert_eq!(list_response.status(), StatusCode::OK);
    let listed = list_response
        .json::<ApiDataResponse<Vec<Todo>>>()
        .await
        .expect("failed to parse list response");
    assert!(listed.data.iter().any(|todo| todo.id == created.data.id));

    let get_response = app.get(&format!("/api/todos/{}", created.data.id)).await;
    assert_eq!(get_response.status(), StatusCode::OK);

    let update_response = app
        .patch_json(
            &format!("/api/todos/{}", created.data.id),
            &json!({ "title": "updated title", "completed": true }),
        )
        .await;
    assert_eq!(update_response.status(), StatusCode::OK);
    let updated = update_response
        .json::<ApiDataResponse<Todo>>()
        .await
        .expect("failed to parse update response");
    assert_eq!(updated.data.title, "updated title");
    assert!(updated.data.completed);

    let delete_response = app.delete(&format!("/api/todos/{}", created.data.id)).await;
    assert_eq!(delete_response.status(), StatusCode::OK);

    let missing_response = app.get(&format!("/api/todos/{}", created.data.id)).await;
    assert_eq!(missing_response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn create_todo_rejects_blank_title() {
    let app = spawn_app().await;

    let response = app
        .post_json("/api/todos", &json!({ "title": "   " }))
        .await;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body = response
        .json::<ApiErrorResponse>()
        .await
        .expect("failed to parse error response");
    assert_eq!(body.code, "bad_request");
}

#[tokio::test]
async fn update_todo_rejects_empty_payload() {
    let app = spawn_app().await;

    let create_response = app
        .post_json("/api/todos", &json!({ "title": "write tests" }))
        .await;
    assert_eq!(create_response.status(), StatusCode::CREATED);
    let created = create_response
        .json::<ApiDataResponse<Todo>>()
        .await
        .expect("failed to parse create response");

    let response = app
        .patch_json(&format!("/api/todos/{}", created.data.id), &json!({}))
        .await;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body = response
        .json::<ApiErrorResponse>()
        .await
        .expect("failed to parse error response");
    assert_eq!(body.code, "bad_request");
}

#[tokio::test]
async fn update_todo_rejects_blank_title() {
    let app = spawn_app().await;

    let create_response = app
        .post_json("/api/todos", &json!({ "title": "write tests" }))
        .await;
    assert_eq!(create_response.status(), StatusCode::CREATED);
    let created = create_response
        .json::<ApiDataResponse<Todo>>()
        .await
        .expect("failed to parse create response");

    let response = app
        .patch_json(
            &format!("/api/todos/{}", created.data.id),
            &json!({ "title": "  " }),
        )
        .await;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body = response
        .json::<ApiErrorResponse>()
        .await
        .expect("failed to parse error response");
    assert_eq!(body.code, "bad_request");
}

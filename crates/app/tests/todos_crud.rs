mod helpers;

use helpers::spawn_app;
use reqwest::StatusCode;
use serde::Deserialize;
use serde_json::json;

#[derive(Debug, Deserialize)]
struct ApiDataResponse<T> {
    data: T,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Todo {
    id: String,
    title: String,
    completed: bool,
}

#[tokio::test]
async fn todos_crud_flow_works() {
    if std::env::var("APP_POSTGRES__RUN_MIGRATIONS")
        .map(|value| value != "true")
        .unwrap_or(true)
    {
        eprintln!("skipping todos_crud_flow_works: set APP_POSTGRES__RUN_MIGRATIONS=true with a reachable database");
        return;
    }

    let app = spawn_app().await;

    let create_response = app
        .post_json("/api/todos", &json!({ "title": "write tests" }))
        .await;
    assert_eq!(create_response.status(), StatusCode::OK);
    let created = create_response
        .json::<ApiDataResponse<Todo>>()
        .await
        .expect("failed to parse create response");

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

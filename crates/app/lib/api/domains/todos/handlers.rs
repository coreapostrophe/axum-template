use std::sync::Arc;

use axum::{
    extract::{Path, State},
    Json,
};
use uuid::Uuid;

use crate::api::utils::response::{
    data::{ApiCreatedResponse, ApiDataResponse},
    error::{ApiError, ApiResult, MapApiError},
};

#[cfg(feature = "openapi")]
use crate::api::utils::response::error::ApiErrorResponse;

use super::{
    models::{Todo, TodoCreateInput, TodoUpdateInput},
    service::TodosService,
};

#[cfg_attr(feature = "openapi", utoipa::path(
    post,
    path = "/api/todos",
    request_body = TodoCreateInput,
    responses(
        (status = 201, description = "Todo created", body = ApiDataResponse<Todo>),
        (status = 400, description = "Invalid request payload", body = ApiErrorResponse),
        (status = 500, description = "Internal server error", body = ApiErrorResponse)
    ),
    tag = "todos"
))]
pub async fn create_todo(
    State(todos_service): State<Arc<TodosService>>,
    Json(payload): Json<TodoCreateInput>,
) -> ApiResult<ApiCreatedResponse<Todo>> {
    let payload = payload
        .normalize_and_validate()
        .map_err(|message| ApiError::BadRequest(message.to_owned()))?;

    let todo = todos_service.create_todo(payload).await.map_api_err()?;
    let location = format!("/api/todos/{}", todo.id);

    Ok(ApiCreatedResponse::new(todo, Some(location)))
}

#[cfg_attr(feature = "openapi", utoipa::path(
    get,
    path = "/api/todos",
    responses(
        (status = 200, description = "Todos retrieved", body = ApiDataResponse<Vec<Todo>>),
        (status = 500, description = "Internal server error", body = ApiErrorResponse)
    ),
    tag = "todos"
))]
pub async fn list_todos(
    State(todos_service): State<Arc<TodosService>>,
) -> ApiResult<ApiDataResponse<Vec<Todo>>> {
    let todos = todos_service.list_todos().await.map_api_err()?;
    Ok(ApiDataResponse::ok(todos))
}

#[cfg_attr(feature = "openapi", utoipa::path(
    get,
    path = "/api/todos/{todo_id}",
    params(
        ("todo_id" = Uuid, Path, description = "Todo identifier")
    ),
    responses(
        (status = 200, description = "Todo retrieved", body = ApiDataResponse<Todo>),
        (status = 404, description = "Todo not found", body = ApiErrorResponse),
        (status = 500, description = "Internal server error", body = ApiErrorResponse)
    ),
    tag = "todos"
))]
pub async fn get_todo(
    State(todos_service): State<Arc<TodosService>>,
    Path(todo_id): Path<Uuid>,
) -> ApiResult<ApiDataResponse<Todo>> {
    let todo = todos_service
        .get_todo(todo_id)
        .await
        .map_api_not_found("todo not found")?;
    Ok(ApiDataResponse::ok(todo))
}

#[cfg_attr(feature = "openapi", utoipa::path(
    patch,
    path = "/api/todos/{todo_id}",
    params(
        ("todo_id" = Uuid, Path, description = "Todo identifier")
    ),
    request_body = TodoUpdateInput,
    responses(
        (status = 200, description = "Todo updated", body = ApiDataResponse<Todo>),
        (status = 400, description = "Invalid request payload", body = ApiErrorResponse),
        (status = 404, description = "Todo not found", body = ApiErrorResponse),
        (status = 500, description = "Internal server error", body = ApiErrorResponse)
    ),
    tag = "todos"
))]
pub async fn update_todo(
    State(todos_service): State<Arc<TodosService>>,
    Path(todo_id): Path<Uuid>,
    Json(payload): Json<TodoUpdateInput>,
) -> ApiResult<ApiDataResponse<Todo>> {
    let payload = payload
        .normalize_and_validate()
        .map_err(|message| ApiError::BadRequest(message.to_owned()))?;

    let todo = todos_service
        .update_todo(todo_id, payload)
        .await
        .map_api_not_found("todo not found")?;
    Ok(ApiDataResponse::ok(todo))
}

#[cfg_attr(feature = "openapi", utoipa::path(
    delete,
    path = "/api/todos/{todo_id}",
    params(
        ("todo_id" = Uuid, Path, description = "Todo identifier")
    ),
    responses(
        (status = 200, description = "Todo deleted", body = ApiDataResponse<Todo>),
        (status = 404, description = "Todo not found", body = ApiErrorResponse),
        (status = 500, description = "Internal server error", body = ApiErrorResponse)
    ),
    tag = "todos"
))]
pub async fn delete_todo(
    State(todos_service): State<Arc<TodosService>>,
    Path(todo_id): Path<Uuid>,
) -> ApiResult<ApiDataResponse<Todo>> {
    let todo = todos_service
        .delete_todo(todo_id)
        .await
        .map_api_not_found("todo not found")?;
    Ok(ApiDataResponse::ok(todo))
}

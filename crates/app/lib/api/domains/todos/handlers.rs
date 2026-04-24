use std::sync::Arc;

use axum::{
    extract::{Path, State},
    Json,
};
use uuid::Uuid;

use crate::api::utils::response::{
    data::ApiDataResponse,
    error::{ApiResult, MapApiError},
};

use super::{
    models::{TodoCreateInput, TodoUpdateInput},
    service::TodosService,
};

pub async fn create_todo(
    State(todos_service): State<Arc<TodosService>>,
    Json(payload): Json<TodoCreateInput>,
) -> ApiResult<ApiDataResponse<super::models::Todo>> {
    let todo = todos_service.create_todo(payload).await.map_api_err()?;
    Ok(ApiDataResponse::ok(todo))
}

pub async fn list_todos(
    State(todos_service): State<Arc<TodosService>>,
) -> ApiResult<ApiDataResponse<Vec<super::models::Todo>>> {
    let todos = todos_service.list_todos().await.map_api_err()?;
    Ok(ApiDataResponse::ok(todos))
}

pub async fn get_todo(
    State(todos_service): State<Arc<TodosService>>,
    Path(todo_id): Path<Uuid>,
) -> ApiResult<ApiDataResponse<super::models::Todo>> {
    let todo = todos_service
        .get_todo(todo_id)
        .await
        .map_api_not_found("todo not found")?;
    Ok(ApiDataResponse::ok(todo))
}

pub async fn update_todo(
    State(todos_service): State<Arc<TodosService>>,
    Path(todo_id): Path<Uuid>,
    Json(payload): Json<TodoUpdateInput>,
) -> ApiResult<ApiDataResponse<super::models::Todo>> {
    let todo = todos_service
        .update_todo(todo_id, payload)
        .await
        .map_api_not_found("todo not found")?;
    Ok(ApiDataResponse::ok(todo))
}

pub async fn delete_todo(
    State(todos_service): State<Arc<TodosService>>,
    Path(todo_id): Path<Uuid>,
) -> ApiResult<ApiDataResponse<super::models::Todo>> {
    let todo = todos_service
        .delete_todo(todo_id)
        .await
        .map_api_not_found("todo not found")?;
    Ok(ApiDataResponse::ok(todo))
}

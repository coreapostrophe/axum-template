use axum::{routing::get, Router};

use super::services::ServiceCollection;

pub mod handlers;
pub mod models;
pub mod service;

pub fn router() -> Router<ServiceCollection> {
    Router::<ServiceCollection>::new()
        .route(
            "/todos",
            get(handlers::list_todos).post(handlers::create_todo),
        )
        .route(
            "/todos/{todo_id}",
            get(handlers::get_todo)
                .patch(handlers::update_todo)
                .delete(handlers::delete_todo),
        )
}

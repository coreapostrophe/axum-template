use axum::Router;

use self::{services::ServiceCollection, todos::router as todos_router};

pub mod services;
pub mod todos;

pub fn router() -> Router<ServiceCollection> {
    Router::<ServiceCollection>::new().merge(todos_router())
}

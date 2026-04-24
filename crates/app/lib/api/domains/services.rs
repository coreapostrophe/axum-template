use std::sync::Arc;

use axum::extract::FromRef;
use sqlx::PgPool;

use super::todos::service::TodosService;

#[derive(Clone)]
pub struct ServiceCollection {
    pg_pool: PgPool,
    todos_service: Arc<TodosService>,
}

impl ServiceCollection {
    pub fn new(pg_pool: PgPool) -> Self {
        let todos_service = Arc::new(TodosService::new(pg_pool.clone()));

        Self {
            pg_pool,
            todos_service,
        }
    }

    pub fn pg_pool(&self) -> PgPool {
        self.pg_pool.clone()
    }

    pub fn todos_service(&self) -> Arc<TodosService> {
        self.todos_service.clone()
    }
}

impl FromRef<ServiceCollection> for Arc<TodosService> {
    fn from_ref(input: &ServiceCollection) -> Self {
        input.todos_service()
    }
}

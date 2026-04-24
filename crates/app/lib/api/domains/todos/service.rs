use sqlx::PgPool;
use uuid::Uuid;

use crate::error::{AppErrorKind, AppResult, ResultExt};

use super::models::{Todo, TodoCreateInput, TodoUpdateInput};

#[derive(Clone)]
pub struct TodosService {
    pg_pool: PgPool,
}

impl TodosService {
    pub fn new(pg_pool: PgPool) -> Self {
        Self { pg_pool }
    }

    pub async fn create_todo(&self, input: TodoCreateInput) -> AppResult<Todo> {
        sqlx::query_as::<_, Todo>(
            r#"
            INSERT INTO todos (title)
            VALUES ($1)
            RETURNING id, title, completed, created_at, updated_at
            "#,
        )
        .bind(input.title)
        .fetch_one(&self.pg_pool)
        .await
        .app_err(AppErrorKind::Database)
    }

    pub async fn list_todos(&self) -> AppResult<Vec<Todo>> {
        sqlx::query_as::<_, Todo>(
            r#"
            SELECT id, title, completed, created_at, updated_at
            FROM todos
            ORDER BY created_at DESC
            "#,
        )
        .fetch_all(&self.pg_pool)
        .await
        .app_err(AppErrorKind::Database)
    }

    pub async fn get_todo(&self, todo_id: Uuid) -> AppResult<Todo> {
        sqlx::query_as::<_, Todo>(
            r#"
            SELECT id, title, completed, created_at, updated_at
            FROM todos
            WHERE id = $1
            "#,
        )
        .bind(todo_id)
        .fetch_one(&self.pg_pool)
        .await
        .app_err(AppErrorKind::Database)
    }

    pub async fn update_todo(&self, todo_id: Uuid, input: TodoUpdateInput) -> AppResult<Todo> {
        sqlx::query_as::<_, Todo>(
            r#"
            UPDATE todos
            SET
                title = COALESCE($2, title),
                completed = COALESCE($3, completed),
                updated_at = NOW()
            WHERE id = $1
            RETURNING id, title, completed, created_at, updated_at
            "#,
        )
        .bind(todo_id)
        .bind(input.title)
        .bind(input.completed)
        .fetch_one(&self.pg_pool)
        .await
        .app_err(AppErrorKind::Database)
    }

    pub async fn delete_todo(&self, todo_id: Uuid) -> AppResult<Todo> {
        sqlx::query_as::<_, Todo>(
            r#"
            DELETE FROM todos
            WHERE id = $1
            RETURNING id, title, completed, created_at, updated_at
            "#,
        )
        .bind(todo_id)
        .fetch_one(&self.pg_pool)
        .await
        .app_err(AppErrorKind::Database)
    }
}

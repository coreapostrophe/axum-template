use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
#[cfg(feature = "openapi")]
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct Todo {
    pub id: Uuid,
    pub title: String,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct TodoCreateInput {
    pub title: String,
}

impl TodoCreateInput {
    pub fn normalize_and_validate(mut self) -> Result<Self, &'static str> {
        self.title = self.title.trim().to_owned();

        if self.title.is_empty() {
            return Err("title cannot be empty");
        }

        Ok(self)
    }
}

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct TodoUpdateInput {
    pub title: Option<String>,
    pub completed: Option<bool>,
}

impl TodoUpdateInput {
    pub fn normalize_and_validate(mut self) -> Result<Self, &'static str> {
        if let Some(title) = self.title.as_mut() {
            *title = title.trim().to_owned();

            if title.is_empty() {
                return Err("title cannot be empty");
            }
        }

        if self.title.is_none() && self.completed.is_none() {
            return Err("at least one field must be provided");
        }

        Ok(self)
    }
}

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::error::AppError;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("{0}")]
    BadRequest(String),
    #[error("{0}")]
    NotFound(String),
    #[error("{0}")]
    Conflict(String),
    #[error("internal server error")]
    Internal,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiErrorResponse {
    pub status: String,
    pub code: String,
    pub message: String,
}

impl ApiErrorResponse {
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            status: "error".to_owned(),
            code: code.into(),
            message: message.into(),
        }
    }
}

pub type ApiResult<T> = Result<T, ApiError>;

pub trait MapApiError<T> {
    fn map_api_err(self) -> ApiResult<T>;
    fn map_api_not_found(self, message: impl Into<String>) -> ApiResult<T>;
}

impl<T> MapApiError<T> for Result<T, AppError> {
    fn map_api_err(self) -> ApiResult<T> {
        self.map_err(|error| {
            error.log_debug();
            ApiError::Internal
        })
    }

    fn map_api_not_found(self, message: impl Into<String>) -> ApiResult<T> {
        self.map_err(|error| {
            let row_not_found = error
                .source()
                .and_then(|source| source.downcast_ref::<sqlx::Error>())
                .is_some_and(|err| matches!(err, sqlx::Error::RowNotFound));

            if row_not_found {
                return ApiError::NotFound(message.into());
            }

            error.log_debug();
            ApiError::Internal
        })
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, code, message) = match self {
            Self::BadRequest(message) => (StatusCode::BAD_REQUEST, "bad_request", message),
            Self::NotFound(message) => (StatusCode::NOT_FOUND, "not_found", message),
            Self::Conflict(message) => (StatusCode::CONFLICT, "conflict", message),
            Self::Internal => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal_server_error",
                "internal server error".to_string(),
            ),
        };

        (status, Json(ApiErrorResponse::new(code, message))).into_response()
    }
}

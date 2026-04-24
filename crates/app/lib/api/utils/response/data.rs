use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ApiDataResponse<D>
where
    D: Serialize,
{
    status: &'static str,
    data: D,
}

impl<D> ApiDataResponse<D>
where
    D: Serialize,
{
    pub fn new(data: D) -> Self {
        Self {
            status: "success",
            data,
        }
    }

    pub fn ok(data: D) -> Self {
        Self::new(data)
    }
}

impl<D> IntoResponse for ApiDataResponse<D>
where
    D: Serialize,
{
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

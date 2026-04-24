use axum::{
    http::StatusCode,
    http::{header::LOCATION, HeaderValue},
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiDataResponse<D> {
    pub status: String,
    pub data: D,
}

impl<D> ApiDataResponse<D>
where
    D: Serialize,
{
    pub fn new(data: D) -> Self {
        Self {
            status: "success".to_owned(),
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

pub struct ApiCreatedResponse<D>
where
    D: Serialize,
{
    body: ApiDataResponse<D>,
    location: Option<String>,
}

impl<D> ApiCreatedResponse<D>
where
    D: Serialize,
{
    pub fn new(data: D, location: Option<String>) -> Self {
        Self {
            body: ApiDataResponse::new(data),
            location,
        }
    }
}

impl<D> IntoResponse for ApiCreatedResponse<D>
where
    D: Serialize,
{
    fn into_response(self) -> Response {
        let mut response = (StatusCode::CREATED, Json(self.body)).into_response();

        if let Some(location) = self.location {
            if let Ok(value) = HeaderValue::from_str(&location) {
                response.headers_mut().insert(LOCATION, value);
            }
        }

        response
    }
}

use std::{fs, io::Error, path::Path};

use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Axum App API",
        version = env!("CARGO_PKG_VERSION"),
        description = "OpenAPI specification for the Axum App service."
    ),
    paths(
        crate::api::routes::health_check,
        crate::api::domains::todos::handlers::create_todo,
        crate::api::domains::todos::handlers::list_todos,
        crate::api::domains::todos::handlers::get_todo,
        crate::api::domains::todos::handlers::update_todo,
        crate::api::domains::todos::handlers::delete_todo
    ),
    tags(
        (name = "health", description = "Service health endpoints"),
        (name = "todos", description = "Todo CRUD endpoints")
    )
)]
pub struct ApiDoc;

pub fn write_openapi_json(path: impl AsRef<Path>) -> Result<(), Error> {
    let path = path.as_ref();

    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        fs::create_dir_all(parent)?;
    }

    let json = serde_json::to_vec_pretty(&ApiDoc::openapi())
        .map_err(|error| Error::other(format!("failed to serialize OpenAPI document: {error}")))?;

    fs::write(path, json)
}

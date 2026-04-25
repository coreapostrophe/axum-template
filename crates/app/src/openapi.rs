use std::{env, path::PathBuf, process};

use axum_applib::api::openapi::write_openapi_json;

fn main() {
    let output_path = env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("openapi.json"));

    if let Err(error) = write_openapi_json(&output_path) {
        eprintln!(
            "failed to generate OpenAPI specification at {}: {error}",
            output_path.display()
        );
        process::exit(1);
    }

    println!("OpenAPI specification written to {}", output_path.display());
}

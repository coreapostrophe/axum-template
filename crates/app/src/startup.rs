use std::net::SocketAddr;

use tracing::info;

use crate::{
    config::AppConfig,
    error::{AppErrorKind, AppResult, ResultExt},
    routes,
};

pub async fn run(config: &AppConfig) -> AppResult<()> {
    let app = routes::router();
    let addr: SocketAddr = config
        .bind_address()
        .parse()
        .app_err(AppErrorKind::AddressParse)?;
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .app_err(AppErrorKind::Bind)?;

    info!(address = %addr, "listening");

    axum::serve(listener, app)
        .await
        .app_err(AppErrorKind::Serve)?;

    Ok(())
}

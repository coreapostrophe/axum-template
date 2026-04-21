use tokio::net::TcpListener;
use tracing::info;

use crate::{
    config::AppConfig,
    error::{AppErrorKind, AppResult, ResultExt},
    routes,
};

pub struct Server;

impl Server {
    pub async fn run(config: &AppConfig) -> AppResult<()> {
        let listener = TcpListener::bind((config.host.as_str(), config.port))
            .await
            .app_err(AppErrorKind::Bind)?;

        if let Ok(address) = listener.local_addr() {
            info!(
                requested_host = %config.host,
                requested_port = config.port,
                address = %address,
                "listening"
            );
        }

        Self::run_with_listener(listener).await
    }

    pub async fn run_with_listener(listener: TcpListener) -> AppResult<()> {
        let app = routes::router();

        axum::serve(listener, app)
            .await
            .app_err(AppErrorKind::Serve)
    }
}

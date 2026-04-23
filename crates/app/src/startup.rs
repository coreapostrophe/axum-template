use tokio::net::TcpListener;
use tracing::info;

use crate::{
    domains,
    error::{AppErrorKind, AppResult, ResultExt},
};

pub enum BindOption<'a> {
    SocketAddressString(&'a str),
    SocketAddress((&'a str, u16)),
    Listener(TcpListener),
}

impl<'a> From<&'a str> for BindOption<'a> {
    fn from(value: &'a str) -> Self {
        Self::SocketAddressString(value)
    }
}

impl<'a> From<(&'a str, u16)> for BindOption<'a> {
    fn from(value: (&'a str, u16)) -> Self {
        Self::SocketAddress(value)
    }
}

impl From<TcpListener> for BindOption<'_> {
    fn from(value: TcpListener) -> Self {
        Self::Listener(value)
    }
}

pub struct Server;

impl Server {
    pub async fn run<'a>(bind: impl Into<BindOption<'a>>) -> AppResult<()> {
        let listener = match bind.into() {
            BindOption::SocketAddressString(address) => TcpListener::bind(address)
                .await
                .app_err(AppErrorKind::Bind)?,
            BindOption::SocketAddress((host, port)) => TcpListener::bind((host, port))
                .await
                .app_err(AppErrorKind::Bind)?,
            BindOption::Listener(listener) => listener,
        };

        if let Ok(address) = listener.local_addr() {
            info!(address = %address, "listening");
        }

        let app = domains::router();

        axum::serve(listener, app)
            .await
            .app_err(AppErrorKind::Serve)
    }
}

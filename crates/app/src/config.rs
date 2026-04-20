use std::env;

use config::{Config as ConfigBuilder, Environment, File};
use serde::Deserialize;

use crate::error::{AppErrorKind, AppResult, ResultExt};

const DEFAULT_HOST: &str = "127.0.0.1";
const DEFAULT_PORT: u16 = 3000;
const DEFAULT_CONFIG_PATH: &str = "config.yaml";

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    #[serde(default)]
    pub logging: LoggingConfig,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct LoggingConfig {
    #[serde(default)]
    pub human_readable: bool,
}

impl AppConfig {
    pub fn load() -> AppResult<Self> {
        let config_path =
            env::var("APP_CONFIG_FILE").unwrap_or_else(|_| DEFAULT_CONFIG_PATH.to_owned());

        let cfg = ConfigBuilder::builder()
            .set_default("host", DEFAULT_HOST)
            .app_err(AppErrorKind::Config)?
            .set_default("port", DEFAULT_PORT)
            .app_err(AppErrorKind::Config)?
            .set_default("logging.human_readable", false)
            .app_err(AppErrorKind::Config)?
            .add_source(File::with_name(&config_path).required(false))
            .add_source(Environment::with_prefix("APP").separator("_"))
            .build()
            .app_err(AppErrorKind::Config)?;

        cfg.try_deserialize().app_err(AppErrorKind::Config)
    }

    pub fn bind_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

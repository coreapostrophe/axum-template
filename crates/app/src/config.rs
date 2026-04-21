use std::{env, path::Path};

use config::{Config as ConfigBuilder, Environment, File};
use serde::Deserialize;

use crate::error::{AppErrorKind, AppResult, ResultExt};

const DEFAULT_HOST: &str = "127.0.0.1";
const DEFAULT_PORT: u16 = 3000;
const DEFAULT_CONFIG_PATH: &str = "config.yaml";

#[derive(Debug, Clone)]
pub struct ConfigSource {
    pub config_file_path: String,
    pub config_file_from_env: bool,
    pub config_file_exists: bool,
}

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
    pub fn load() -> AppResult<(Self, ConfigSource)> {
        let (config_path, config_file_from_env) = match env::var("APP_CONFIG_FILE") {
            Ok(path) => (path, true),
            Err(_) => (DEFAULT_CONFIG_PATH.to_owned(), false),
        };

        let config_source = ConfigSource {
            config_file_path: config_path.clone(),
            config_file_from_env,
            config_file_exists: Path::new(&config_path).exists(),
        };

        let cfg = ConfigBuilder::builder()
            .set_default("host", DEFAULT_HOST)
            .app_err(AppErrorKind::Config)?
            .set_default("port", DEFAULT_PORT)
            .app_err(AppErrorKind::Config)?
            .set_default("logging.human_readable", false)
            .app_err(AppErrorKind::Config)?
            .add_source(File::with_name(&config_path).required(false))
            .add_source(
                Environment::with_prefix("APP")
                    .prefix_separator("_")
                    .separator("__"),
            )
            .build()
            .app_err(AppErrorKind::Config)?;

        let config = cfg.try_deserialize().app_err(AppErrorKind::Config)?;
        Ok((config, config_source))
    }
}

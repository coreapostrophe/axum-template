use std::{env, path::Path, time::Duration};

use config::{Config as ConfigBuilder, Environment, File};
use serde::Deserialize;
use sqlx::{postgres::PgPoolOptions, PgPool};

use crate::error::{AppErrorKind, AppResult, ResultExt};

const DEFAULT_HOST: &str = "127.0.0.1";
const DEFAULT_PORT: u16 = 3000;
const DEFAULT_CONFIG_PATH: &str = "config.yaml";
const DEFAULT_POSTGRES_HOST: &str = "127.0.0.1";
const DEFAULT_POSTGRES_PORT: u16 = 5432;
const DEFAULT_POSTGRES_USER: &str = "postgres";
const DEFAULT_POSTGRES_PASSWORD: &str = "postgres";
const DEFAULT_POSTGRES_DB: &str = "app";
const DEFAULT_POSTGRES_MAX_CONNECTIONS: u32 = 20;
const DEFAULT_POSTGRES_ACQUIRE_TIMEOUT_SECONDS: u64 = 3;
const DEFAULT_POSTGRES_RUN_MIGRATIONS: bool = false;

#[derive(Debug, Clone)]
pub struct ConfigSource {
    pub config_file_path: String,
    pub config_file_from_env: bool,
    pub config_file_exists: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub api: ApiConfig,
    pub postgres: PostgresConfig,
    #[serde(default)]
    pub logging: LoggingConfig,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct LoggingConfig {
    #[serde(default)]
    pub human_readable: bool,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct ApiConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PostgresConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub db_name: String,
    pub max_connections: u32,
    pub acquire_timeout_seconds: u64,
    pub run_migrations: bool,
}

impl PostgresConfig {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user, self.password, self.host, self.port, self.db_name
        )
    }

    pub fn pg_pool(&self) -> AppResult<PgPool> {
        PgPoolOptions::new()
            .max_connections(self.max_connections)
            .acquire_timeout(Duration::from_secs(self.acquire_timeout_seconds))
            .connect_lazy(&self.connection_string())
            .app_err(AppErrorKind::DbPool)
    }
}

impl ServerConfig {
    pub fn get() -> AppResult<(Self, ConfigSource)> {
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
            .set_default("api.host", DEFAULT_HOST)
            .app_err(AppErrorKind::Config)?
            .set_default("api.port", DEFAULT_PORT)
            .app_err(AppErrorKind::Config)?
            .set_default("logging.human_readable", false)
            .app_err(AppErrorKind::Config)?
            .set_default("postgres.host", DEFAULT_POSTGRES_HOST)
            .app_err(AppErrorKind::Config)?
            .set_default("postgres.port", DEFAULT_POSTGRES_PORT)
            .app_err(AppErrorKind::Config)?
            .set_default("postgres.user", DEFAULT_POSTGRES_USER)
            .app_err(AppErrorKind::Config)?
            .set_default("postgres.password", DEFAULT_POSTGRES_PASSWORD)
            .app_err(AppErrorKind::Config)?
            .set_default("postgres.db_name", DEFAULT_POSTGRES_DB)
            .app_err(AppErrorKind::Config)?
            .set_default("postgres.max_connections", DEFAULT_POSTGRES_MAX_CONNECTIONS)
            .app_err(AppErrorKind::Config)?
            .set_default(
                "postgres.acquire_timeout_seconds",
                DEFAULT_POSTGRES_ACQUIRE_TIMEOUT_SECONDS,
            )
            .app_err(AppErrorKind::Config)?
            .set_default("postgres.run_migrations", DEFAULT_POSTGRES_RUN_MIGRATIONS)
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

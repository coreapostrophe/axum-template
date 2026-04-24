use axum_applib::{config::ServerConfig, error::AppResult, observability, server::Server};

#[tokio::main]
async fn main() -> AppResult<()> {
    let (config, config_source) = match ServerConfig::get() {
        Ok(config) => config,
        Err(error) => {
            observability::init_tracing(false);
            error.log_debug();
            return Err(error);
        }
    };

    observability::init_tracing(config.logging.human_readable);

    tracing::info!(
        config_file_path = %config_source.config_file_path,
        config_file_exists = config_source.config_file_exists,
        config_file_from_env = config_source.config_file_from_env,
        "configuration loaded"
    );

    let pg_pool = match config.postgres.pg_pool() {
        Ok(pool) => pool,
        Err(error) => {
            error.log_debug();
            return Err(error);
        }
    };

    if config.postgres.run_migrations {
        if let Err(error) = sqlx::migrate!("./migrations").run(&pg_pool).await {
            let app_error = axum_applib::error::AppError::from_source(
                axum_applib::error::AppErrorKind::Migration,
                error,
            );
            app_error.log_debug();
            return Err(app_error);
        }
    }

    if let Err(error) = Server::run((config.api.host.as_str(), config.api.port), pg_pool).await {
        error.log_debug();
        return Err(error);
    }

    Ok(())
}

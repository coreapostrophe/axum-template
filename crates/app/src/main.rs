use axum_applib::{config::AppConfig, error::AppResult, observability, startup::Server};

#[tokio::main]
async fn main() -> AppResult<()> {
    let (config, config_source) = match AppConfig::load() {
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

    if let Err(error) = Server::run(&config).await {
        error.log_debug();
        return Err(error);
    }

    Ok(())
}

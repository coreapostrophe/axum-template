mod config;
mod error;
mod observability;
mod routes;
mod startup;

use config::AppConfig;
use error::AppResult;

#[tokio::main]
async fn main() -> AppResult<()> {
    let config = match AppConfig::load() {
        Ok(config) => config,
        Err(error) => {
            observability::init_tracing(false);
            error.log_debug();
            return Err(error);
        }
    };

    observability::init_tracing(config.logging.human_readable);

    if let Err(error) = startup::run(&config).await {
        error.log_debug();
        return Err(error);
    }

    Ok(())
}

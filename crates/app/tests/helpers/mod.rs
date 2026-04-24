use std::time::Duration;

use axum_applib::server::Server;
use serde::Serialize;
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;

pub struct TestApp {
    pub address: String,
    pub client: reqwest::Client,
}

#[allow(dead_code)]
impl TestApp {
    pub async fn get(&self, path: &str) -> reqwest::Response {
        self.client
            .get(self.url(path))
            .send()
            .await
            .unwrap_or_else(|error| panic!("request failed for {path}: {error}"))
    }

    pub async fn post_json<T: Serialize + ?Sized>(
        &self,
        path: &str,
        body: &T,
    ) -> reqwest::Response {
        self.client
            .post(self.url(path))
            .json(body)
            .send()
            .await
            .unwrap_or_else(|error| panic!("request failed for {path}: {error}"))
    }

    pub async fn patch_json<T: Serialize + ?Sized>(
        &self,
        path: &str,
        body: &T,
    ) -> reqwest::Response {
        self.client
            .patch(self.url(path))
            .json(body)
            .send()
            .await
            .unwrap_or_else(|error| panic!("request failed for {path}: {error}"))
    }

    pub async fn delete(&self, path: &str) -> reqwest::Response {
        self.client
            .delete(self.url(path))
            .send()
            .await
            .unwrap_or_else(|error| panic!("request failed for {path}: {error}"))
    }

    fn url(&self, path: &str) -> String {
        format!("{}{}", self.address, path)
    }
}

pub async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("failed to bind random port");
    let port = listener
        .local_addr()
        .expect("failed to read local address")
        .port();
    let pg_pool = PgPoolOptions::new()
        .max_connections(1)
        .connect_lazy(&test_database_url())
        .expect("failed to build lazy postgres pool for tests");

    tokio::spawn(async move {
        Server::run(listener, pg_pool)
            .await
            .unwrap_or_else(|error| panic!("test server crashed: {error}"));
    });

    let test_app = TestApp {
        address: format!("http://127.0.0.1:{port}"),
        client: reqwest::Client::builder()
            .timeout(Duration::from_secs(2))
            .build()
            .expect("failed to build reqwest client"),
    };

    wait_until_ready(&test_app).await;
    test_app
}

fn test_database_url() -> String {
    let host = std::env::var("APP_POSTGRES__HOST").unwrap_or_else(|_| "127.0.0.1".to_owned());
    let port = std::env::var("APP_POSTGRES__PORT").unwrap_or_else(|_| "5432".to_owned());
    let user = std::env::var("APP_POSTGRES__USER").unwrap_or_else(|_| "postgres".to_owned());
    let password =
        std::env::var("APP_POSTGRES__PASSWORD").unwrap_or_else(|_| "postgres".to_owned());
    let db_name = std::env::var("APP_POSTGRES__DB_NAME").unwrap_or_else(|_| "app".to_owned());

    format!(
        "postgres://{}:{}@{}:{}/{}",
        user, password, host, port, db_name
    )
}

async fn wait_until_ready(app: &TestApp) {
    for _ in 0..20 {
        if let Ok(response) = app.client.get(app.url("/health")).send().await {
            if response.status().is_success() {
                return;
            }
        }

        tokio::time::sleep(Duration::from_millis(25)).await;
    }

    panic!("test server did not become ready in time");
}

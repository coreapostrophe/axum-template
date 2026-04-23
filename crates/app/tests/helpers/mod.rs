use std::time::Duration;

use axum_applib::startup::Server;
use tokio::net::TcpListener;

pub struct TestApp {
    pub address: String,
    pub client: reqwest::Client,
}

impl TestApp {
    pub async fn get(&self, path: &str) -> reqwest::Response {
        self.client
            .get(self.url(path))
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

    tokio::spawn(async move {
        Server::run(listener)
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

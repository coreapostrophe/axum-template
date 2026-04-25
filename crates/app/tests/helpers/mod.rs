use std::{env, io, thread, time::Duration};

use axum_applib::server::Server;
use serde::Serialize;
use sqlx::postgres::PgPoolOptions;
use tokio::{net::TcpListener, task::JoinHandle};
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub client: reqwest::Client,
    server_handle: JoinHandle<()>,
    _database: TestDatabase,
}

impl Drop for TestApp {
    fn drop(&mut self) {
        self.server_handle.abort();
    }
}

#[derive(Debug, Clone)]
struct TestDatabaseConfig {
    host: String,
    port: String,
    user: String,
    password: String,
    maintenance_db_name: String,
}

impl TestDatabaseConfig {
    fn from_env() -> Self {
        Self {
            host: env::var("APP_POSTGRES__HOST").unwrap_or_else(|_| "127.0.0.1".to_owned()),
            port: env::var("APP_POSTGRES__PORT").unwrap_or_else(|_| "5432".to_owned()),
            user: env::var("APP_POSTGRES__USER").unwrap_or_else(|_| "postgres".to_owned()),
            password: env::var("APP_POSTGRES__PASSWORD").unwrap_or_else(|_| "postgres".to_owned()),
            maintenance_db_name: env::var("APP_POSTGRES__MAINTENANCE_DB")
                .unwrap_or_else(|_| "postgres".to_owned()),
        }
    }

    fn connection_string(&self, db_name: &str) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user, self.password, self.host, self.port, db_name
        )
    }

    fn maintenance_connection_string(&self) -> String {
        self.connection_string(&self.maintenance_db_name)
    }
}

#[derive(Debug)]
struct TestDatabase {
    db_name: String,
    connection_string: String,
    maintenance_connection_string: String,
}

impl TestDatabase {
    async fn create() -> Self {
        let config = TestDatabaseConfig::from_env();
        let maintenance_connection_string = config.maintenance_connection_string();
        let maintenance_pool = PgPoolOptions::new()
            .max_connections(1)
            .connect(&maintenance_connection_string)
            .await
            .expect("failed to connect to maintenance database for tests");

        let db_name = format!("test_{}", Uuid::new_v4().simple());
        let create_database_query = format!(r#"CREATE DATABASE "{}""#, db_name);

        sqlx::query(&create_database_query)
            .execute(&maintenance_pool)
            .await
            .expect("failed to create isolated test database");

        let connection_string = config.connection_string(&db_name);
        let database_pool = PgPoolOptions::new()
            .max_connections(2)
            .connect(&connection_string)
            .await
            .expect("failed to connect to isolated test database");

        sqlx::migrate!("./migrations")
            .run(&database_pool)
            .await
            .expect("failed to run test database migrations");

        database_pool.close().await;
        maintenance_pool.close().await;

        Self {
            db_name,
            connection_string,
            maintenance_connection_string,
        }
    }

    async fn drop_database(db_name: &str, maintenance_connection_string: &str) {
        let Ok(maintenance_pool) = PgPoolOptions::new()
            .max_connections(1)
            .connect(maintenance_connection_string)
            .await
        else {
            return;
        };

        // Ensure all lingering test connections are terminated so DROP DATABASE can succeed.
        let _ = sqlx::query(
            "SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname = $1 AND pid <> pg_backend_pid()",
        )
        .bind(db_name)
        .execute(&maintenance_pool)
        .await;

        let drop_database_query = format!(r#"DROP DATABASE IF EXISTS "{}""#, db_name);
        let _ = sqlx::query(&drop_database_query)
            .execute(&maintenance_pool)
            .await;

        maintenance_pool.close().await;
    }
}

impl Drop for TestDatabase {
    fn drop(&mut self) {
        let db_name = self.db_name.clone();
        let db_name_for_log = db_name.clone();
        let maintenance_connection_string = self.maintenance_connection_string.clone();

        let cleanup_result = thread::Builder::new()
            .name("test-db-cleanup".to_owned())
            .spawn(move || {
                let runtime = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .expect("failed to create runtime for test database cleanup");

                runtime.block_on(async move {
                    TestDatabase::drop_database(&db_name, &maintenance_connection_string).await;
                });
            })
            .and_then(|join_handle| {
                join_handle
                    .join()
                    .map_err(|_| io::Error::other("test database cleanup panicked"))
            });

        if let Err(error) = cleanup_result {
            eprintln!("warning: failed to clean up test database {db_name_for_log}: {error}");
        }
    }
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
    let database = TestDatabase::create().await;

    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("failed to bind random port");
    let port = listener
        .local_addr()
        .expect("failed to read local address")
        .port();
    let pg_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect_lazy(&database.connection_string)
        .expect("failed to build lazy postgres pool for tests");

    let server_handle = tokio::spawn(async move {
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
        server_handle,
        _database: database,
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

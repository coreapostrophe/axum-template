# Axum + SQLx Postgres Template

A production-oriented Rust API template using Axum, SQLx, PostgreSQL, structured errors, integration tests, SQLx offline query checking, and CI workflows.

## Highlights

- Axum 0.8 HTTP API with domain-oriented module structure
- PostgreSQL via SQLx with migrations
- Compile-time checked SQL queries (SQLx macros)
- SQLx offline mode enabled by default
- Per-test isolated databases for integration tests
- Devcontainer support (app + postgres)
- CI with fmt, clippy, tests, build, and SQLx metadata verification
- Security workflow with cargo-audit

## Repository Layout

- `crates/app/src/main.rs`: app entrypoint
- `crates/app/lib/`: core library modules
- `crates/app/lib/api/`: routes, handlers, domain services, response types
- `crates/app/migrations/`: SQL migration files
- `crates/app/tests/`: integration tests and test helpers
- `scripts/migrate.sh`: run migrations
- `scripts/prepare_sqlx.sh`: refresh/check SQLx offline metadata
- `.sqlx/`: SQLx query metadata cache (committed)
- `.devcontainer/`: devcontainer setup
- `.github/workflows/`: CI and security workflows

## Prerequisites

For local (non-devcontainer) development:

- Rust toolchain (workspace pinned to Rust 1.95.0)
- PostgreSQL (local or containerized)
- `sqlx-cli`

Install sqlx-cli:

```bash
cargo install sqlx-cli --no-default-features --features rustls,postgres --locked
```

## Quick Start

### Option A: Devcontainer (recommended)

1. Open this repository in VS Code.
2. Reopen in container.
3. The devcontainer will:
   - start PostgreSQL
   - install/update `sqlx-cli`
   - run migrations on container start
4. Run the app:

```bash
cargo run -p axum_app
```

Server default address:

- `http://127.0.0.1:3000`

### Option B: Local machine

Start PostgreSQL (example using the included compose file):

```bash
docker compose -f .devcontainer/docker-compose.yml up -d postgres
```

Set environment variables (example):

```bash
export APP_POSTGRES__HOST=127.0.0.1
export APP_POSTGRES__PORT=5432
export APP_POSTGRES__USER=postgres
export APP_POSTGRES__PASSWORD=postgres
export APP_POSTGRES__DB_NAME=app
export APP_POSTGRES__RUN_MIGRATIONS=true
```

Run migrations and start:

```bash
bash scripts/migrate.sh
cargo run -p axum_app
```

## Configuration

Configuration is loaded in this order:

1. In-code defaults
2. Optional config file (`APP_CONFIG_FILE`, default: `config.yaml`)
3. Environment variables prefixed with `APP_` and `__` separators

Example env keys:

- `APP_API__HOST` (default `127.0.0.1`)
- `APP_API__PORT` (default `3000`)
- `APP_POSTGRES__HOST` (default `127.0.0.1`)
- `APP_POSTGRES__PORT` (default `5432`)
- `APP_POSTGRES__USER` (default `postgres`)
- `APP_POSTGRES__PASSWORD` (default `postgres`)
- `APP_POSTGRES__DB_NAME` (default `app`)
- `APP_POSTGRES__MAX_CONNECTIONS` (default `20`)
- `APP_POSTGRES__ACQUIRE_TIMEOUT_SECONDS` (default `3`)
- `APP_POSTGRES__RUN_MIGRATIONS` (default `false`)
- `APP_API__CORS_ALLOWED_ORIGINS` (comma-separated list, defaults to local dev origins)

Test-specific key:

- `APP_POSTGRES__MAINTENANCE_DB` (default `postgres`)

## API Overview

### Health

- `GET /health` -> `200 OK`

Response:

```json
{
  "status": "healthy"
}
```

### Todos

- `GET /api/todos` -> list todos (`200`)
- `POST /api/todos` -> create todo (`201` + `Location` header)
- `GET /api/todos/{todo_id}` -> get one (`200` or `404`)
- `PATCH /api/todos/{todo_id}` -> partial update (`200`, `400`, or `404`)
- `DELETE /api/todos/{todo_id}` -> delete (`200` or `404`)

Create request example:

```json
{
  "title": "write tests"
}
```

Validation rules:

- title is trimmed and cannot be empty
- update payload must include at least one field (`title` or `completed`)

Success envelope:

```json
{
  "status": "success",
  "data": {}
}
```

Error envelope:

```json
{
  "status": "error",
  "code": "bad_request",
  "message": "..."
}
```

## OpenAPI Specification

Generate an OpenAPI JSON file from code annotations:

```bash
cargo run -p axum_app --features openapi --bin openapi -- openapi/openapi.json
```

If no output path is provided, the generator writes to `openapi.json` in the current working directory.

## Database and Migrations

Migration files live in:

- `crates/app/migrations/`

Run migrations:

```bash
bash scripts/migrate.sh
```

Notes:

- Do not edit already applied migration files in shared environments.
- Add a new migration file for schema changes.

## SQLx Offline Mode

This template enables SQLx offline mode by default via:

- `.cargo/config.toml` (`SQLX_OFFLINE=true`)

That means query macros compile against checked metadata in `.sqlx/`.

### Refresh metadata after query/schema changes

```bash
bash scripts/prepare_sqlx.sh
```

### Validate metadata (used in CI)

```bash
bash scripts/prepare_sqlx.sh --check
```

When you change queries or migrations, commit updated files in `.sqlx/`.

## Testing

Run tests:

```bash
cargo test --all-targets --all-features
```

Integration tests use isolated databases per test case:

- each test creates a unique test DB
- migrations run in that DB
- DB is dropped on teardown

This requires a DB user with permission to create/drop databases and access to a maintenance DB (usually `postgres`).

## Useful Commands

```bash
cargo fmt
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
cargo build --all-targets --all-features
cargo build --release --all-targets --all-features
bash scripts/migrate.sh
bash scripts/prepare_sqlx.sh --check
```

## CI and Security Workflows

CI (`.github/workflows/ci.yml`) runs:

- formatting check
- clippy (warnings denied)
- tests
- build
- SQLx metadata check on stable lane

Security (`.github/workflows/security.yml`) runs:

- `cargo audit`
- with a temporary ignore for `RUSTSEC-2023-0071` (transitive via SQLx macro toolchain)

## Troubleshooting

### SQLx compile error: metadata missing/stale

If you see messages about setting `DATABASE_URL` or updating query cache:

```bash
bash scripts/prepare_sqlx.sh
```

Then commit updated `.sqlx` files.

### Migration checksum/state issues

If local migration state is inconsistent, reset local DB and re-run:

```bash
sqlx database drop -y
sqlx database create
bash scripts/prepare_sqlx.sh
```

### Tests cannot create isolated DB

Check DB privileges and env values:

- `APP_POSTGRES__USER`
- `APP_POSTGRES__PASSWORD`
- `APP_POSTGRES__HOST`
- `APP_POSTGRES__PORT`
- `APP_POSTGRES__MAINTENANCE_DB`

### CORS not behaving as expected

Verify `APP_API__CORS_ALLOWED_ORIGINS` is a comma-separated list of valid origins.

#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

check_mode="${1:-}"

if ! command -v sqlx >/dev/null 2>&1; then
    echo "sqlx-cli is required. Install with: cargo install sqlx-cli --no-default-features --features rustls,postgres --locked" >&2
    exit 1
fi

if [[ -n "${check_mode}" && "${check_mode}" != "--check" ]]; then
    echo "unknown argument: ${check_mode}" >&2
    echo "usage: bash scripts/prepare_sqlx.sh [--check]" >&2
    exit 1
fi

if [[ -z "${DATABASE_URL:-}" ]]; then
    host="${APP_POSTGRES__HOST:-127.0.0.1}"
    port="${APP_POSTGRES__PORT:-5432}"
    user="${APP_POSTGRES__USER:-postgres}"
    password="${APP_POSTGRES__PASSWORD:-postgres}"
    db_name="${APP_POSTGRES__DB_NAME:-app}"

    export DATABASE_URL="postgres://${user}:${password}@${host}:${port}/${db_name}"
fi

bash scripts/migrate.sh

if [[ "${check_mode}" == "--check" ]]; then
    SQLX_OFFLINE=false cargo sqlx prepare --workspace --check -- --all-targets --all-features
else
    SQLX_OFFLINE=false cargo sqlx prepare --workspace -- --all-targets --all-features
fi

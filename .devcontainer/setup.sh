#!/usr/bin/env bash
set -euo pipefail

echo "Installing dependencies..."

SQLX_CLI_VERSION="${SQLX_CLI_VERSION:-0.8.6}"

if command -v sqlx >/dev/null 2>&1; then
	installed_version="$(sqlx --version | awk '{print $2}')"

	if [[ "${installed_version}" == "${SQLX_CLI_VERSION}" ]]; then
		echo "sqlx-cli already installed: $(sqlx --version)"
	else
		echo "Updating sqlx-cli from ${installed_version} to ${SQLX_CLI_VERSION}..."
		cargo install sqlx-cli --version "${SQLX_CLI_VERSION}" --force --no-default-features --features rustls,postgres --locked
	fi
else
	echo "Installing sqlx-cli ${SQLX_CLI_VERSION} (postgres + rustls)..."
	cargo install sqlx-cli --version "${SQLX_CLI_VERSION}" --no-default-features --features rustls,postgres --locked
fi

echo "Setup complete!"


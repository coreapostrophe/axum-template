#!/bin/bash
set -euo pipefail

echo "Installing dependencies..."

if command -v sqlx >/dev/null 2>&1; then
	echo "sqlx-cli already installed: $(sqlx --version)"
else
	echo "Installing sqlx-cli (postgres + rustls)..."
	cargo install sqlx-cli --no-default-features --features rustls,postgres --locked
fi

echo "Setup complete!"


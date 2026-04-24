FROM mcr.microsoft.com/devcontainers/rust:bullseye

RUN rustup update stable && \
    rustup default stable && \
    rustup component add rustfmt clippy rust-src

USER vscode

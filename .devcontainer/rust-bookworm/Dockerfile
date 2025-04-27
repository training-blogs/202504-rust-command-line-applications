FROM mcr.microsoft.com/devcontainers/rust:1-1-bookworm

RUN apt update && apt install -y mingw-w64

RUN rustup target add x86_64-unknown-linux-gnu \
    && rustup target add x86_64-pc-windows-gnu

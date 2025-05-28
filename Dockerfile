# syntax=docker/dockerfile:1

# Build stage: compile the Rust binary statically
FROM rust:1.77-alpine AS builder
WORKDIR /app

# Install required build tools
RUN apk add --no-cache musl-dev build-base openssl-dev protobuf-dev

# Copy source
COPY Cargo.toml Cargo.lock* ./
COPY build.rs ./
COPY src ./src
COPY proto ./proto

# Build for musl to enable a static binary
RUN rustup target add x86_64-unknown-linux-musl \
    && cargo build --release --target x86_64-unknown-linux-musl

# Final stage: minimal scratch image
FROM scratch AS runtime
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/codex /codex

ENTRYPOINT ["/codex"]


###################
# Build stage
###################
FROM rust:1-slim AS builder

WORKDIR /usr/src/api

COPY Cargo.toml Cargo.lock ./

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/src/api/target \
    mkdir src && \
    echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs && \
    cargo build --release && \
    rm -r src && \
    rm target/release/deps/api*

COPY migrations migrations
COPY src ./src

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/src/api/target \
    cargo build --release && \
    cp target/release/api ./api

###################
# Prod stage
###################
FROM debian:bookworm-slim

WORKDIR /app

RUN apt update && apt install -y glibc-source && rm -rf /var/lib/apt/lists/*

RUN useradd -m api

COPY --from=builder /usr/src/api/api .

RUN chown -R api:api /app

USER api

CMD ["./api"]

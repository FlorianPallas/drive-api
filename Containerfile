FROM docker.io/lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

# ---
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# ---
FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
ENV SQLX_OFFLINE=true
RUN cargo build --release --bin drive

# ---
FROM docker.io/debian:trixie-slim AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/drive /usr/local/bin
ENTRYPOINT ["/usr/local/bin/drive"]

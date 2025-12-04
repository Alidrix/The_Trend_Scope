FROM rust:1.79 AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock* ./
COPY src ./src
# Use the lockfile to avoid pulling in newer unstable dependencies during builds
RUN cargo build --release --locked

FROM debian:bookworm-slim
WORKDIR /app
RUN apt-get update && apt-get install -y libpq5 ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/youtube_tiktok_backend /app/backend
ENV RUST_LOG=info
EXPOSE 4443
CMD ["/app/backend"]

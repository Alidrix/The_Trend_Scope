# Multi-stage build for backend service
FROM rust:1.79 AS builder
WORKDIR /app
COPY backend/Cargo.toml backend/Cargo.lock* ./
COPY backend/src ./src
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
RUN apt-get update && apt-get install -y libpq5 ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/youtube_tiktok_backend /app/backend
EXPOSE 4443
CMD ["/app/backend"]

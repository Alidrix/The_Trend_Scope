FROM rust:1.79 AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock* ./
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
RUN apt-get update && apt-get install -y libpq5 ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/youtube_tiktok_backend /app/backend
ENV RUST_LOG=info
ENV APP_USERNAME=admin
ENV APP_PASSWORD=changemechangeme!!
ENV APP_SECRET=prod-secret
ENV YOUTUBE_API_KEY=demo
ENV DATABASE_URL=postgres://postgres:postgres@db:5432/viral
CMD ["/app/backend"]
EXPOSE 4443

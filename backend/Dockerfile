FROM rust:1.83-bookworm AS builder
WORKDIR /app

COPY backend/Cargo.toml backend/Cargo.lock* ./backend/
COPY backend/src ./backend/src
COPY db ./db

WORKDIR /app/backend
RUN cargo build --release --locked --bin youtube_tiktok_backend --bin worker

FROM debian:bookworm-slim
WORKDIR /app

RUN apt-get update \
  && apt-get install -y libpq5 ca-certificates curl \
  && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/backend/target/release/youtube_tiktok_backend /app/api
COPY --from=builder /app/backend/target/release/worker /app/worker

ENV RUST_LOG=info
EXPOSE 4443

CMD ["/app/api"]

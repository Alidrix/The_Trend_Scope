use dotenvy::dotenv;
use std::net::SocketAddr;
use tracing::info;
use youtube_tiktok_backend::{app, error::AppError};

#[tokio::main]
async fn main() -> Result<(), AppError> {
    dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let state = app::build_state().await?;
    let router = app::build_router(state)?;

    let addr: SocketAddr = "0.0.0.0:4443".parse().map_err(|_| AppError::Internal)?;
    info!("API listening on {addr}");

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .map_err(|_| AppError::Internal)?;
    axum::serve(listener, router)
        .await
        .map_err(|_| AppError::Internal)
}

use dotenvy::dotenv;
use tokio::time::{sleep, Duration};
use tracing::{error, info};
use youtube_tiktok_backend::{
    app,
    error::AppError,
    services::{analytics, queue},
};

#[tokio::main]
async fn main() -> Result<(), AppError> {
    dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let state = app::build_state().await?;
    let interval = state.config.scan.interval_minutes.max(1);

    info!("Worker started (scan interval: {interval} minutes)");

    loop {
        if let Err(err) = queue::publish_scan_tick(&state.nats).await {
            error!("nats publish failed: {err}");
        }

        if let Err(err) = analytics::ensure_schema(&state.clickhouse).await {
            error!("clickhouse schema check failed: {err}");
        }

        sleep(Duration::from_secs(interval * 60)).await;
    }
}

#![allow(dead_code)]
#[path = "../config.rs"]
mod config;
#[path = "../error.rs"]
mod error;
#[path = "../worker/mod.rs"]
mod worker;

use dotenvy::dotenv;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), error::AppError> {
    dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let app = config::AppConfig::from_env()?;
    let interval = std::env::var("SCAN_INTERVAL_MINUTES")
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(30);

    info!("worker started");
    worker::scheduler::run_scheduler(app.youtube.regions, app.youtube.themes, interval).await;
    Ok(())
}

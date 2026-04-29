use std::time::Duration;

use crate::{config::AppConfig, error::AppError};
use clickhouse::Client as ClickHouseClient;
use reqwest::Client;
use sqlx::{postgres::PgPoolOptions, PgPool};

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub pool: PgPool,
    pub redis: redis::Client,
    pub nats: async_nats::Client,
    pub clickhouse: ClickHouseClient,
    pub http: Client,
}

async fn connect_postgres_with_retry(database_url: &str) -> Result<PgPool, AppError> {
    let mut last_error = None;

    for attempt in 1..=30 {
        match PgPoolOptions::new()
            .max_connections(10)
            .connect(database_url)
            .await
        {
            Ok(pool) => return Ok(pool),
            Err(err) => {
                tracing::warn!(attempt, error = %err, "postgres connection attempt failed");
                last_error = Some(err);
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        }
    }

    match last_error {
        Some(err) => Err(AppError::Database(err)),
        None => Err(AppError::Internal),
    }
}

async fn connect_nats_with_retry(nats_url: &str) -> Result<async_nats::Client, AppError> {
    let mut last_error = None;

    for attempt in 1..=30 {
        match async_nats::connect(nats_url.to_string()).await {
            Ok(client) => return Ok(client),
            Err(err) => {
                tracing::warn!(attempt, error = %err, "nats connection attempt failed");
                last_error = Some(err);
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        }
    }

    Err(AppError::Config(format!(
        "cannot connect NATS after retries: {}",
        last_error
            .map(|err| err.to_string())
            .unwrap_or_else(|| "unknown error".to_string())
    )))
}

impl AppState {
    pub async fn from_config(config: AppConfig) -> Result<Self, AppError> {
        let pool = connect_postgres_with_retry(&config.database.database_url).await?;

        let redis = redis::Client::open(config.redis.redis_url.clone())
            .map_err(|err| AppError::Config(format!("invalid REDIS_URL: {err}")))?;

        let nats = connect_nats_with_retry(&config.nats.nats_url).await?;

        let clickhouse = ClickHouseClient::default()
            .with_url(config.clickhouse.url.clone())
            .with_database(config.clickhouse.database.clone())
            .with_user(config.clickhouse.user.clone())
            .with_password(config.clickhouse.password.clone());

        Ok(Self {
            config,
            pool,
            redis,
            nats,
            clickhouse,
            http: Client::new(),
        })
    }
}

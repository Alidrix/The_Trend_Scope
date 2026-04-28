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

impl AppState {
    pub async fn from_config(config: AppConfig) -> Result<Self, AppError> {
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(&config.database.database_url)
            .await?;

        let redis = redis::Client::open(config.redis.redis_url.clone())
            .map_err(|err| AppError::Config(format!("invalid REDIS_URL: {err}")))?;

        let nats = async_nats::connect(config.nats.nats_url.clone())
            .await
            .map_err(|err| AppError::Config(format!("cannot connect NATS: {err}")))?;

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

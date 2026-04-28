use crate::{
    config::{AppConfig, AuthConfig, YoutubeConfig},
    services::{analytics::AnalyticsService, cache::CacheService, queue::QueueService},
};
use reqwest::Client;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub auth: AuthConfig,
    pub youtube: YoutubeConfig,
    pub http: Client,
    pub cache: CacheService,
    pub queue: QueueService,
    pub analytics: AnalyticsService,
    pub config: AppConfig,
}

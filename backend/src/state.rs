use crate::config::{AuthConfig, YoutubeConfig};
use reqwest::Client;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub auth: AuthConfig,
    pub youtube: YoutubeConfig,
    pub http: Client,
}

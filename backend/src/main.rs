#![allow(dead_code)]
mod config;
mod error;
mod repositories {
    pub mod users;
    pub mod videos;
}
mod routes {
    pub mod auth;
    pub mod billing;
    pub mod health;
    pub mod notes;
    pub mod plans;
    pub mod radar;
    pub mod videos;
}
mod models {
    pub mod plan;
    pub mod video;
}
mod services {
    pub mod access;
    pub mod analytics;
    pub mod cache;
    pub mod queue;
    pub mod rate_limit;
    pub mod scoring;
    pub mod youtube;
}
mod state;

use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts, State},
    http::{header, request::Parts, HeaderValue, Method},
    routing::{get, post},
    Router,
};
use config::{AppConfig, AuthConfig};
use dotenvy::dotenv;
use error::AppError;
use jsonwebtoken::{decode, DecodingKey, Validation};
use routes::{
    auth::{auth_status, login, register, Claims},
    billing::billing_status,
    health::health,
    notes::update_note,
    plans::list_plans,
    radar::daily_radar,
    videos::{list_videos, refresh_videos, scan_videos},
};
use sqlx::{postgres::PgPoolOptions, PgPool};
use state::AppState;
use std::net::SocketAddr;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::{info, warn};

#[tokio::main]
async fn main() -> Result<(), AppError> {
    dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let config = AppConfig::from_env()?;

    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(&config.database.database_url)
        .await?;

    apply_bootstrap_migration(&pool).await?;
    repositories::users::ensure_seed_user(&pool, &config.auth).await?;

    let cache = services::cache::CacheService::new(&config.redis.redis_url)?;
    let queue = services::queue::QueueService::connect(&config.nats.nats_url).await?;
    let analytics = services::analytics::AnalyticsService::new(
        &config.clickhouse.url,
        &config.clickhouse.database,
        &config.clickhouse.user,
        &config.clickhouse.password,
    );

    let state = AppState {
        pool,
        auth: config.auth.clone(),
        youtube: config.youtube.clone(),
        http: reqwest::Client::new(),
        cache,
        queue,
        analytics,
        config: config.clone(),
    };

    let cors = if let Ok(origin) = HeaderValue::from_str(&config.frontend_origin) {
        CorsLayer::new()
            .allow_origin(origin)
            .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
            .allow_methods([Method::GET, Method::POST, Method::PATCH])
    } else {
        warn!("invalid FRONTEND_ORIGIN, falling back to strict localhost CORS");
        CorsLayer::new()
            .allow_origin(HeaderValue::from_static("http://localhost:5173"))
            .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
            .allow_methods([Method::GET, Method::POST, Method::PATCH])
    };

    let app = Router::new()
        .route("/api/v1/health", get(health))
        .route("/api/v1/ready", get(health))
        .route("/api/v1/auth/login", post(login))
        .route("/api/v1/auth/status", get(auth_status))
        .route("/api/v1/auth/register", post(register))
        .route("/api/v1/plans", get(list_plans))
        .route("/api/v1/billing/status", get(billing_status))
        .route(
            "/api/v1/radar/daily",
            get(|auth: AuthBearer, state: State<AppState>| async move {
                daily_radar(auth, state).await
            }),
        )
        .route(
            "/api/v1/videos",
            get(|auth: AuthBearer, state: State<AppState>| async move {
                list_videos(auth, state).await
            })
            .post(
                |auth: AuthBearer, state: State<AppState>, payload| async move {
                    refresh_videos(auth, state, payload).await
                },
            ),
        )
        .route(
            "/api/v1/videos/scan",
            post(|auth: AuthBearer, state: State<AppState>| async move {
                scan_videos(auth, state).await
            }),
        )
        .route(
            "/api/v1/notes",
            post(
                |_auth: AuthBearer, state: State<AppState>, payload| async move {
                    update_note(state, payload).await
                },
            ),
        )
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let addr: SocketAddr = "0.0.0.0:4443".parse().unwrap();
    info!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .map_err(|_| AppError::Internal)?;
    axum::serve(listener, app)
        .await
        .map_err(|_| AppError::Internal)
}

async fn apply_bootstrap_migration(pool: &PgPool) -> Result<(), AppError> {
    const INIT_SQL: &str = include_str!("../../db/migrations/init.sql");

    sqlx::raw_sql(INIT_SQL).execute(pool).await?;

    Ok(())
}

impl FromRef<AppState> for AuthConfig {
    fn from_ref(app: &AppState) -> AuthConfig {
        app.auth.clone()
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthBearer
where
    AuthConfig: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get(header::AUTHORIZATION)
            .cloned()
            .ok_or(AppError::Unauthorized)?;
        let value = auth_header
            .to_str()
            .map_err(|_| AppError::Unauthorized)?
            .to_string();

        if !value.to_ascii_lowercase().starts_with("bearer ") {
            return Err(AppError::Unauthorized);
        }

        let token = value[7..].trim();
        let config = AuthConfig::from_ref(state);
        let decoded = decode::<Claims>(
            token,
            &DecodingKey::from_secret(config.jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| AppError::Unauthorized)?;
        Ok(AuthBearer {
            sub: decoded.claims.sub,
        })
    }
}

pub struct AuthBearer {
    pub sub: String,
}

#[cfg(test)]
mod tests {
    use crate::config::normalize_database_url;

    #[test]
    fn normalize_database_url_adds_sslmode() {
        std::env::set_var("DATABASE_URL", "postgres://localhost:5432/db");
        let normalized = normalize_database_url().expect("normalize");
        assert!(normalized.contains("sslmode=disable"));
    }
}

mod config;
mod error;
mod repositories {
    pub mod users;
    pub mod videos;
}
mod routes {
    pub mod auth;
    pub mod health;
    pub mod notes;
    pub mod videos;
}
mod models {
    pub mod video;
}
mod services {
    pub mod youtube;
}
mod state;

use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts, State},
    http::{self, header, request::Parts},
    routing::{get, post},
    Router,
};
use config::{normalize_database_url, AuthConfig, YoutubeConfig};
use dotenvy::dotenv;
use error::AppError;
use jsonwebtoken::{decode, DecodingKey, Validation};
use routes::{
    auth::{auth_status, login, register, Claims},
    health::health,
    notes::update_note,
    videos::{list_videos, refresh_videos, scan_videos},
};
use sqlx::{postgres::PgPoolOptions, PgPool};
use state::AppState;
use std::net::SocketAddr;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let database_url = normalize_database_url()?;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    apply_bootstrap_migration(&pool).await?;

    let auth = AuthConfig::from_env()?;
    repositories::users::ensure_seed_user(&pool, &auth).await?;
    let youtube = YoutubeConfig::from_env();
    let state = AppState {
        pool,
        auth,
        youtube,
        http: reqwest::Client::new(),
    };

    let app = Router::new()
        .route("/api/v1/health", get(health))
        .route("/api/v1/auth/login", post(login))
        .route("/api/v1/auth/status", get(auth_status))
        .route("/api/v1/auth/register", post(register))
        .route(
            "/api/v1/videos",
            get(|auth: AuthBearer, state: State<AppState>| async move {
                let _ = auth;
                list_videos(state).await
            })
            .post(
                |auth: AuthBearer, state: State<AppState>, payload| async move {
                    let _ = auth;
                    refresh_videos(state, payload).await
                },
            ),
        )
        .route(
            "/api/v1/videos/scan",
            post(|auth: AuthBearer, state: State<AppState>| async move {
                let _ = auth;
                scan_videos(state).await
            }),
        )
        .route(
            "/api/v1/notes",
            post(
                |auth: AuthBearer, state: State<AppState>, payload| async move {
                    let _ = auth;
                    update_note(state, payload).await
                },
            ),
        )
        .layer(
            CorsLayer::very_permissive()
                .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
                .allow_methods([http::Method::GET, http::Method::POST]),
        )
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

    for statement in INIT_SQL
        .split(';')
        .map(str::trim)
        .filter(|sql| !sql.is_empty())
    {
        sqlx::query(statement).execute(pool).await?;
    }

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
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(config.jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| AppError::Unauthorized)?;
        Ok(AuthBearer)
    }
}

struct AuthBearer;

#[cfg(test)]
mod tests {
    use crate::config::normalize_database_url;

    #[test]
    fn normalize_database_url_adds_sslmode() {
        std::env::set_var("DATABASE_URL", "postgres://localhost:5432/db");
        let normalized = normalize_database_url().expect("normalize");
        assert!(normalized.contains("sslmode=require"));
    }
}

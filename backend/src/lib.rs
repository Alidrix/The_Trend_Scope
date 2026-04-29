pub mod app;
pub mod config;
pub mod error;
pub mod models {
    pub mod plan;
    pub mod source;
    pub mod subscription;
    pub mod user;
    pub mod video;
}
pub mod repositories {
    pub mod admin;
    pub mod alerts;
    pub mod audit_logs;
    pub mod consents;
    pub mod email_logs;
    pub mod email_verification;
    pub mod favorites;
    pub mod notifications;
    pub mod password_reset;
    pub mod reports;
    pub mod stripe_events;
    pub mod subscriptions;
    pub mod usage;
    pub mod users;
    pub mod videos;
    pub mod watchlists;
}
pub mod routes {
    pub mod admin;
    pub mod alerts;
    pub mod auth;
    pub mod billing;
    pub mod consents;
    pub mod exports;
    pub mod favorites;
    pub mod health;
    pub mod me;
    pub mod metrics;
    pub mod notes;
    pub mod notifications;
    pub mod plans;
    pub mod radar;
    pub mod reports;
    pub mod videos;
    pub mod watchlists;
}
pub mod services {
    pub mod access;
    pub mod alerts;
    pub mod analytics;
    pub mod cache;
    pub mod email;
    pub mod privacy;
    pub mod queue;
    pub mod rate_limit;
    pub mod reports;
    pub mod scoring;
    pub mod storage;
    pub mod stripe;
    pub mod telegram;
    pub mod youtube;
}
pub mod sources;
pub mod state;

use async_trait::async_trait;
use axum::{
    extract::{FromRef, FromRequestParts},
    http::{header, request::Parts},
};
use jsonwebtoken::{decode, DecodingKey, Validation};

use crate::{config::AuthConfig, error::AppError, routes::auth::Claims};

#[derive(Clone, Debug)]
pub struct AuthBearer {
    pub sub: String,
}

impl FromRef<state::AppState> for AuthConfig {
    fn from_ref(app: &state::AppState) -> AuthConfig {
        app.config.auth.clone()
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

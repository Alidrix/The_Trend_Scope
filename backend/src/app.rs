use axum::{
    http::{self, header, HeaderValue},
    routing::{get, post},
    Router,
};
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use crate::{
    config::AppConfig,
    error::AppError,
    repositories::users::ensure_seed_user,
    routes::{
        auth::{auth_status, login, register},
        billing::{billing_checkout, billing_portal, billing_status, billing_webhook},
        consents::{get_consents, post_consent},
        health::health,
        me::{data_export_request, delete_request, get_me, patch_me},
        notes::update_note,
        plans::list_plans,
        radar::daily_radar,
        videos::{list_videos, refresh_videos, scan_videos},
    },
    state::AppState,
    AuthBearer,
};

pub async fn build_state() -> Result<AppState, AppError> {
    let config = AppConfig::from_env()?;
    let state = AppState::from_config(config.clone()).await?;

    apply_bootstrap_migration(&state.pool).await?;
    ensure_seed_user(&state.pool, &state.config.auth).await?;

    Ok(state)
}

pub fn build_router(state: AppState) -> Result<Router, AppError> {
    let frontend_origin: HeaderValue = state
        .config
        .frontend_origin
        .parse()
        .map_err(|_| AppError::Config("FRONTEND_ORIGIN is invalid".into()))?;

    Ok(
        Router::new()
            .route("/api/v1/health", get(health))
            .route("/api/v1/auth/login", post(login))
            .route("/api/v1/auth/status", get(auth_status))
            .route("/api/v1/auth/register", post(register))
            .route("/api/v1/plans", get(list_plans))
            .route("/api/v1/billing/status", get(billing_status))
            .route("/api/v1/billing/checkout", post(billing_checkout))
            .route("/api/v1/billing/portal", post(billing_portal))
            .route("/api/v1/billing/webhook", post(billing_webhook))
            .route(
                "/api/v1/radar/daily",
                get(|auth: AuthBearer, state| async move { daily_radar(auth, state).await }),
            )
            .route(
                "/api/v1/videos",
                get(|auth: AuthBearer, state| async move { list_videos(auth, state).await }).post(
                    |auth: AuthBearer, state, payload| async move {
                        refresh_videos(auth, state, payload).await
                    },
                ),
            )
            .route(
                "/api/v1/videos/scan",
                post(|auth: AuthBearer, state| async move { scan_videos(auth, state).await }),
            )
            .route(
                "/api/v1/notes",
                post(|_auth: AuthBearer, state, payload| async move {
                    update_note(state, payload).await
                }),
            )
            .route("/api/v1/me", get(get_me).patch(patch_me))
            .route("/api/v1/me/consents", get(get_consents).post(post_consent))
            .route("/api/v1/me/data-export", post(data_export_request))
            .route("/api/v1/me/delete-request", post(delete_request))
            .layer(
                CorsLayer::new()
                    .allow_origin(frontend_origin)
                    .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
                    .allow_methods([
                        http::Method::GET,
                        http::Method::POST,
                        http::Method::PATCH,
                        http::Method::DELETE,
                    ]),
            )
            .layer(TraceLayer::new_for_http())
            .with_state(state),
    )
}

pub async fn apply_bootstrap_migration(pool: &sqlx::PgPool) -> Result<(), AppError> {
    const INIT_SQL: &str = include_str!("../../db/migrations/init.sql");
    sqlx::raw_sql(INIT_SQL).execute(pool).await?;
    Ok(())
}

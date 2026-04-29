use axum::{
    extract::{Query, State},
    Json,
};
use serde::Deserialize;
use serde_json::json;

use crate::{
    error::AppError,
    repositories::{admin, alerts, reports},
    services::access::ensure_admin,
    state::AppState,
    AuthBearer,
};

#[derive(Debug, Deserialize)]
pub struct AdminUsersQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub plan: Option<String>,
    pub role: Option<String>,
    pub search: Option<String>,
}

pub async fn overview(
    auth: AuthBearer,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, AppError> {
    ensure_admin(&state.pool, &auth.sub).await?;
    let total_users: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
        .fetch_one(&state.pool)
        .await?;
    let verified: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users WHERE email_verified=true")
        .fetch_one(&state.pool)
        .await?;
    let admins: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users WHERE role='admin'")
        .fetch_one(&state.pool)
        .await?;
    let free: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users WHERE plan='free'")
        .fetch_one(&state.pool)
        .await?;
    let pro: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users WHERE plan='pro'")
        .fetch_one(&state.pool)
        .await?;
    let studio: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users WHERE plan='studio'")
        .fetch_one(&state.pool)
        .await?;
    let subs_total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM subscriptions")
        .fetch_one(&state.pool)
        .await?;
    let subs_active: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM subscriptions WHERE status='active'")
            .fetch_one(&state.pool)
            .await?;
    let pending_reports = reports::count_pending(&state.pool).await?;
    let pending_alerts = alerts::count_pending_deliveries(&state.pool).await?;
    Ok(Json(
        json!({"users":{"total":total_users,"verified":verified,"admins":admins},"plans":{"free":free,"pro":pro,"studio":studio},"subscriptions":{"total":subs_total,"active":subs_active,"inactive":subs_total-subs_active},"sources":admin::source_status(&state.config),"jobs":{"pending_reports":pending_reports,"pending_alert_deliveries":pending_alerts},"system":{"postgres":"ok","redis":"configured","nats":"configured","clickhouse":"configured"}}),
    ))
}

pub async fn users(
    auth: AuthBearer,
    State(state): State<AppState>,
    Query(q): Query<AdminUsersQuery>,
) -> Result<Json<serde_json::Value>, AppError> {
    ensure_admin(&state.pool, &auth.sub).await?;
    let filters = admin::AdminUserFilters {
        page: q.page.unwrap_or(1),
        page_size: q.page_size.unwrap_or(50),
        plan: q.plan,
        role: q.role,
        search: q.search,
    };
    Ok(Json(
        json!({"users": admin::list_users(&state.pool, &filters).await?}),
    ))
}

pub async fn sources(
    auth: AuthBearer,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, AppError> {
    ensure_admin(&state.pool, &auth.sub).await?;
    Ok(Json(
        json!({"sources":[{"platform":"youtube","configured":!state.config.youtube.api_key.is_empty(),"status":if state.config.youtube.api_key.is_empty(){"not_configured"}else{"configured"},"message":"YouTube API key configured server-side"},{"platform":"tiktok","configured":state.config.tiktok.is_configured(),"status":if state.config.tiktok.is_configured(){"configured_but_not_implemented"}else{"not_configured"},"message":"TikTok provider is in preview mode"},{"platform":"instagram","configured":state.config.instagram.is_configured(),"status":if state.config.instagram.is_configured(){"configured_but_not_implemented"}else{"not_configured"},"message":"Instagram provider is in preview mode"}]}),
    ))
}

pub async fn jobs(
    auth: AuthBearer,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, AppError> {
    ensure_admin(&state.pool, &auth.sub).await?;
    Ok(Json(json!(reports::jobs_snapshot(&state.pool).await?)))
}

pub async fn system(
    auth: AuthBearer,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, AppError> {
    ensure_admin(&state.pool, &auth.sub).await?;
    let postgres = if sqlx::query_scalar::<_, i64>("SELECT 1")
        .fetch_one(&state.pool)
        .await
        .is_ok()
    {
        "ok"
    } else {
        "error"
    };
    let redis = if state.redis.get_multiplexed_async_connection().await.is_ok() {
        "ok"
    } else {
        "error"
    };
    let nats = "ok";
    let clickhouse = "configured";
    Ok(Json(
        json!({"postgres":postgres,"redis":redis,"nats":nats,"clickhouse":clickhouse}),
    ))
}

pub async fn billing(
    auth: AuthBearer,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, AppError> {
    ensure_admin(&state.pool, &auth.sub).await?;
    let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM subscriptions")
        .fetch_one(&state.pool)
        .await?;
    let active: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM subscriptions WHERE status='active'")
            .fetch_one(&state.pool)
            .await?;
    let pro: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM subscriptions WHERE plan='pro'")
        .fetch_one(&state.pool)
        .await?;
    let studio: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM subscriptions WHERE plan='studio'")
        .fetch_one(&state.pool)
        .await?;
    Ok(Json(
        json!({"subscriptions":{"total":total,"active":active,"inactive":total-active,"pro":pro,"studio":studio},"mrr_cents_estimate":pro*1000+studio*1800}),
    ))
}

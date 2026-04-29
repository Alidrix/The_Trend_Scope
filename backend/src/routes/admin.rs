use axum::{
    extract::{Query, State},
    Json,
};
use serde::Deserialize;
use serde_json::json;

use crate::{
    error::AppError,
    repositories::{admin, alerts, email_logs, notifications, reports},
    services::{
        access::ensure_admin,
        email,
        telegram::{send_telegram_alert, TelegramAlertMessage},
    },
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
#[derive(Debug, Deserialize)]
pub struct TestTelegramPayload {
    pub chat_id: Option<String>,
}
#[derive(Debug, Deserialize)]
pub struct TestSmtpPayload {
    pub to: String,
}
// keep existing funcs abbreviated
pub async fn overview(
    auth: AuthBearer,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, AppError> {
    ensure_admin(&state.pool, &auth.sub).await?;
    let total_users: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
        .fetch_one(&state.pool)
        .await?;
    Ok(Json(json!({"users":{"total":total_users}})))
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
        json!({"users": admin::list_users(&state.pool,&filters).await?}),
    ))
}
pub async fn sources(
    auth: AuthBearer,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, AppError> {
    ensure_admin(&state.pool, &auth.sub).await?;
    Ok(Json(json!({"sources":admin::source_status(&state.config)})))
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
    Ok(Json(
        json!({"smtp_configured":state.config.smtp.is_configured(),"telegram_configured":state.config.telegram.is_configured()}),
    ))
}
pub async fn billing(
    auth: AuthBearer,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, AppError> {
    ensure_admin(&state.pool, &auth.sub).await?;
    Ok(Json(json!({"ok":true})))
}

pub async fn email_logs_list(
    auth: AuthBearer,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, AppError> {
    ensure_admin(&state.pool, &auth.sub).await?;
    Ok(Json(
        json!({"logs": email_logs::latest(&state.pool, 50).await?}),
    ))
}
pub async fn notifications_snapshot(
    auth: AuthBearer,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, AppError> {
    ensure_admin(&state.pool, &auth.sub).await?;
    Ok(Json(notifications::admin_snapshot(&state.pool).await?))
}
pub async fn exports_list(
    auth: AuthBearer,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, AppError> {
    ensure_admin(&state.pool, &auth.sub).await?;
    let exports = reports::latest_exports(&state.pool).await?.into_iter().map(|r| json!({"id":r.id,"title":r.title,"format":r.format,"file_url":r.file_url,"created_at":r.created_at})).collect::<Vec<_>>();
    Ok(Json(json!({"exports":exports})))
}
pub async fn test_telegram(
    auth: AuthBearer,
    State(state): State<AppState>,
    Json(payload): Json<TestTelegramPayload>,
) -> Result<Json<serde_json::Value>, AppError> {
    ensure_admin(&state.pool, &auth.sub).await?;
    if !state.config.telegram.is_configured() {
        return Ok(Json(json!({"sent":false,"reason":"not_configured"})));
    }
    let chat_id = payload.chat_id.or_else(|| {
        state
            .config
            .telegram
            .fallback_chat_id()
            .map(|x| x.to_string())
    });
    let Some(chat_id) = chat_id else {
        return Ok(Json(json!({"sent":false,"reason":"chat_id_missing"})));
    };
    send_telegram_alert(
        &state.config.telegram,
        TelegramAlertMessage {
            chat_id,
            title: "Test admin ops".into(),
            platform: "youtube".into(),
            region: None,
            category: None,
            views_per_hour: Some(1234),
            trend_score: Some(1.2),
            url: None,
        },
    )
    .await?;
    Ok(Json(json!({"sent":true})))
}
pub async fn test_smtp(
    auth: AuthBearer,
    State(state): State<AppState>,
    Json(payload): Json<TestSmtpPayload>,
) -> Result<Json<serde_json::Value>, AppError> {
    ensure_admin(&state.pool, &auth.sub).await?;
    if !state.config.smtp.is_configured() {
        return Ok(Json(json!({"sent":false,"reason":"not_configured"})));
    }
    email::send_email(
        &state.pool,
        &state.config.smtp,
        None,
        &payload.to,
        "Trend Scope SMTP test",
        "<p>SMTP test admin</p>",
    )
    .await?;
    Ok(Json(json!({"sent":true})))
}

use serde::Serialize;
use sqlx::PgPool;

use crate::error::AppError;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Report {
    pub id: uuid::Uuid,
    pub title: String,
    pub period_start: chrono::NaiveDate,
    pub period_end: chrono::NaiveDate,
    pub status: String,
    pub file_url: Option<String>,
    pub summary: serde_json::Value,
    pub platforms: Vec<String>,
    pub categories: Vec<String>,
    pub format: String,
    pub error_message: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone)]
pub struct NewReport {
    pub user_id: uuid::Uuid,
    pub title: String,
    pub period_start: chrono::NaiveDate,
    pub period_end: chrono::NaiveDate,
    pub platforms: Vec<String>,
    pub categories: Vec<String>,
    pub format: String,
}

pub async fn list(pool: &PgPool, user_id: uuid::Uuid) -> Result<Vec<Report>, AppError> {
    sqlx::query_as::<_, Report>("SELECT id, title, period_start, period_end, status, file_url, summary, platforms, categories, format, error_message, created_at, completed_at FROM reports WHERE user_id = $1 ORDER BY created_at DESC")
        .bind(user_id)
        .fetch_all(pool)
        .await
        .map_err(AppError::from)
}

pub async fn create(pool: &PgPool, input: &NewReport) -> Result<uuid::Uuid, AppError> {
    let id = uuid::Uuid::new_v4();
    sqlx::query("INSERT INTO reports (id, user_id, title, period_start, period_end, status, summary, platforms, categories, format) VALUES ($1,$2,$3,$4,$5,'pending','{}',$6,$7,$8)")
        .bind(id)
        .bind(input.user_id)
        .bind(&input.title)
        .bind(input.period_start)
        .bind(input.period_end)
        .bind(&input.platforms)
        .bind(&input.categories)
        .bind(&input.format)
        .execute(pool)
        .await?;
    Ok(id)
}

pub async fn find_one(
    pool: &PgPool,
    user_id: uuid::Uuid,
    id: uuid::Uuid,
) -> Result<Option<Report>, AppError> {
    sqlx::query_as::<_, Report>("SELECT id, title, period_start, period_end, status, file_url, summary, platforms, categories, format, error_message, created_at, completed_at FROM reports WHERE user_id = $1 AND id = $2")
        .bind(user_id)
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(AppError::from)
}

pub async fn count_pending(pool: &PgPool) -> Result<i64, AppError> {
    Ok(
        sqlx::query_scalar("SELECT COUNT(*) FROM reports WHERE status='pending'")
            .fetch_one(pool)
            .await?,
    )
}

pub async fn jobs_snapshot(pool: &PgPool) -> Result<serde_json::Value, AppError> {
    let pending_reports: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM reports WHERE status='pending'")
            .fetch_one(pool)
            .await?;
    let completed_24h: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM reports WHERE status='completed' AND completed_at>=NOW()-INTERVAL '24 hours'",
    )
    .fetch_one(pool)
    .await?;
    let pending_alerts: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM alert_deliveries WHERE status='pending'")
            .fetch_one(pool)
            .await
            .unwrap_or(0);
    let sent_24h: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM alert_deliveries WHERE status IN ('sent','logged') AND created_at>=NOW()-INTERVAL '24 hours'",
    )
    .fetch_one(pool)
    .await
    .unwrap_or(0);

    Ok(serde_json::json!({
        "pending_reports": pending_reports,
        "completed_reports_24h": completed_24h,
        "pending_alert_deliveries": pending_alerts,
        "sent_alert_deliveries_24h": sent_24h,
        "recent_reports": [],
        "recent_alert_deliveries": []
    }))
}

pub async fn user_can_access_export(
    pool: &PgPool,
    user_id: uuid::Uuid,
    is_admin: bool,
    filename: &str,
) -> Result<bool, AppError> {
    if is_admin {
        return Ok(
            sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM reports WHERE file_url=$1)")
                .bind(format!("/api/v1/exports/{filename}"))
                .fetch_one(pool)
                .await
                .unwrap_or(false),
        );
    }
    Ok(
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM reports WHERE user_id=$1 AND file_url=$2)")
            .bind(user_id)
            .bind(format!("/api/v1/exports/{filename}"))
            .fetch_one(pool)
            .await
            .unwrap_or(false),
    )
}

pub async fn latest_exports(pool: &PgPool) -> Result<Vec<Report>, AppError> {
    sqlx::query_as::<_, Report>("SELECT id, title, period_start, period_end, status, file_url, summary, platforms, categories, format, error_message, created_at, completed_at FROM reports WHERE file_url IS NOT NULL ORDER BY created_at DESC LIMIT 50")
        .fetch_all(pool)
        .await
        .map_err(AppError::from)
}

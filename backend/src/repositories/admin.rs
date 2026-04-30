use serde::Serialize;
use sqlx::PgPool;

use crate::{config::AppConfig, error::AppError};

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct AdminUser {
    pub id: uuid::Uuid,
    pub username: String,
    pub role: String,
    pub plan: String,
    pub email_verified: bool,
    pub subscription_status: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
#[derive(Debug, Default)]
pub struct AdminUserFilters {
    pub page: i64,
    pub page_size: i64,
    pub plan: Option<String>,
    pub role: Option<String>,
    pub search: Option<String>,
}

pub async fn list_users(
    pool: &PgPool,
    filters: &AdminUserFilters,
) -> Result<Vec<AdminUser>, AppError> {
    let page = filters.page.max(1);
    let page_size = filters.page_size.clamp(1, 50);
    let offset = (page - 1) * page_size;
    sqlx::query_as::<_,AdminUser>("SELECT u.id, u.username, u.role, u.plan::text as plan, u.email_verified, COALESCE((SELECT s.status FROM subscriptions s WHERE s.user_id=u.id ORDER BY s.created_at DESC LIMIT 1),'inactive') as subscription_status, u.created_at FROM users u WHERE ($1::text IS NULL OR u.plan::text = $1) AND ($2::text IS NULL OR u.role = $2) AND ($3::text IS NULL OR u.username ILIKE ('%' || $3 || '%')) ORDER BY u.created_at DESC LIMIT $4 OFFSET $5") .bind(filters.plan.as_deref()).bind(filters.role.as_deref()).bind(filters.search.as_deref()).bind(page_size).bind(offset).fetch_all(pool).await.map_err(AppError::from)
}

pub fn source_status(config: &AppConfig) -> serde_json::Value {
    serde_json::json!({"youtube": if config.youtube.api_key.is_empty(){"not_configured"} else {"configured"},"tiktok":"coming_soon","instagram":"coming_soon"})
}

pub async fn overview_snapshot(
    pool: &PgPool,
    config: &AppConfig,
) -> Result<serde_json::Value, AppError> {
    let (users_total, users_verified, users_admins):(i64,i64,i64)=sqlx::query_as("SELECT COUNT(*), COUNT(*) FILTER (WHERE email_verified=true), COUNT(*) FILTER (WHERE role='admin') FROM users").fetch_one(pool).await?;
    let (free,pro,studio):(i64,i64,i64)=sqlx::query_as("SELECT COUNT(*) FILTER (WHERE plan='free'), COUNT(*) FILTER (WHERE plan='pro'), COUNT(*) FILTER (WHERE plan='studio') FROM users").fetch_one(pool).await?;
    let (sub_total,sub_active,sub_inactive):(i64,i64,i64)=sqlx::query_as("SELECT COUNT(*), COUNT(*) FILTER (WHERE status='active'), COUNT(*) FILTER (WHERE status<>'active') FROM subscriptions").fetch_one(pool).await?;
    let (rules_enabled, alert_sent, alert_failed, alert_skipped):(i64,i64,i64,i64)=sqlx::query_as("SELECT (SELECT COUNT(*) FROM alerts WHERE enabled=true), (SELECT COUNT(*) FROM alert_deliveries WHERE status='sent' AND created_at>NOW()-INTERVAL '24 hours'), (SELECT COUNT(*) FROM alert_deliveries WHERE status='failed' AND created_at>NOW()-INTERVAL '24 hours'), (SELECT COUNT(*) FROM alert_deliveries WHERE status IN ('skipped','logged') AND created_at>NOW()-INTERVAL '24 hours')").fetch_one(pool).await?;
    let (rep_pending, rep_completed, rep_failed):(i64,i64,i64)=sqlx::query_as("SELECT COUNT(*) FILTER (WHERE status='pending'), COUNT(*) FILTER (WHERE status='completed' AND updated_at>NOW()-INTERVAL '24 hours'), COUNT(*) FILTER (WHERE status='failed' AND updated_at>NOW()-INTERVAL '24 hours') FROM reports").fetch_one(pool).await?;
    let (notif_total, notif_unread): (i64, i64) = sqlx::query_as(
        "SELECT COUNT(*), COUNT(*) FILTER (WHERE read_at IS NULL) FROM notifications",
    )
    .fetch_one(pool)
    .await?;
    let (mail_sent,mail_failed,mail_skipped):(i64,i64,i64)=sqlx::query_as("SELECT COUNT(*) FILTER (WHERE status='sent' AND created_at>NOW()-INTERVAL '24 hours'), COUNT(*) FILTER (WHERE status='failed' AND created_at>NOW()-INTERVAL '24 hours'), COUNT(*) FILTER (WHERE status='skipped' AND created_at>NOW()-INTERVAL '24 hours') FROM email_logs").fetch_one(pool).await?;
    Ok(
        serde_json::json!({"users":{"total":users_total,"verified":users_verified,"admins":users_admins},"plans":{"free":free,"pro":pro,"studio":studio},"subscriptions":{"total":sub_total,"active":sub_active,"inactive":sub_inactive},"alerts":{"rules_enabled":rules_enabled,"deliveries_sent_24h":alert_sent,"deliveries_failed_24h":alert_failed,"deliveries_skipped_24h":alert_skipped},"reports":{"pending":rep_pending,"completed_24h":rep_completed,"failed_24h":rep_failed},"notifications":{"total":notif_total,"unread":notif_unread},"emails":{"sent_24h":mail_sent,"failed_24h":mail_failed,"skipped_24h":mail_skipped},"sources":source_status(config)}),
    )
}

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
    sqlx::query_as::<_, AdminUser>(
        "SELECT u.id, u.username, u.role, u.plan::text as plan, u.email_verified,
         COALESCE((SELECT s.status FROM subscriptions s WHERE s.user_id=u.id ORDER BY s.created_at DESC LIMIT 1),'inactive') as subscription_status,
         u.created_at
         FROM users u
         WHERE ($1::text IS NULL OR u.plan::text = $1)
           AND ($2::text IS NULL OR u.role = $2)
           AND ($3::text IS NULL OR u.username ILIKE ('%' || $3 || '%'))
         ORDER BY u.created_at DESC LIMIT $4 OFFSET $5",
    )
    .bind(filters.plan.as_deref())
    .bind(filters.role.as_deref())
    .bind(filters.search.as_deref())
    .bind(page_size)
    .bind(offset)
    .fetch_all(pool)
    .await
    .map_err(AppError::from)
}

pub fn source_status(config: &AppConfig) -> serde_json::Value {
    serde_json::json!({
      "youtube": if config.youtube.api_key.is_empty() {"not_configured"} else {"configured"},
      "tiktok": if config.tiktok.api_key.is_empty() || config.tiktok.api_base.is_empty() {"not_configured"} else {"configured_but_not_implemented"},
      "instagram": if config.instagram.api_key.is_empty() || config.instagram.api_base.is_empty() {"not_configured"} else {"configured_but_not_implemented"}
    })
}

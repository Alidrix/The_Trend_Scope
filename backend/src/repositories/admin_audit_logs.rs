use crate::error::AppError;
use serde_json::Value;
use sqlx::PgPool;

#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize)]
pub struct AdminAuditLogRow {
    pub id: uuid::Uuid,
    pub admin_username: String,
    pub action: String,
    pub target: Option<String>,
    pub status: String,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub metadata: Value,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

pub struct CreateAdminAuditLog<'a> {
    pub admin_username: &'a str,
    pub action: &'a str,
    pub target: Option<&'a str>,
    pub status: &'a str,
    pub ip_address: Option<&'a str>,
    pub user_agent: Option<&'a str>,
    pub metadata: Value,
}

pub async fn create(pool: &PgPool, input: CreateAdminAuditLog<'_>) -> Result<(), AppError> {
    sqlx::query(
        r#"
        INSERT INTO admin_audit_logs
          (admin_username, action, target, status, ip_address, user_agent, metadata)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#,
    )
    .bind(input.admin_username)
    .bind(input.action)
    .bind(input.target)
    .bind(input.status)
    .bind(input.ip_address)
    .bind(input.user_agent)
    .bind(input.metadata)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn latest(pool: &PgPool, limit: i64) -> Result<Vec<AdminAuditLogRow>, AppError> {
    sqlx::query_as::<_, AdminAuditLogRow>(
        r#"
        SELECT id, admin_username, action, target, status, ip_address, user_agent, metadata, created_at
        FROM admin_audit_logs
        ORDER BY created_at DESC
        LIMIT $1
        "#,
    )
    .bind(limit.clamp(1, 200))
    .fetch_all(pool)
    .await
    .map_err(AppError::from)
}

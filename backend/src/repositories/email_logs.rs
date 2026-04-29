use crate::error::AppError;
use serde::Serialize;
use sqlx::PgPool;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct EmailLogRow {
    pub recipient: String,
    pub subject: String,
    pub status: String,
    pub error_message: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

pub async fn log(
    pool: &PgPool,
    user_id: Option<uuid::Uuid>,
    recipient: &str,
    subject: &str,
    status: &str,
    provider_message_id: Option<&str>,
    error_message: Option<&str>,
) -> Result<(), AppError> {
    sqlx::query(
        "INSERT INTO email_logs (user_id, recipient, subject, status, provider_message_id, error_message) VALUES ($1, $2, $3, $4, $5, $6)",
    )
    .bind(user_id)
    .bind(recipient)
    .bind(subject)
    .bind(status)
    .bind(provider_message_id)
    .bind(error_message)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn latest(pool: &PgPool, limit: i64) -> Result<Vec<EmailLogRow>, AppError> {
    sqlx::query_as::<_, EmailLogRow>(
        "SELECT recipient, subject, status, error_message, created_at FROM email_logs ORDER BY created_at DESC LIMIT $1",
    )
    .bind(limit)
    .fetch_all(pool)
    .await
    .map_err(AppError::from)
}

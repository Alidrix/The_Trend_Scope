use crate::error::AppError;
use sqlx::PgPool;

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

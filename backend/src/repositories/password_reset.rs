use crate::error::AppError;
use sqlx::PgPool;

pub async fn create_token(
    pool: &PgPool,
    user_id: uuid::Uuid,
    token_hash: &str,
    expires_at: chrono::DateTime<chrono::Utc>,
) -> Result<(), AppError> {
    sqlx::query(
        "INSERT INTO password_reset_tokens (user_id, token_hash, expires_at) VALUES ($1, $2, $3)",
    )
    .bind(user_id)
    .bind(token_hash)
    .bind(expires_at)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn consume_token(
    pool: &PgPool,
    token_hash: &str,
) -> Result<Option<uuid::Uuid>, AppError> {
    sqlx::query_scalar(
        "UPDATE password_reset_tokens\n         SET used_at = NOW()\n         WHERE token_hash = $1 AND used_at IS NULL AND expires_at > NOW()\n         RETURNING user_id",
    )
    .bind(token_hash)
    .fetch_optional(pool)
    .await
    .map_err(AppError::from)
}

use crate::error::AppError;
use sqlx::PgPool;

pub async fn create_token(
    pool: &PgPool,
    user_id: uuid::Uuid,
    token_hash: &str,
    expires_at: chrono::DateTime<chrono::Utc>,
) -> Result<(), AppError> {
    sqlx::query("INSERT INTO email_verification_tokens (user_id, token_hash, expires_at) VALUES ($1, $2, $3)")
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
    let mut tx = pool.begin().await?;

    let user_id = sqlx::query_scalar::<_, uuid::Uuid>(
        "SELECT user_id FROM email_verification_tokens WHERE token_hash = $1 AND used_at IS NULL AND expires_at > NOW() ORDER BY created_at DESC LIMIT 1 FOR UPDATE",
    )
    .bind(token_hash)
    .fetch_optional(&mut *tx)
    .await?;

    if let Some(user_id) = user_id {
        sqlx::query("UPDATE email_verification_tokens SET used_at = NOW() WHERE token_hash = $1 AND used_at IS NULL")
            .bind(token_hash)
            .execute(&mut *tx)
            .await?;
        tx.commit().await?;
        Ok(Some(user_id))
    } else {
        tx.rollback().await?;
        Ok(None)
    }
}

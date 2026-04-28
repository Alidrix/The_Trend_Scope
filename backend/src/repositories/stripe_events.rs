use crate::error::AppError;
use sqlx::PgPool;

pub async fn is_processed(pool: &PgPool, event_id: &str) -> Result<bool, AppError> {
    let exists: Option<String> = sqlx::query_scalar("SELECT id FROM stripe_events WHERE id = $1")
        .bind(event_id)
        .fetch_optional(pool)
        .await?;
    Ok(exists.is_some())
}

pub async fn mark_processed(
    pool: &PgPool,
    event_id: &str,
    event_type: &str,
    payload: &serde_json::Value,
) -> Result<(), AppError> {
    sqlx::query(
        "INSERT INTO stripe_events (id, event_type, payload) VALUES ($1, $2, $3) ON CONFLICT (id) DO NOTHING",
    )
    .bind(event_id)
    .bind(event_type)
    .bind(payload)
    .execute(pool)
    .await?;
    Ok(())
}

use crate::error::AppError;
use sqlx::PgPool;

pub async fn current_plan(pool: &PgPool, username: &str) -> Result<Option<String>, AppError> {
    sqlx::query_scalar("SELECT plan::text FROM users WHERE username = $1")
        .bind(username)
        .fetch_optional(pool)
        .await
        .map_err(AppError::from)
}

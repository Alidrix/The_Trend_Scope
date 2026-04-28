use sqlx::PgPool;

use crate::error::AppError;

pub async fn ensure_admin(pool: &PgPool, user_id_or_username: &str) -> Result<(), AppError> {
    let role: Option<String> =
        sqlx::query_scalar("SELECT role FROM users WHERE username = $1 OR id::text = $1 LIMIT 1")
            .bind(user_id_or_username)
            .fetch_optional(pool)
            .await?;

    match role.as_deref() {
        Some("admin") => Ok(()),
        Some(_) | None => Err(AppError::Forbidden),
    }
}

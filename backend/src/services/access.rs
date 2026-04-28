use sqlx::PgPool;

use crate::{error::AppError, AuthBearer};

pub async fn ensure_admin(pool: &PgPool, auth: &AuthBearer) -> Result<(), AppError> {
    let role = sqlx::query_scalar::<_, String>("SELECT role FROM users WHERE username = $1")
        .bind(&auth.sub)
        .fetch_optional(pool)
        .await?
        .unwrap_or_else(|| "user".to_string());

    if role == "admin" {
        Ok(())
    } else {
        Err(AppError::Forbidden)
    }
}

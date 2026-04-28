use crate::error::AppError;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn insert_log(
    pool: &PgPool,
    user_id: Option<Uuid>,
    action: &str,
    resource: &str,
) -> Result<(), AppError> {
    sqlx::query("INSERT INTO audit_logs (user_id, action, resource) VALUES ($1, $2, $3)")
        .bind(user_id)
        .bind(action)
        .bind(resource)
        .execute(pool)
        .await?;
    Ok(())
}

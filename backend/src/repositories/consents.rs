use crate::error::AppError;
use serde::Serialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct ConsentRow {
    pub consent_type: String,
    pub granted: bool,
    pub version: String,
}

pub async fn list_consents(pool: &PgPool, user_id: Uuid) -> Result<Vec<ConsentRow>, AppError> {
    sqlx::query_as::<_, ConsentRow>(
        "SELECT consent_type, granted, version FROM consents WHERE user_id = $1 ORDER BY created_at DESC",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
    .map_err(AppError::from)
}

pub async fn insert_consent(
    pool: &PgPool,
    user_id: Uuid,
    consent_type: &str,
    granted: bool,
    version: &str,
) -> Result<(), AppError> {
    sqlx::query(
        "INSERT INTO consents (user_id, consent_type, granted, version) VALUES ($1, $2, $3, $4)",
    )
    .bind(user_id)
    .bind(consent_type)
    .bind(granted)
    .bind(version)
    .execute(pool)
    .await?;
    Ok(())
}

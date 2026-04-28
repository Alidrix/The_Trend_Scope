use crate::{
    error::AppError,
    repositories::{audit_logs, consents},
};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn record_consent(
    pool: &PgPool,
    user_id: Uuid,
    consent_type: &str,
    granted: bool,
    version: &str,
) -> Result<(), AppError> {
    consents::insert_consent(pool, user_id, consent_type, granted, version).await?;
    audit_logs::insert_log(pool, Some(user_id), "consent.update", "consents").await
}

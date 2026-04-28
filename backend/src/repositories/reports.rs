use serde::Serialize;
use sqlx::PgPool;

use crate::error::AppError;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Report {
    pub id: uuid::Uuid,
    pub title: String,
    pub period_start: chrono::NaiveDate,
    pub period_end: chrono::NaiveDate,
    pub status: String,
    pub file_url: Option<String>,
    pub summary: serde_json::Value,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
}

pub async fn list(pool: &PgPool, user_id: uuid::Uuid) -> Result<Vec<Report>, AppError> {
    sqlx::query_as::<_, Report>("SELECT id, title, period_start, period_end, status, file_url, summary, created_at, completed_at FROM reports WHERE user_id = $1 ORDER BY created_at DESC")
        .bind(user_id)
        .fetch_all(pool)
        .await
        .map_err(AppError::from)
}

pub async fn create(
    pool: &PgPool,
    user_id: uuid::Uuid,
    title: &str,
    start: chrono::NaiveDate,
    end: chrono::NaiveDate,
) -> Result<uuid::Uuid, AppError> {
    let id = uuid::Uuid::new_v4();
    sqlx::query("INSERT INTO reports (id, user_id, title, period_start, period_end, status, summary) VALUES ($1,$2,$3,$4,$5,'pending','{}')")
        .bind(id)
        .bind(user_id)
        .bind(title)
        .bind(start)
        .bind(end)
        .execute(pool)
        .await?;
    Ok(id)
}

pub async fn find_one(
    pool: &PgPool,
    user_id: uuid::Uuid,
    id: uuid::Uuid,
) -> Result<Option<Report>, AppError> {
    sqlx::query_as::<_, Report>("SELECT id, title, period_start, period_end, status, file_url, summary, created_at, completed_at FROM reports WHERE user_id = $1 AND id = $2")
        .bind(user_id)
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(AppError::from)
}

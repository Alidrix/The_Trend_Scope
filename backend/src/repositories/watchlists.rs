use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::error::AppError;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Watchlist {
    pub id: uuid::Uuid,
    pub name: String,
    pub keywords: Vec<String>,
    pub categories: Vec<String>,
    pub platforms: Vec<String>,
    pub regions: Vec<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

pub async fn list(pool: &PgPool, user_id: uuid::Uuid) -> Result<Vec<Watchlist>, AppError> {
    sqlx::query_as::<_, Watchlist>(
        "SELECT id, name, keywords, categories, platforms, regions, created_at, updated_at FROM watchlists WHERE user_id = $1 ORDER BY created_at DESC",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
    .map_err(AppError::from)
}

pub async fn count(pool: &PgPool, user_id: uuid::Uuid) -> Result<i64, AppError> {
    sqlx::query_scalar("SELECT COUNT(*) FROM watchlists WHERE user_id = $1")
        .bind(user_id)
        .fetch_one(pool)
        .await
        .map_err(AppError::from)
}

pub async fn create(
    pool: &PgPool,
    user_id: uuid::Uuid,
    item: &WatchlistPayload,
) -> Result<(), AppError> {
    sqlx::query("INSERT INTO watchlists (user_id, name, keywords, categories, platforms, regions) VALUES ($1, $2, $3, $4, $5, $6)")
        .bind(user_id)
        .bind(&item.name)
        .bind(&item.keywords)
        .bind(&item.categories)
        .bind(&item.platforms)
        .bind(&item.regions)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn update(
    pool: &PgPool,
    user_id: uuid::Uuid,
    id: uuid::Uuid,
    item: &WatchlistPayload,
) -> Result<(), AppError> {
    sqlx::query("UPDATE watchlists SET name = $3, keywords = $4, categories = $5, platforms = $6, regions = $7, updated_at = NOW() WHERE id = $1 AND user_id = $2")
        .bind(id)
        .bind(user_id)
        .bind(&item.name)
        .bind(&item.keywords)
        .bind(&item.categories)
        .bind(&item.platforms)
        .bind(&item.regions)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn delete(pool: &PgPool, user_id: uuid::Uuid, id: uuid::Uuid) -> Result<(), AppError> {
    sqlx::query("DELETE FROM watchlists WHERE id = $1 AND user_id = $2")
        .bind(id)
        .bind(user_id)
        .execute(pool)
        .await?;
    Ok(())
}

#[derive(Debug, Deserialize)]
pub struct WatchlistPayload {
    pub name: String,
    #[serde(default)]
    pub keywords: Vec<String>,
    #[serde(default)]
    pub categories: Vec<String>,
    #[serde(default)]
    pub platforms: Vec<String>,
    #[serde(default)]
    pub regions: Vec<String>,
}

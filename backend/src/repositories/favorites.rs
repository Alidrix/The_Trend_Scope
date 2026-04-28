use serde::Serialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppError;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct FavoriteItem {
    pub platform: String,
    pub trend_id: String,
    pub title: Option<String>,
    pub thumbnail_url: Option<String>,
    pub views_per_hour: Option<i64>,
    pub trend_score: Option<f64>,
    pub category: Option<String>,
    pub region: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

pub async fn list(pool: &PgPool, user_id: Uuid) -> Result<Vec<FavoriteItem>, AppError> {
    let rows = sqlx::query_as::<_, FavoriteItem>(
        "SELECT f.platform, f.trend_id, v.title, v.thumbnail_url, v.views_per_hour,\n                CASE WHEN v.views_per_hour IS NULL THEN NULL ELSE LEAST(100.0, GREATEST(0.0, v.views_per_hour::double precision / 300.0)) END as trend_score,\n                v.category, v.region, f.created_at\n         FROM favorites f\n         LEFT JOIN videos v ON v.youtube_id = f.trend_id\n         WHERE f.user_id = $1 ORDER BY f.created_at DESC",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn create(
    pool: &PgPool,
    user_id: Uuid,
    platform: &str,
    trend_id: &str,
) -> Result<(), AppError> {
    sqlx::query("INSERT INTO favorites (user_id, platform, trend_id) VALUES ($1, $2, $3) ON CONFLICT (user_id, platform, trend_id) DO NOTHING")
        .bind(user_id)
        .bind(platform)
        .bind(trend_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn delete(
    pool: &PgPool,
    user_id: Uuid,
    platform: &str,
    trend_id: &str,
) -> Result<(), AppError> {
    sqlx::query("DELETE FROM favorites WHERE user_id = $1 AND platform = $2 AND trend_id = $3")
        .bind(user_id)
        .bind(platform)
        .bind(trend_id)
        .execute(pool)
        .await?;
    Ok(())
}

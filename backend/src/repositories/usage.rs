use crate::{error::AppError, models::video::Video};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn mark_daily_views(
    pool: &PgPool,
    user_id: Uuid,
    trends: &[Video],
) -> Result<u64, AppError> {
    let mut inserted = 0u64;
    for trend in trends {
        let result = sqlx::query(
            "INSERT INTO trend_views (user_id, trend_id, platform, viewed_date)
             VALUES ($1, $2, 'youtube', CURRENT_DATE)
             ON CONFLICT (user_id, trend_id, platform, viewed_date) DO NOTHING",
        )
        .bind(user_id)
        .bind(&trend.youtube_id)
        .execute(pool)
        .await?;
        inserted += result.rows_affected();
    }
    Ok(inserted)
}

pub async fn count_unique_daily_views(pool: &PgPool, user_id: Uuid) -> Result<i64, AppError> {
    sqlx::query_scalar(
        "SELECT COUNT(*) FROM trend_views WHERE user_id = $1 AND viewed_date = CURRENT_DATE",
    )
    .bind(user_id)
    .fetch_one(pool)
    .await
    .map_err(AppError::from)
}

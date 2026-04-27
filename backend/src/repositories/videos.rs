use crate::{error::AppError, models::video::NewVideo};
use chrono::Utc;
use sqlx::{PgPool, Row};

pub async fn upsert_video(pool: &PgPool, item: &NewVideo) -> Result<(uuid::Uuid, bool), AppError> {
    let row = sqlx::query(
        r#"
        INSERT INTO videos (
            id, youtube_id, title, category, region, thumbnail_url, channel_title, description, url,
            views_per_hour, duration_seconds, published_at, notes, last_seen_at, updated_at
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, NOW(), NOW())
        ON CONFLICT (youtube_id) DO UPDATE SET
            title = EXCLUDED.title,
            category = EXCLUDED.category,
            region = EXCLUDED.region,
            thumbnail_url = EXCLUDED.thumbnail_url,
            channel_title = EXCLUDED.channel_title,
            description = EXCLUDED.description,
            url = EXCLUDED.url,
            views_per_hour = EXCLUDED.views_per_hour,
            duration_seconds = EXCLUDED.duration_seconds,
            published_at = EXCLUDED.published_at,
            last_seen_at = NOW(),
            updated_at = NOW()
        RETURNING id, (xmax = 0) AS inserted
        "#,
    )
    .bind(uuid::Uuid::new_v4())
    .bind(&item.youtube_id)
    .bind(&item.title)
    .bind(&item.category)
    .bind(&item.region)
    .bind(&item.thumbnail_url)
    .bind(&item.channel_title)
    .bind(&item.description)
    .bind(&item.url)
    .bind(item.views_per_hour)
    .bind(item.duration_seconds)
    .bind(item.published_at)
    .bind(Option::<String>::None)
    .fetch_one(pool)
    .await?;

    Ok((row.get("id"), row.get("inserted")))
}

pub async fn insert_video_stat(
    pool: &PgPool,
    video_id: uuid::Uuid,
    vph: i64,
) -> Result<(), AppError> {
    sqlx::query(
        "INSERT INTO video_stats (id, video_id, views_per_hour, collected_at) VALUES ($1, $2, $3, $4)",
    )
    .bind(uuid::Uuid::new_v4())
    .bind(video_id)
    .bind(vph)
    .bind(Utc::now())
    .execute(pool)
    .await?;

    Ok(())
}

use axum::{extract::State, Json};
use sqlx::PgPool;
use tracing::error;

use crate::{
    error::{ApiMessage, AppError},
    models::video::{ApiVideosResponse, NewVideo, ScanResponse, Video, VideoPayload},
    repositories::videos::{insert_video_stat, upsert_video},
    services::access::ensure_admin,
    services::youtube::scan_theme_region,
    state::AppState,
    AuthBearer,
};

pub async fn list_videos(
    auth: AuthBearer,
    State(state): State<AppState>,
) -> Result<Json<ApiVideosResponse>, AppError> {
    ensure_admin(&state.pool, &auth.sub).await?;
    let records = sqlx::query(
        "SELECT id, platform, youtube_id, title, category, region, thumbnail_url, channel_title, description, url, views_per_hour, duration_seconds, published_at, notes FROM videos ORDER BY views_per_hour DESC",
    )
    .map(Video::from_row)
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(ApiVideosResponse { videos: records }))
}

pub async fn refresh_videos(
    auth: AuthBearer,
    State(state): State<AppState>,
    Json(payload): Json<Vec<VideoPayload>>,
) -> Result<Json<ApiMessage>, AppError> {
    ensure_admin(&state.pool, &auth.sub).await?;
    for item in payload {
        let candidate = NewVideo {
            platform: item.platform,
            youtube_id: item.youtube_id,
            title: item.title,
            category: item.category,
            region: "manual".into(),
            thumbnail_url: None,
            channel_title: None,
            description: None,
            url: "".into(),
            views_per_hour: item.views_per_hour,
            duration_seconds: item.duration_seconds,
            published_at: item.published_at,
        };

        let (video_id, _) = upsert_video(&state.pool, &candidate).await?;
        insert_video_stat(
            &state.pool,
            video_id,
            &candidate.platform,
            candidate.views_per_hour,
        )
        .await?;
    }

    Ok(Json(ApiMessage {
        message: "videos refreshed".into(),
    }))
}

pub async fn scan_videos(
    auth: AuthBearer,
    State(state): State<AppState>,
) -> Result<Json<ScanResponse>, AppError> {
    ensure_admin(&state.pool, &auth.sub).await?;
    if state.config.youtube.api_key.is_empty() {
        return Err(AppError::BadRequest(
            "YOUTUBE_API_KEY is missing in environment".into(),
        ));
    }

    let mut inserted = 0usize;
    let mut updated = 0usize;

    for region in &state.config.youtube.regions {
        for theme in &state.config.youtube.themes {
            let scanned =
                match scan_theme_region(&state.http, &state.config.youtube.api_key, region, theme)
                    .await
                {
                    Ok(videos) => videos,
                    Err(err) => {
                        error!(region, theme, error = %err, "youtube scan failed for region/theme");
                        continue;
                    }
                };

            process_scanned_videos(&state.pool, scanned, &mut inserted, &mut updated).await?;
        }
    }

    Ok(Json(ScanResponse {
        message: "scan completed".into(),
        inserted,
        updated,
        total: inserted + updated,
    }))
}

async fn process_scanned_videos(
    pool: &PgPool,
    videos: Vec<NewVideo>,
    inserted: &mut usize,
    updated: &mut usize,
) -> Result<(), AppError> {
    for item in videos {
        let (video_id, was_inserted) = upsert_video(pool, &item).await?;
        insert_video_stat(pool, video_id, &item.platform, item.views_per_hour).await?;

        if was_inserted {
            *inserted += 1;
        } else {
            *updated += 1;
        }
    }

    Ok(())
}

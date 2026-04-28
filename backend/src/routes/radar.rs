use axum::{extract::State, Json};
use serde::Serialize;

use crate::{
    error::AppError,
    models::{
        plan::{PlanLimits, PlanTier},
        video::Video,
    },
    repositories::usage,
    state::AppState,
    AuthBearer,
};

#[derive(Debug, Serialize)]
pub struct RadarResponse {
    pub plan: PlanTier,
    pub limits: PlanLimits,
    pub usage_today: i64,
    pub remaining_today: Option<i64>,
    pub trends: Vec<Video>,
    pub upgrade_required: bool,
    pub message: Option<String>,
}

pub async fn daily_radar(
    auth: AuthBearer,
    State(state): State<AppState>,
) -> Result<Json<RadarResponse>, AppError> {
    let user = sqlx::query_scalar::<_, uuid::Uuid>("SELECT id FROM users WHERE username = $1")
        .bind(&auth.sub)
        .fetch_one(&state.pool)
        .await?;

    let tier: PlanTier = sqlx::query_scalar("SELECT plan FROM users WHERE id = $1")
        .bind(user)
        .fetch_one(&state.pool)
        .await?;

    let limits = PlanLimits::from_tier(tier);
    let usage_today = usage::count_unique_daily_views(&state.pool, user).await?;

    let fetch_limit = limits.daily_trend_limit.unwrap_or(100) as i64;
    let trends = sqlx::query(
        "SELECT id, youtube_id, title, category, region, thumbnail_url, channel_title, description, url, views_per_hour, duration_seconds, published_at, notes FROM videos ORDER BY views_per_hour DESC LIMIT $1",
    )
    .bind(fetch_limit)
    .map(Video::from_row)
    .fetch_all(&state.pool)
    .await?;

    let (visible_trends, upgrade_required, message) = if let Some(limit) = limits.daily_trend_limit
    {
        let allowed = (limit as i64 - usage_today).max(0) as usize;
        let visible = trends.into_iter().take(allowed).collect::<Vec<_>>();
        let blocked = allowed == 0;
        let msg = if blocked {
            Some("Tu as consulté tes 3 tendances gratuites du jour. Passe en Pro pour débloquer toutes les tendances.".to_string())
        } else {
            None
        };
        (visible, blocked, msg)
    } else {
        (trends, false, None)
    };

    let newly_counted = usage::mark_daily_views(&state.pool, user, &visible_trends).await? as i64;
    let new_usage_today = usage_today + newly_counted;

    let remaining_today = limits
        .daily_trend_limit
        .map(|limit| (limit as i64 - new_usage_today).max(0));

    Ok(Json(RadarResponse {
        plan: tier,
        limits,
        usage_today: new_usage_today,
        remaining_today,
        trends: visible_trends,
        upgrade_required,
        message,
    }))
}

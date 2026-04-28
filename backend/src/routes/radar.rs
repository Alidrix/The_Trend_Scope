use axum::{
    extract::{Query, State},
    Json,
};
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Deserialize)]
pub struct RadarFilters {
    pub platform: Option<String>,
    pub region: Option<String>,
    pub category: Option<String>,
    pub format: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct RadarResponse {
    pub plan: PlanTier,
    pub remaining_today: Option<i64>,
    pub upgrade_required: bool,
    pub kpis: serde_json::Value,
    pub filters: serde_json::Value,
    pub trends: Vec<Video>,
}

pub async fn daily_radar(
    auth: AuthBearer,
    State(state): State<AppState>,
    Query(filters): Query<RadarFilters>,
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
        "SELECT id, youtube_id, title, category, region, thumbnail_url, channel_title, description, url, views_per_hour, duration_seconds, published_at, notes\n         FROM videos\n         WHERE ($1::TEXT IS NULL OR region = $1)\n           AND ($2::TEXT IS NULL OR category = $2)\n         ORDER BY views_per_hour DESC LIMIT $3",
    )
    .bind(filters.region.clone())
    .bind(filters.category.clone())
    .bind(fetch_limit)
    .map(Video::from_row)
    .fetch_all(&state.pool)
    .await?;

    let (visible_trends, upgrade_required) = if let Some(limit) = limits.daily_trend_limit {
        let allowed = (limit as i64 - usage_today).max(0) as usize;
        let visible = trends.into_iter().take(allowed).collect::<Vec<_>>();
        (visible, allowed == 0)
    } else {
        (trends, false)
    };

    let newly_counted = usage::mark_daily_views(&state.pool, user, &visible_trends).await? as i64;
    let new_usage_today = usage_today + newly_counted;

    let remaining_today = limits
        .daily_trend_limit
        .map(|limit| (limit as i64 - new_usage_today).max(0));

    let avg_views = if visible_trends.is_empty() {
        0
    } else {
        visible_trends.iter().map(|t| t.views_per_hour).sum::<i64>() / visible_trends.len() as i64
    };

    let strong_opps = visible_trends
        .iter()
        .filter(|t| t.views_per_hour >= 20_000)
        .count();

    Ok(Json(RadarResponse {
        plan: tier,
        remaining_today,
        upgrade_required,
        kpis: serde_json::json!({
            "trends_detected": visible_trends.len(),
            "average_views_per_hour": avg_views,
            "rising_niches": visible_trends.iter().filter(|t| t.views_per_hour >= 10_000).count(),
            "strong_opportunities": strong_opps
        }),
        filters: serde_json::json!({
            "platform": filters.platform.unwrap_or_else(|| "youtube".into()),
            "region": filters.region.unwrap_or_else(|| "FR".into()),
            "category": filters.category.unwrap_or_else(|| "business".into()),
            "format": filters.format,
        }),
        trends: visible_trends,
    }))
}

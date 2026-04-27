use axum::{extract::State, Json};
use serde::Serialize;

use crate::{
    error::AppError,
    models::{
        plan::{PlanLimits, PlanTier},
        video::Video,
    },
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
    let usage_today: i64 = sqlx::query_scalar(
        "SELECT trends_viewed FROM user_usage_daily WHERE user_id = $1 AND usage_date = CURRENT_DATE",
    )
    .bind(user)
    .fetch_optional(&state.pool)
    .await?
    .unwrap_or(0);

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
            Some(
                "Tu as consulté tes 3 tendances gratuites du jour. Passe en Pro pour débloquer toutes les tendances."
                    .to_string(),
            )
        } else {
            None
        };
        (visible, blocked, msg)
    } else {
        (trends, false, None)
    };

    let shown = visible_trends.len() as i64;
    if shown > 0 {
        sqlx::query(
            r#"INSERT INTO user_usage_daily (user_id, usage_date, trends_viewed)
               VALUES ($1, CURRENT_DATE, $2)
               ON CONFLICT (user_id, usage_date)
               DO UPDATE SET trends_viewed = user_usage_daily.trends_viewed + EXCLUDED.trends_viewed"#,
        )
        .bind(user)
        .bind(shown)
        .execute(&state.pool)
        .await?;
    }

    let remaining_today = limits
        .daily_trend_limit
        .map(|limit| (limit as i64 - usage_today - shown).max(0));

    Ok(Json(RadarResponse {
        plan: tier,
        limits,
        usage_today: usage_today + shown,
        remaining_today,
        trends: visible_trends,
        upgrade_required,
        message,
    }))
}

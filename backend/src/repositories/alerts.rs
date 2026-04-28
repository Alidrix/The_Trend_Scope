use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::error::AppError;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct AlertRule {
    pub id: uuid::Uuid,
    pub name: String,
    pub platform: Option<String>,
    pub region: Option<String>,
    pub category: Option<String>,
    pub keyword: Option<String>,
    pub min_views_per_hour: Option<i64>,
    pub min_trend_score: Option<f64>,
    pub channel: String,
    pub enabled: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize)]
pub struct AlertRulePayload {
    pub name: String,
    pub platform: Option<String>,
    pub region: Option<String>,
    pub category: Option<String>,
    pub keyword: Option<String>,
    pub min_views_per_hour: Option<i64>,
    pub min_trend_score: Option<f64>,
    pub channel: Option<String>,
    pub enabled: Option<bool>,
}

pub async fn list(pool: &PgPool, user_id: uuid::Uuid) -> Result<Vec<AlertRule>, AppError> {
    sqlx::query_as::<_, AlertRule>("SELECT id, name, platform, region, category, keyword, min_views_per_hour, min_trend_score, channel, enabled, created_at FROM alert_rules WHERE user_id = $1 ORDER BY created_at DESC")
        .bind(user_id)
        .fetch_all(pool)
        .await
        .map_err(AppError::from)
}

pub async fn create(
    pool: &PgPool,
    user_id: uuid::Uuid,
    p: &AlertRulePayload,
) -> Result<(), AppError> {
    sqlx::query("INSERT INTO alert_rules (user_id, name, platform, region, category, keyword, min_views_per_hour, min_trend_score, channel, enabled) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10)")
        .bind(user_id)
        .bind(&p.name)
        .bind(&p.platform)
        .bind(&p.region)
        .bind(&p.category)
        .bind(&p.keyword)
        .bind(p.min_views_per_hour)
        .bind(p.min_trend_score)
        .bind(p.channel.clone().unwrap_or_else(|| "web".into()))
        .bind(p.enabled.unwrap_or(true))
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn update(
    pool: &PgPool,
    user_id: uuid::Uuid,
    id: uuid::Uuid,
    p: &AlertRulePayload,
) -> Result<(), AppError> {
    sqlx::query("UPDATE alert_rules SET name=$3, platform=$4, region=$5, category=$6, keyword=$7, min_views_per_hour=$8, min_trend_score=$9, channel=$10, enabled=$11, updated_at=NOW() WHERE id=$1 AND user_id=$2")
        .bind(id)
        .bind(user_id)
        .bind(&p.name)
        .bind(&p.platform)
        .bind(&p.region)
        .bind(&p.category)
        .bind(&p.keyword)
        .bind(p.min_views_per_hour)
        .bind(p.min_trend_score)
        .bind(p.channel.clone().unwrap_or_else(|| "web".into()))
        .bind(p.enabled.unwrap_or(true))
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn delete(pool: &PgPool, user_id: uuid::Uuid, id: uuid::Uuid) -> Result<(), AppError> {
    sqlx::query("DELETE FROM alert_rules WHERE id = $1 AND user_id = $2")
        .bind(id)
        .bind(user_id)
        .execute(pool)
        .await?;
    Ok(())
}

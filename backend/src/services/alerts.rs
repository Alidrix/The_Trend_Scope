use crate::{error::AppError, repositories::alerts};
use sqlx::{PgPool, Row};

pub async fn process_alert_rules_for_recent_trends(pool: &PgPool) -> Result<u64, AppError> {
    let rules = alerts::list_enabled(pool).await?;
    let trends = sqlx::query("SELECT youtube_id, title, COALESCE(description,''), platform, COALESCE(region,''), COALESCE(category,''), views_per_hour FROM videos WHERE updated_at >= NOW() - INTERVAL '2 hours' OR published_at >= NOW() - INTERVAL '48 hours' ORDER BY views_per_hour DESC LIMIT 200").fetch_all(pool).await?;
    let mut created = 0;
    for r in rules {
        for t in &trends {
            let platform: String = t.get(3);
            let region: String = t.get(4);
            let category: String = t.get(5);
            let title: String = t.get(1);
            let desc: String = t.get(2);
            let vph: i64 = t.get(6);
            let trend_id: String = t.get(0);
            let score = ((vph as f64) / 1000.0).min(100.0);
            if r.platform.as_deref().is_some_and(|x| x != platform) {
                continue;
            }
            if r.region.as_deref().is_some_and(|x| x != region) {
                continue;
            }
            if r.category.as_deref().is_some_and(|x| x != category) {
                continue;
            }
            if r.keyword.as_deref().is_some_and(|k| {
                !format!("{} {}", title.to_lowercase(), desc.to_lowercase())
                    .contains(&k.to_lowercase())
            }) {
                continue;
            }
            if r.min_views_per_hour.is_some_and(|m| vph < m) {
                continue;
            }
            if r.min_trend_score.is_some_and(|m| score < m) {
                continue;
            }
            let status = match r.channel.as_str() {
                "web" => "logged",
                "email" => "pending",
                _ => "pending",
            };
            let res=sqlx::query("INSERT INTO alert_deliveries (alert_rule_id,status,payload,platform,trend_id) VALUES ($1,$2,$3,$4,$5) ON CONFLICT (alert_rule_id,platform,trend_id) DO NOTHING")
        .bind(r.id).bind(status).bind(serde_json::json!({"title":title,"platform":platform,"views_per_hour":vph})).bind(&platform).bind(&trend_id).execute(pool).await?;
            created += res.rows_affected();
        }
    }
    Ok(created)
}

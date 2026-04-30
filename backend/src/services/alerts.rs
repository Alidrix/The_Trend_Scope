use crate::{
    config::AppConfig,
    error::AppError,
    repositories::{alerts, notifications},
    services::{
        email,
        telegram::{send_telegram_alert, TelegramAlertMessage},
    },
};
use sqlx::{PgPool, Row};

fn mask_chat_id(value: &str) -> String {
    if value.len() <= 4 {
        "****".into()
    } else {
        format!("***{}", &value[value.len() - 4..])
    }
}

#[derive(Debug, Clone)]
pub struct AlertRuleMatchInput {
    pub platform: Option<String>,
    pub region: Option<String>,
    pub category: Option<String>,
    pub keyword: Option<String>,
    pub min_views_per_hour: Option<i64>,
    pub min_trend_score: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct TrendMatchInput {
    pub platform: String,
    pub region: String,
    pub category: String,
    pub title: String,
    pub description: String,
    pub views_per_hour: i64,
}

pub fn approximate_trend_score(views_per_hour: i64) -> f64 {
    (views_per_hour as f64 / 1000.0).min(100.0)
}

pub fn alert_matches_rule(rule: &AlertRuleMatchInput, trend: &TrendMatchInput) -> bool {
    let score = approximate_trend_score(trend.views_per_hour);
    if rule
        .platform
        .as_deref()
        .is_some_and(|x| x != trend.platform)
        || rule.region.as_deref().is_some_and(|x| x != trend.region)
        || rule
            .category
            .as_deref()
            .is_some_and(|x| x != trend.category)
    {
        return false;
    }
    if rule.keyword.as_deref().is_some_and(|k| {
        !format!(
            "{} {}",
            trend.title.to_lowercase(),
            trend.description.to_lowercase()
        )
        .contains(&k.to_lowercase())
    }) {
        return false;
    }
    if rule
        .min_views_per_hour
        .is_some_and(|m| trend.views_per_hour < m)
        || rule.min_trend_score.is_some_and(|m| score < m)
    {
        return false;
    }
    true
}
pub async fn process_alert_rules_for_recent_trends(
    pool: &PgPool,
    config: &AppConfig,
) -> Result<u64, AppError> {
    let rules = alerts::list_enabled(pool).await?;
    let trends = sqlx::query("SELECT youtube_id, title, COALESCE(description,''), platform, COALESCE(region,''), COALESCE(category,''), views_per_hour, COALESCE(url,'') FROM videos WHERE updated_at >= NOW() - INTERVAL '2 hours' OR published_at >= NOW() - INTERVAL '48 hours' ORDER BY views_per_hour DESC LIMIT 200").fetch_all(pool).await?;
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
            let url: String = t.get(7);
            let score = approximate_trend_score(vph);
            let rule = AlertRuleMatchInput {
                platform: r.platform.clone(),
                region: r.region.clone(),
                category: r.category.clone(),
                keyword: r.keyword.clone(),
                min_views_per_hour: r.min_views_per_hour,
                min_trend_score: r.min_trend_score,
            };
            let trend = TrendMatchInput {
                platform: platform.clone(),
                region: region.clone(),
                category: category.clone(),
                title: title.clone(),
                description: desc.clone(),
                views_per_hour: vph,
            };
            if !alert_matches_rule(&rule, &trend) {
                continue;
            }

            let status: &str;
            let mut payload = serde_json::json!({"channel":r.channel,"title":title,"platform":platform,"views_per_hour":vph});
            match r.channel.as_str() {
                "web" => { status = "logged"; let _ = notifications::create(pool, r.user_id, "Nouvelle tendance détectée", &format!("{} accélère sur {}", title, platform), "trend_alert", serde_json::json!({"trend_id": trend_id,"platform": platform,"views_per_hour": vph,"url": url})).await?; },
                "email" => {
                    if std::env::var("SMTP_HOST").unwrap_or_default().is_empty() {
                        status = "skipped";
                        payload["error_message"] = serde_json::json!("SMTP is not configured");
                    } else if email::send_email(
                        pool,
                        &config.smtp,
                        Some(r.user_id),
                        &r.user_email_or_username,
                        &format!("Alerte tendance: {}", title),
                        &format!("<h1>Nouvelle tendance détectée</h1><p><strong>{}</strong></p><p>Plateforme : {}</p><p>Vues / heure : {}</p>", title, platform, vph),
                    )
                    .await
                    .is_ok()
                    {
                        status = "sent";
                    } else {
                        status = "failed";
                        payload["error_message"] = serde_json::json!("Email send failed");
                    }
                }
                "telegram" => {
                    if !config.telegram.is_configured() {
                        status = "skipped";
                        payload["error_message"] =
                            serde_json::json!("Telegram bot token is not configured");
                    } else if let Some(chat_id) = r
                        .telegram_chat_id
                        .clone()
                        .or_else(|| config.telegram.fallback_chat_id().map(|v| v.to_string()))
                    {
                        payload["telegram_chat_id"] = serde_json::json!(mask_chat_id(&chat_id));
                        let msg = TelegramAlertMessage {
                            chat_id,
                            title: title.clone(),
                            platform: platform.clone(),
                            region: Some(region.clone()),
                            category: Some(category.clone()),
                            views_per_hour: Some(vph),
                            trend_score: Some(score),
                            url: if url.is_empty() {
                                None
                            } else {
                                Some(url.clone())
                            },
                        };
                        if let Err(err) = send_telegram_alert(&config.telegram, msg).await {
                            status = "failed";
                            payload["error_message"] = serde_json::json!(
                                format!("{}", err).replace(&config.telegram.bot_token, "***")
                            );
                        } else {
                            status = "sent";
                        }
                    } else {
                        status = "skipped";
                        payload["error_message"] =
                            serde_json::json!("Telegram chat id is not configured");
                    }
                }
                _ => continue,
            }
            let delivered_at = if status == "sent" {
                Some(chrono::Utc::now())
            } else {
                None
            };
            let res = sqlx::query("INSERT INTO alert_deliveries (alert_rule_id,status,payload,platform,trend_id,delivered_at) VALUES ($1,$2,$3,$4,$5,$6) ON CONFLICT (alert_rule_id,platform,trend_id) DO NOTHING")
            .bind(r.id).bind(status).bind(payload).bind(&platform).bind(&trend_id).bind(delivered_at).execute(pool).await?;
            created += res.rows_affected();
        }
    }
    Ok(created)
}

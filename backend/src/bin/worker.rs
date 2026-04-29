use dotenvy::dotenv;
use tokio::time::{sleep, Duration};
use tracing::{error, info, warn};
use youtube_tiktok_backend::{
    config::AppConfig,
    error::AppError,
    repositories::videos,
    services::{alerts, analytics, cache, queue, reports, scoring, youtube},
    state::AppState,
};

#[tokio::main]
async fn main() -> Result<(), AppError> {
    dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let config = AppConfig::from_env()?;
    let state = AppState::from_config(config).await?;
    let interval = state.config.scan.interval_minutes.max(1);

    info!(
        interval,
        regions = ?state.config.youtube.regions,
        themes = ?state.config.youtube.themes,
        "worker startup"
    );

    loop {
        let mut redis = state
            .redis
            .get_multiplexed_async_connection()
            .await
            .map_err(|_| AppError::Internal)?;

        let lock_key = "scan_lock:youtube";
        let lock_set: Option<String> = redis::cmd("SET")
            .arg(lock_key)
            .arg("1")
            .arg("NX")
            .arg("EX")
            .arg(60 * interval)
            .query_async(&mut redis)
            .await
            .map_err(|_| AppError::Internal)?;

        if lock_set.is_none() {
            warn!("scan skipped because lock already exists");
            sleep(Duration::from_secs(interval * 60)).await;
            continue;
        }

        if let Err(err) = queue::publish_scan_tick(&state.nats).await {
            error!(?err, "nats publish failed");
        }
        if let Err(err) = analytics::ensure_schema(&state.clickhouse).await {
            error!(?err, "clickhouse schema check failed");
        }

        let mut scanned = 0_u64;
        for region in &state.config.youtube.regions {
            for theme in &state.config.youtube.themes {
                match youtube::scan_theme_region(
                    &state.http,
                    &state.config.youtube.api_key,
                    region,
                    theme,
                )
                .await
                {
                    Ok(items) => {
                        for item in items {
                            let age_hours = ((chrono::Utc::now() - item.published_at)
                                .num_minutes()
                                .max(1) as f64)
                                / 60.0;
                            let trend_score =
                                scoring::trend_score(item.views_per_hour as f64, age_hours);
                            let cache_key =
                                format!("radar:{}:{}:{}", region, theme, item.youtube_id);
                            let payload = serde_json::json!({
                                "youtube_id": item.youtube_id,
                                "region": region,
                                "category": theme,
                                "views_per_hour": item.views_per_hour,
                                "freshness_score": (72.0 - age_hours).max(0.0),
                                "velocity_score": item.views_per_hour as f64 / 1000.0,
                                "trend_score": trend_score,
                                "opportunity_score": (trend_score * 0.9).min(100.0)
                            });

                            let (video_id, _) = videos::upsert_video(&state.pool, &item).await?;
                            videos::insert_video_stat(
                                &state.pool,
                                video_id,
                                &item.platform,
                                item.views_per_hour,
                            )
                            .await?;
                            let _ = cache::set_json(
                                &state.redis,
                                &cache_key,
                                &payload.to_string(),
                                900,
                            )
                            .await;
                            let _ = queue::publish_scan_tick(&state.nats).await;
                            scanned += 1;
                        }
                    }
                    Err(err) => error!(region, theme, ?err, "youtube scan failed"),
                }
            }
        }

        if let Err(err) = reports::process_pending_reports(&state.pool).await {
            error!(?err, "pending reports processing failed");
        }
        if let Err(err) = alerts::process_alert_rules_for_recent_trends(&state.pool).await {
            error!(?err, "alert rules processing failed");
        }

        let _: () = redis::cmd("DEL")
            .arg(lock_key)
            .query_async(&mut redis)
            .await
            .map_err(|_| AppError::Internal)?;

        info!(scanned, "worker cycle completed");
        sleep(Duration::from_secs(interval * 60)).await;
    }
}

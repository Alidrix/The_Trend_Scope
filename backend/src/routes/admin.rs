use crate::{
    error::AppError,
    repositories::{admin, email_logs, notifications, reports, subscriptions},
    services::{
        access::ensure_admin,
        email, stripe,
        telegram::{send_telegram_alert, TelegramAlertMessage},
        youtube,
    },
    state::AppState,
    AuthBearer,
};
use axum::{
    extract::{Query, State},
    Json,
};
use serde::Deserialize;
use serde_json::json;
#[derive(Debug, Deserialize)]
pub struct AdminUsersQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub plan: Option<String>,
    pub role: Option<String>,
    pub search: Option<String>,
}
#[derive(Debug, Deserialize)]
pub struct TestTelegramPayload {
    pub chat_id: Option<String>,
}
#[derive(Debug, Deserialize)]
pub struct TestSmtpPayload {
    pub to: String,
}
pub async fn overview(
    auth: AuthBearer,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, AppError> {
    ensure_admin(&state.pool, &auth.sub).await?;
    Ok(Json(
        admin::overview_snapshot(&state.pool, &state.config).await?,
    ))
}
pub async fn users(
    auth: AuthBearer,
    State(state): State<AppState>,
    Query(q): Query<AdminUsersQuery>,
) -> Result<Json<serde_json::Value>, AppError> {
    ensure_admin(&state.pool, &auth.sub).await?;
    let filters = admin::AdminUserFilters {
        page: q.page.unwrap_or(1),
        page_size: q.page_size.unwrap_or(50),
        plan: q.plan,
        role: q.role,
        search: q.search,
    };
    Ok(Json(
        json!({"users":admin::list_users(&state.pool,&filters).await?}),
    ))
}
pub async fn sources(
    auth: AuthBearer,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, AppError> {
    ensure_admin(&state.pool, &auth.sub).await?;
    Ok(Json(json!({"sources":admin::source_status(&state.config)})))
}
pub async fn jobs(
    auth: AuthBearer,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, AppError> {
    ensure_admin(&state.pool, &auth.sub).await?;
    Ok(Json(json!(reports::jobs_snapshot(&state.pool).await?)))
}
pub async fn system(
    auth: AuthBearer,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, AppError> {
    ensure_admin(&state.pool, &auth.sub).await?;
    let pg = postgres_status(&state).await;
    let redis = redis_status(&state).await;
    let nats = nats_status(&state);
    let ch = if state.config.clickhouse.url.is_empty() {
        "not_configured"
    } else {
        "configured"
    };
    let s3 = s3_status(&state);
    Ok(Json(
        json!({"runtime":{"env":state.config.env,"frontend_origin":state.config.frontend_origin},"services":{"postgres":pg,"redis":redis,"nats":nats,"clickhouse":ch},"integrations":integration_statuses(&state),"storage":{"local_exports_dir":state.config.storage.local_exports_dir,"s3":s3}}),
    ))
}
pub async fn billing(
    auth: AuthBearer,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, AppError> {
    ensure_admin(&state.pool, &auth.sub).await?;
    let mut snap = subscriptions::admin_billing_snapshot(&state.pool).await?;
    snap["stripe"] = stripe_flags();
    Ok(Json(snap))
}
pub async fn go_live_checklist(
    auth: AuthBearer,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, AppError> {
    ensure_admin(&state.pool, &auth.sub).await?;
    Ok(Json(json!({ "items": go_live_items(&state).await })))
}

async fn go_live_items(state: &AppState) -> Vec<serde_json::Value> {
    vec![
        json!({"key":"youtube","label":"YouTube API key configured","status": if state.config.youtube.api_key.is_empty(){"error"}else{"ok"},"blocking":true}),
        json!({"key":"stripe","label":"Stripe configured","status": if stripe::config_from_env().is_some(){"ok"}else{"warning"},"blocking":true}),
        json!({"key":"smtp","label":"SMTP configured","status": if state.config.smtp.is_configured(){"ok"}else{"warning"},"blocking":false}),
        json!({"key":"telegram","label":"Telegram configured","status": if state.config.telegram.is_configured(){"ok"}else{"warning"},"blocking":false}),
        json!({"key":"cloudflare","label":"Cloudflare token configured","status": if std::env::var("CF_DNS_API_TOKEN").unwrap_or_default().is_empty(){"error"}else{"ok"},"blocking":true}),
        json!({"key":"traefik_dynamic","label":"Traefik dynamic.yml configured","status":"manual","blocking":true}),
        json!({"key":"database","label":"Database via PgBouncer","status":postgres_status(state).await,"blocking":true}),
        json!({"key":"redis","label":"Redis configured","status":redis_status(state).await,"blocking":true}),
        json!({"key":"nats","label":"NATS configured","status":nats_status(state),"blocking":true}),
        json!({"key":"exports","label":"Local exports directory configured","status":"ok","blocking":false}),
    ]
}

async fn postgres_status(state: &AppState) -> &'static str {
    if sqlx::query_scalar::<_, i32>("SELECT 1")
        .fetch_one(&state.pool)
        .await
        .is_ok()
    {
        "ok"
    } else {
        "error"
    }
}

async fn redis_status(state: &AppState) -> &'static str {
    if state.redis.get_connection().is_ok() {
        "ok"
    } else {
        "error"
    }
}

async fn metrics_status(state: &AppState) -> &'static str {
    let users_ok = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM users")
        .fetch_one(&state.pool)
        .await
        .is_ok();
    let reports_ok = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM reports")
        .fetch_one(&state.pool)
        .await
        .is_ok();
    if users_ok && reports_ok {
        "ok"
    } else {
        "error"
    }
}

fn nats_status(state: &AppState) -> &'static str {
    if state.config.nats.nats_url.trim().is_empty() {
        "not_configured"
    } else {
        "configured"
    }
}

fn s3_status(state: &AppState) -> &'static str {
    if [
        state.config.storage.s3_endpoint.as_str(),
        state.config.storage.s3_bucket.as_str(),
        state.config.storage.s3_access_key_id.as_str(),
        state.config.storage.s3_secret_access_key.as_str(),
    ]
    .iter()
    .all(|v| !v.is_empty())
    {
        "configured"
    } else {
        "not_configured"
    }
}

fn integration_statuses(state: &AppState) -> serde_json::Value {
    json!({
        "youtube": if state.config.youtube.api_key.is_empty() { "not_configured" } else { "configured" },
        "stripe": if stripe::config_from_env().is_some() { "configured" } else { "not_configured" },
        "smtp": if state.config.smtp.is_configured() { "configured" } else { "not_configured" },
        "telegram": if state.config.telegram.is_configured() { "configured" } else { "not_configured" },
        "cloudflare": if std::env::var("CF_DNS_API_TOKEN").unwrap_or_default().is_empty() {
            "not_configured"
        } else {
            "configured"
        }
    })
}

fn stripe_flags() -> serde_json::Value {
    json!({
        "configured": stripe::config_from_env().is_some(),
        "webhook_configured": !std::env::var("STRIPE_WEBHOOK_SECRET").unwrap_or_default().is_empty(),
        "price_pro_configured": !std::env::var("STRIPE_PRICE_PRO_MONTHLY").unwrap_or_default().is_empty(),
        "price_studio_configured": !std::env::var("STRIPE_PRICE_STUDIO_MONTHLY").unwrap_or_default().is_empty()
    })
}
// existing misc
pub async fn email_logs_list(
    auth: AuthBearer,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, AppError> {
    ensure_admin(&state.pool, &auth.sub).await?;
    Ok(Json(
        json!({"logs": email_logs::latest(&state.pool, 50).await?}),
    ))
}
pub async fn notifications_snapshot(
    auth: AuthBearer,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, AppError> {
    ensure_admin(&state.pool, &auth.sub).await?;
    Ok(Json(notifications::admin_snapshot(&state.pool).await?))
}
pub async fn exports_list(
    auth: AuthBearer,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, AppError> {
    ensure_admin(&state.pool, &auth.sub).await?;
    let exports = reports::latest_exports(&state.pool)
        .await?
        .into_iter()
        .map(|r| json!({"id":r.id,"title":r.title,"format":r.format,"file_url":r.file_url,"created_at":r.created_at}))
        .collect::<Vec<_>>();
    Ok(Json(json!({"exports":exports})))
}
pub async fn test_telegram(
    auth: AuthBearer,
    State(state): State<AppState>,
    Json(payload): Json<TestTelegramPayload>,
) -> Result<Json<serde_json::Value>, AppError> {
    ensure_admin(&state.pool, &auth.sub).await?;
    if !state.config.telegram.is_configured() {
        return Ok(Json(json!({"sent":false,"reason":"not_configured"})));
    };
    let chat_id = payload.chat_id.or_else(|| {
        state
            .config
            .telegram
            .fallback_chat_id()
            .map(|x| x.to_string())
    });
    let Some(chat_id) = chat_id else {
        return Ok(Json(json!({"sent":false,"reason":"chat_id_missing"})));
    };
    send_telegram_alert(
        &state.config.telegram,
        TelegramAlertMessage {
            chat_id,
            title: "Test admin ops".into(),
            platform: "youtube".into(),
            region: None,
            category: None,
            views_per_hour: Some(1234),
            trend_score: Some(1.2),
            url: None,
        },
    )
    .await?;
    Ok(Json(json!({"sent":true})))
}
pub async fn test_smtp(
    auth: AuthBearer,
    State(state): State<AppState>,
    Json(payload): Json<TestSmtpPayload>,
) -> Result<Json<serde_json::Value>, AppError> {
    ensure_admin(&state.pool, &auth.sub).await?;
    if !state.config.smtp.is_configured() {
        return Ok(Json(json!({"sent":false,"reason":"not_configured"})));
    };
    email::send_email(
        &state.pool,
        &state.config.smtp,
        None,
        &payload.to,
        "Trend Scope SMTP test",
        "<p>SMTP test admin</p>",
    )
    .await?;
    Ok(Json(json!({"sent":true})))
}
pub async fn test_youtube(
    auth: AuthBearer,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, AppError> {
    ensure_admin(&state.pool, &auth.sub).await?;
    if state.config.youtube.api_key.is_empty() {
        return Ok(Json(
            json!({"ok":false,"reason":"not_configured","message":"YouTube API key is not configured"}),
        ));
    };
    match youtube::validate_api_key(&state.http, &state.config.youtube).await {
        Ok(_) => Ok(Json(
            json!({"ok":true,"message":"YouTube API key accepted"}),
        )),
        Err(_) => Ok(Json(
            json!({"ok":false,"reason":"youtube_api_error","message":"youtube api validation failed"}),
        )),
    }
}
pub async fn test_stripe(
    auth: AuthBearer,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, AppError> {
    ensure_admin(&state.pool, &auth.sub).await?;
    let configured = json!({"secret_key":!std::env::var("STRIPE_SECRET_KEY").unwrap_or_default().is_empty(),"webhook_secret":!std::env::var("STRIPE_WEBHOOK_SECRET").unwrap_or_default().is_empty(),"pro_price":!std::env::var("STRIPE_PRICE_PRO_MONTHLY").unwrap_or_default().is_empty(),"studio_price":!std::env::var("STRIPE_PRICE_STUDIO_MONTHLY").unwrap_or_default().is_empty()});
    let ok = configured["secret_key"].as_bool() == Some(true)
        && configured["webhook_secret"].as_bool() == Some(true)
        && configured["pro_price"].as_bool() == Some(true)
        && configured["studio_price"].as_bool() == Some(true);
    if ok {
        Ok(Json(json!({"ok":true,"configured":configured})))
    } else {
        Ok(Json(
            json!({"ok":false,"configured":configured,"reason":"missing_required_stripe_config"}),
        ))
    }
}

pub async fn smoke(
    auth: AuthBearer,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, AppError> {
    ensure_admin(&state.pool, &auth.sub).await?;
    let storage = if state.config.storage.local_exports_dir.trim().is_empty() {
        "not_configured"
    } else if tokio::fs::create_dir_all(&state.config.storage.local_exports_dir)
        .await
        .is_ok()
    {
        "ok"
    } else {
        "error"
    };

    let checks = json!({
        "postgres": postgres_status(&state).await,
        "redis": redis_status(&state).await,
        "nats": nats_status(&state),
        "metrics": metrics_status(&state).await,
        "youtube_config": if state.config.youtube.api_key.is_empty() { "not_configured" } else { "configured" },
        "stripe_config": if stripe::config_from_env().is_some() { "configured" } else { "not_configured" },
        "smtp_config": if state.config.smtp.is_configured() { "configured" } else { "not_configured" },
        "telegram_config": if state.config.telegram.is_configured() { "configured" } else { "not_configured" },
        "storage": storage
    });
    let blocking = json!({
        "postgres": true,
        "redis": true,
        "nats": true,
        "youtube_config": true,
        "stripe_config": true,
        "storage": true,
        "smtp_config": false,
        "telegram_config": false,
        "metrics": false
    });

    let mut ok = true;
    let acceptable = ["ok", "configured"];
    for key in [
        "postgres",
        "redis",
        "nats",
        "youtube_config",
        "stripe_config",
        "storage",
    ] {
        let status = checks.get(key).and_then(|v| v.as_str()).unwrap_or("error");
        if !acceptable.contains(&status) {
            ok = false;
        }
    }

    Ok(Json(json!({"ok":ok,"checks":checks,"blocking":blocking})))
}

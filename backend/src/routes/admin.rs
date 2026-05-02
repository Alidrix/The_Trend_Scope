use crate::{
    error::AppError,
    repositories::{admin, admin_audit_logs, email_logs, notifications, reports, subscriptions},
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
    http::{header, HeaderMap},
    Json,
};
use serde::Deserialize;
use serde_json::{json, Value};
use std::time::SystemTime;
use tokio::fs;
#[derive(Debug, Deserialize)]
pub struct AdminUsersQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub plan: Option<String>,
    pub role: Option<String>,
    pub search: Option<String>,
}
#[derive(Debug, Deserialize)]
pub struct AuditLogsQuery {
    pub limit: Option<i64>,
    pub action: Option<String>,
    pub status: Option<String>,
    pub admin_username: Option<String>,
    pub since: Option<String>,
    pub until: Option<String>,
}
#[derive(Debug, Clone, Default)]
struct AdminAuditContext {
    ip_address: Option<String>,
    user_agent: Option<String>,
}

#[derive(Debug, serde::Serialize)]
struct BackupStatus {
    directory: String,
    latest_file: Option<String>,
    latest_age_seconds: Option<u64>,
    latest_size_bytes: Option<u64>,
    checksum_file: Option<String>,
    checksum_present: bool,
    status: String,
}

#[derive(Debug, Deserialize)]
pub struct TestTelegramPayload {
    pub chat_id: Option<String>,
}
#[derive(Debug, Deserialize)]
pub struct TestSmtpPayload {
    pub to: String,
}

async fn audit_admin_action(
    state: &AppState,
    admin_username: &str,
    action: &str,
    target: Option<&str>,
    status: &str,
    context: AdminAuditContext,
    metadata: Value,
) {
    if let Err(err) = admin_audit_logs::create(
        &state.pool,
        admin_audit_logs::CreateAdminAuditLog {
            admin_username,
            action,
            target,
            status,
            ip_address: context.ip_address.as_deref(),
            user_agent: context.user_agent.as_deref(),
            metadata,
        },
    )
    .await
    {
        tracing::warn!(?err, action, "failed to write admin audit log");
    }
}

fn audit_context_from_headers(headers: &HeaderMap) -> AdminAuditContext {
    let user_agent = headers
        .get(header::USER_AGENT)
        .and_then(|value| value.to_str().ok())
        .map(|value| value.chars().take(512).collect::<String>());
    let ip_address = headers
        .get("cf-connecting-ip")
        .or_else(|| headers.get("x-real-ip"))
        .or_else(|| headers.get("x-forwarded-for"))
        .and_then(|value| value.to_str().ok())
        .map(|value| value.split(',').next().unwrap_or(value).trim().to_string())
        .filter(|value| !value.is_empty())
        .map(|value| value.chars().take(128).collect::<String>());
    AdminAuditContext {
        ip_address,
        user_agent,
    }
}

fn parse_rfc3339_utc(value: Option<String>) -> Option<chrono::DateTime<chrono::Utc>> {
    value
        .and_then(|raw| chrono::DateTime::parse_from_rfc3339(&raw).ok())
        .map(|dt| dt.with_timezone(&chrono::Utc))
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
// ... unchanged endpoints
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
    headers: HeaderMap,
) -> Result<Json<serde_json::Value>, AppError> {
    let audit_context = audit_context_from_headers(&headers);
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
    audit_admin_action(
        &state,
        &auth.sub,
        "system",
        None,
        "ok",
        audit_context,
        json!({"postgres": pg, "redis": redis, "nats": nats}),
    )
    .await;
    Ok(Json(
        json!({"runtime":{"env":state.config.env,"frontend_origin":state.config.frontend_origin},"services":{"postgres":pg,"redis":redis,"nats":nats,"clickhouse":ch},"integrations":integration_statuses(&state),"storage":{"local_exports_dir":state.config.storage.local_exports_dir,"s3":s3}}),
    ))
}

pub async fn billing(
    auth: AuthBearer,
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<serde_json::Value>, AppError> {
    let audit_context = audit_context_from_headers(&headers);
    ensure_admin(&state.pool, &auth.sub).await?;
    let mut snap = subscriptions::admin_billing_snapshot(&state.pool).await?;
    snap["stripe"] = stripe_flags();
    audit_admin_action(
        &state,
        &auth.sub,
        "billing",
        None,
        "ok",
        audit_context,
        json!({"stripe": snap["stripe"]}),
    )
    .await;
    Ok(Json(snap))
}

pub async fn go_live_checklist(
    auth: AuthBearer,
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<serde_json::Value>, AppError> {
    let audit_context = audit_context_from_headers(&headers);
    ensure_admin(&state.pool, &auth.sub).await?;
    audit_admin_action(
        &state,
        &auth.sub,
        "go_live_checklist",
        None,
        "ok",
        audit_context,
        json!({}),
    )
    .await;
    Ok(Json(json!({ "items": go_live_items(&state).await })))
}

pub async fn audit_logs(
    auth: AuthBearer,
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(q): Query<AuditLogsQuery>,
) -> Result<Json<serde_json::Value>, AppError> {
    let audit_context = audit_context_from_headers(&headers);
    ensure_admin(&state.pool, &auth.sub).await?;
    let filters = admin_audit_logs::AdminAuditLogFilters {
        limit: q.limit.unwrap_or(100),
        action: q.action,
        status: q.status,
        admin_username: q.admin_username,
        since: parse_rfc3339_utc(q.since),
        until: parse_rfc3339_utc(q.until),
    };
    let logs = admin_audit_logs::search(&state.pool, &filters).await?;
    audit_admin_action(
        &state,
        &auth.sub,
        "audit_logs",
        None,
        "ok",
        audit_context,
        json!({"count": logs.len()}),
    )
    .await;
    Ok(Json(json!({ "logs": logs })))
}

async fn latest_backup_status(
    directory: &str,
    prefix: &str,
    extension: &str,
    max_age_hours: u64,
) -> BackupStatus {
    let mut status = BackupStatus {
        directory: directory.to_string(),
        latest_file: None,
        latest_age_seconds: None,
        latest_size_bytes: None,
        checksum_file: None,
        checksum_present: false,
        status: "not_found".to_string(),
    };
    let mut entries = match fs::read_dir(directory).await {
        Ok(entries) => entries,
        Err(_) => {
            status.status = "error".to_string();
            return status;
        }
    };
    let mut latest_name: Option<String> = None;
    while let Ok(Some(entry)) = entries.next_entry().await {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let Some(name) = path.file_name().and_then(|value| value.to_str()) else {
            continue;
        };
        if name.starts_with(prefix)
            && name.ends_with(extension)
            && latest_name
                .as_ref()
                .map(|v| name > v.as_str())
                .unwrap_or(true)
        {
            latest_name = Some(name.to_string());
        }
    }
    let Some(latest_file) = latest_name else {
        return status;
    };
    status.latest_file = Some(latest_file.clone());
    let file_path = format!("{directory}/{latest_file}");
    if let Ok(metadata) = fs::metadata(&file_path).await {
        status.latest_size_bytes = Some(metadata.len());
        if let Ok(modified) = metadata.modified() {
            if let Ok(age) = SystemTime::now().duration_since(modified) {
                status.latest_age_seconds = Some(age.as_secs());
            }
        }
    }
    let checksum_file = format!("{latest_file}.sha256");
    status.checksum_present = fs::metadata(format!("{directory}/{checksum_file}"))
        .await
        .is_ok();
    status.checksum_file = Some(checksum_file);

    let too_old = status
        .latest_age_seconds
        .map(|age| age > max_age_hours * 3600)
        .unwrap_or(true);
    status.status = if !status.checksum_present || too_old {
        "warning".to_string()
    } else {
        "ok".to_string()
    };
    status
}

pub async fn backups_status(
    auth: AuthBearer,
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<serde_json::Value>, AppError> {
    let audit_context = audit_context_from_headers(&headers);
    ensure_admin(&state.pool, &auth.sub).await?;
    let backup_dir = std::env::var("BACKUP_DIR").unwrap_or_else(|_| "backups/postgres".to_string());
    let exports_dir =
        std::env::var("EXPORTS_BACKUP_DIR").unwrap_or_else(|_| "backups/exports".to_string());
    let backup_retention_days = std::env::var("BACKUP_RETENTION_DAYS")
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(14);
    let audit_retention_days = std::env::var("AUDIT_RETENTION_DAYS")
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(90);
    let max_backup_age_hours = std::env::var("MAX_BACKUP_AGE_HOURS")
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(24);

    let postgres =
        latest_backup_status(&backup_dir, "postgres-", ".sql.gz", max_backup_age_hours).await;
    let exports =
        latest_backup_status(&exports_dir, "exports-", ".tar.gz", max_backup_age_hours).await;
    let mut warnings = Vec::new();
    if postgres.status != "ok" {
        warnings.push(format!("postgres status: {}", postgres.status));
    }
    if exports.status != "ok" {
        warnings.push(format!("exports status: {}", exports.status));
    }
    audit_admin_action(
        &state,
        &auth.sub,
        "backups_status",
        None,
        "ok",
        audit_context,
        json!({"postgres_status": postgres.status, "exports_status": exports.status}),
    )
    .await;
    Ok(Json(json!({
        "postgres": postgres,
        "exports": exports,
        "retention": {
            "backup_retention_days": backup_retention_days,
            "audit_retention_days": audit_retention_days,
            "max_backup_age_hours": max_backup_age_hours
        },
        "warnings": warnings
    })))
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
async fn local_storage_status(state: &AppState) -> &'static str {
    if state.config.storage.local_exports_dir.trim().is_empty() {
        return "not_configured";
    }
    if tokio::fs::create_dir_all(&state.config.storage.local_exports_dir)
        .await
        .is_ok()
    {
        "ok"
    } else {
        "error"
    }
}
fn smoke_blocking_map() -> serde_json::Value {
    json!({"postgres": true,"redis": true,"nats": true,"youtube_config": true,"stripe_config": true,"storage": true,"smtp_config": false,"telegram_config": false,"metrics": false})
}
fn smoke_is_ok(checks: &serde_json::Value) -> bool {
    let acceptable = ["ok", "configured"];
    [
        "postgres",
        "redis",
        "nats",
        "youtube_config",
        "stripe_config",
        "storage",
    ]
    .iter()
    .all(|key| {
        checks
            .get(*key)
            .and_then(|v| v.as_str())
            .map(|status| acceptable.contains(&status))
            .unwrap_or(false)
    })
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
    json!({"youtube": if state.config.youtube.api_key.is_empty() { "not_configured" } else { "configured" },"stripe": if stripe::config_from_env().is_some() { "configured" } else { "not_configured" },"smtp": if state.config.smtp.is_configured() { "configured" } else { "not_configured" },"telegram": if state.config.telegram.is_configured() { "configured" } else { "not_configured" },"cloudflare": if std::env::var("CF_DNS_API_TOKEN").unwrap_or_default().is_empty() {"not_configured"} else {"configured"}})
}
fn stripe_flags() -> serde_json::Value {
    json!({"configured": stripe::config_from_env().is_some(),"webhook_configured": !std::env::var("STRIPE_WEBHOOK_SECRET").unwrap_or_default().is_empty(),"price_pro_configured": !std::env::var("STRIPE_PRICE_PRO_MONTHLY").unwrap_or_default().is_empty(),"price_studio_configured": !std::env::var("STRIPE_PRICE_STUDIO_MONTHLY").unwrap_or_default().is_empty()})
}

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
    headers: HeaderMap,
) -> Result<Json<serde_json::Value>, AppError> {
    ensure_admin(&state.pool, &auth.sub).await?;
    let audit_context = audit_context_from_headers(&headers);
    let exports = reports::latest_exports(&state.pool).await?.into_iter().map(|r| json!({"id":r.id,"title":r.title,"format":r.format,"file_url":r.file_url,"created_at":r.created_at})).collect::<Vec<_>>();
    audit_admin_action(
        &state,
        &auth.sub,
        "exports_list",
        None,
        "ok",
        audit_context,
        json!({"count": exports.len()}),
    )
    .await;
    Ok(Json(json!({"exports":exports})))
}

pub async fn test_telegram(
    auth: AuthBearer,
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<TestTelegramPayload>,
) -> Result<Json<serde_json::Value>, AppError> {
    ensure_admin(&state.pool, &auth.sub).await?;
    let audit_context = audit_context_from_headers(&headers);
    if !state.config.telegram.is_configured() {
        audit_admin_action(&state, &auth.sub, "test_telegram", None, "failed", audit_context.clone(), json!({"chat_id_provided": payload.chat_id.is_some(),"configured": state.config.telegram.is_configured()})).await;
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
        audit_admin_action(
            &state,
            &auth.sub,
            "test_telegram",
            None,
            "failed",
            audit_context.clone(),
            json!({"chat_id_provided": false,"configured": true}),
        )
        .await;
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
    audit_admin_action(
        &state,
        &auth.sub,
        "test_telegram",
        None,
        "sent",
        audit_context.clone(),
        json!({"chat_id_provided": true,"configured": true}),
    )
    .await;
    Ok(Json(json!({"sent":true})))
}

pub async fn test_smtp(
    auth: AuthBearer,
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<TestSmtpPayload>,
) -> Result<Json<serde_json::Value>, AppError> {
    ensure_admin(&state.pool, &auth.sub).await?;
    let audit_context = audit_context_from_headers(&headers);
    if !state.config.smtp.is_configured() {
        audit_admin_action(
            &state,
            &auth.sub,
            "test_smtp",
            Some(&payload.to),
            "failed",
            audit_context.clone(),
            json!({"configured": false}),
        )
        .await;
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
    audit_admin_action(
        &state,
        &auth.sub,
        "test_smtp",
        Some(&payload.to),
        "sent",
        audit_context.clone(),
        json!({"configured": true}),
    )
    .await;
    Ok(Json(json!({"sent":true})))
}

pub async fn test_youtube(
    auth: AuthBearer,
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<serde_json::Value>, AppError> {
    ensure_admin(&state.pool, &auth.sub).await?;
    let audit_context = audit_context_from_headers(&headers);
    if state.config.youtube.api_key.is_empty() {
        audit_admin_action(
            &state,
            &auth.sub,
            "test_youtube",
            None,
            "failed",
            audit_context.clone(),
            json!({"configured": false}),
        )
        .await;
        return Ok(Json(
            json!({"ok":false,"reason":"not_configured","message":"YouTube API key is not configured"}),
        ));
    };
    match youtube::validate_api_key(&state.http, &state.config.youtube).await {
        Ok(_) => {
            audit_admin_action(
                &state,
                &auth.sub,
                "test_youtube",
                None,
                "ok",
                audit_context.clone(),
                json!({"configured": true}),
            )
            .await;
            Ok(Json(
                json!({"ok":true,"message":"YouTube API key accepted"}),
            ))
        }
        Err(_) => {
            audit_admin_action(
                &state,
                &auth.sub,
                "test_youtube",
                None,
                "failed",
                audit_context.clone(),
                json!({"configured": true}),
            )
            .await;
            Ok(Json(
                json!({"ok":false,"reason":"youtube_api_error","message":"youtube api validation failed"}),
            ))
        }
    }
}

pub async fn test_stripe(
    auth: AuthBearer,
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<serde_json::Value>, AppError> {
    ensure_admin(&state.pool, &auth.sub).await?;
    let audit_context = audit_context_from_headers(&headers);
    let configured = json!({"secret_key":!std::env::var("STRIPE_SECRET_KEY").unwrap_or_default().is_empty(),"webhook_secret":!std::env::var("STRIPE_WEBHOOK_SECRET").unwrap_or_default().is_empty(),"pro_price":!std::env::var("STRIPE_PRICE_PRO_MONTHLY").unwrap_or_default().is_empty(),"studio_price":!std::env::var("STRIPE_PRICE_STUDIO_MONTHLY").unwrap_or_default().is_empty()});
    let ok = configured["secret_key"].as_bool() == Some(true)
        && configured["webhook_secret"].as_bool() == Some(true)
        && configured["pro_price"].as_bool() == Some(true)
        && configured["studio_price"].as_bool() == Some(true);
    audit_admin_action(&state, &auth.sub, "test_stripe", None, if ok {"ok"} else {"failed"}, audit_context.clone(), json!({"secret_key_configured": configured["secret_key"],"webhook_configured": configured["webhook_secret"],"pro_price_configured": configured["pro_price"],"studio_price_configured": configured["studio_price"]})).await;
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
    headers: HeaderMap,
) -> Result<Json<serde_json::Value>, AppError> {
    ensure_admin(&state.pool, &auth.sub).await?;
    let audit_context = audit_context_from_headers(&headers);
    let checks = json!({"postgres": postgres_status(&state).await,"redis": redis_status(&state).await,"nats": nats_status(&state),"metrics": metrics_status(&state).await,"youtube_config": if state.config.youtube.api_key.is_empty() { "not_configured" } else { "configured" },"stripe_config": if stripe::config_from_env().is_some() { "configured" } else { "not_configured" },"smtp_config": if state.config.smtp.is_configured() { "configured" } else { "not_configured" },"telegram_config": if state.config.telegram.is_configured() { "configured" } else { "not_configured" },"storage": local_storage_status(&state).await});
    let blocking = smoke_blocking_map();
    let ok = smoke_is_ok(&checks);
    let blocking_failed = [
        "postgres",
        "redis",
        "nats",
        "youtube_config",
        "stripe_config",
        "storage",
    ]
    .iter()
    .filter(|k| {
        checks
            .get(**k)
            .and_then(|v| v.as_str())
            .map(|x| !["ok", "configured"].contains(&x))
            .unwrap_or(true)
    })
    .count();
    audit_admin_action(
        &state,
        &auth.sub,
        "smoke",
        None,
        if ok { "ok" } else { "failed" },
        audit_context,
        json!({"ok": ok, "blocking_failed": blocking_failed}),
    )
    .await;
    Ok(Json(json!({"ok":ok,"checks":checks,"blocking":blocking})))
}

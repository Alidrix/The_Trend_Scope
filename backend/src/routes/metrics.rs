use axum::{extract::State, response::IntoResponse};
use crate::state::AppState;

async fn count_or_zero(state: &AppState, sql: &str) -> i64 {
    sqlx::query_scalar::<_, i64>(sql)
        .fetch_one(&state.pool)
        .await
        .unwrap_or_else(|e| {
            tracing::warn!(error = %e, "metrics query failed");
            0
        })
}

pub async fn metrics(State(state): State<AppState>) -> impl IntoResponse {
    let pending = count_or_zero(&state, "SELECT COUNT(*) FROM reports WHERE status='pending'").await;
    let completed = count_or_zero(&state, "SELECT COUNT(*) FROM reports WHERE status='completed'").await;
    let failed = count_or_zero(&state, "SELECT COUNT(*) FROM reports WHERE status='failed'").await;
    let ads_sent = count_or_zero(&state, "SELECT COUNT(*) FROM alert_deliveries WHERE status='sent'").await;
    let ads_failed = count_or_zero(&state, "SELECT COUNT(*) FROM alert_deliveries WHERE status='failed'").await;
    let ads_skipped = count_or_zero(&state, "SELECT COUNT(*) FROM alert_deliveries WHERE status IN ('skipped','logged')").await;
    let notif_unread = count_or_zero(&state, "SELECT COUNT(*) FROM notifications WHERE read_at IS NULL").await;
    let mail_sent = count_or_zero(&state, "SELECT COUNT(*) FROM email_logs WHERE status='sent'").await;
    let mail_failed = count_or_zero(&state, "SELECT COUNT(*) FROM email_logs WHERE status='failed'").await;
    let mail_skipped = count_or_zero(&state, "SELECT COUNT(*) FROM email_logs WHERE status='skipped'").await;
    let body = format!("trend_scope_reports_pending {pending}\ntrend_scope_reports_completed_total {completed}\ntrend_scope_reports_failed_total {failed}\ntrend_scope_alert_deliveries_sent_total {ads_sent}\ntrend_scope_alert_deliveries_failed_total {ads_failed}\ntrend_scope_alert_deliveries_skipped_total {ads_skipped}\ntrend_scope_notifications_unread_total {notif_unread}\ntrend_scope_email_logs_sent_total {mail_sent}\ntrend_scope_email_logs_failed_total {mail_failed}\ntrend_scope_email_logs_skipped_total {mail_skipped}\n");
    ([("content-type", "text/plain; version=0.0.4")], body)
}

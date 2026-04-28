use crate::error::AppError;

pub async fn send_email(to: &str, subject: &str, html: &str) -> Result<(), AppError> {
    let smtp_host = std::env::var("SMTP_HOST").unwrap_or_default();
    if smtp_host.is_empty() {
        tracing::warn!("SMTP is not configured; skipping email to={to} subject={subject}");
        return Ok(());
    }

    // MVP fallback: keep app stable even without a complete SMTP transport integration.
    tracing::info!(
        "Pretend-sending email via {smtp_host} to={to} subject={subject} body_len={}",
        html.len()
    );
    Ok(())
}

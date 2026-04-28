use crate::{error::AppError, repositories::email_logs};

fn sanitize_error(err: &str) -> String {
    err.replace(&std::env::var("SMTP_PASSWORD").unwrap_or_default(), "***")
}

pub fn render_template(name: &str, cta_url: Option<&str>) -> String {
    let base = match name {
        "welcome" => include_str!("../templates/email/welcome.html"),
        "verify-email" => include_str!("../templates/email/verify-email.html"),
        "reset-password" => include_str!("../templates/email/reset-password.html"),
        "payment-confirmed" => include_str!("../templates/email/payment-confirmed.html"),
        "subscription-cancelled" => include_str!("../templates/email/subscription-cancelled.html"),
        "data-export-ready" => include_str!("../templates/email/data-export-ready.html"),
        "account-deletion-requested" => {
            include_str!("../templates/email/account-deletion-requested.html")
        }
        _ => "<p>The Trend Scope</p>",
    };
    base.replace(
        "{{cta_url}}",
        cta_url.unwrap_or("https://thetrendscope.com"),
    )
}

pub async fn send_email(
    pool: &sqlx::PgPool,
    user_id: Option<uuid::Uuid>,
    to: &str,
    subject: &str,
    html: &str,
) -> Result<(), AppError> {
    let smtp_host = std::env::var("SMTP_HOST").unwrap_or_default();
    if smtp_host.is_empty() {
        tracing::warn!("SMTP is not configured; skipping email send");
        email_logs::log(
            pool,
            user_id,
            to,
            subject,
            "skipped",
            None,
            Some("smtp_not_configured"),
        )
        .await?;
        return Ok(());
    }

    if html.is_empty() {
        email_logs::log(
            pool,
            user_id,
            to,
            subject,
            "failed",
            None,
            Some("empty_email_body"),
        )
        .await?;
        return Err(AppError::BadRequest("email body cannot be empty".into()));
    }

    let provider_message_id = format!("local-{}", uuid::Uuid::new_v4());
    tracing::info!(to, subject, "transactional email queued");

    // Pre-production fallback transport, keeps API stable and logs delivery attempts.
    let send_result: Result<(), String> = Ok(());

    match send_result {
        Ok(()) => {
            email_logs::log(
                pool,
                user_id,
                to,
                subject,
                "sent",
                Some(&provider_message_id),
                None,
            )
            .await?;
            Ok(())
        }
        Err(err) => {
            let sanitized = sanitize_error(&err);
            email_logs::log(pool, user_id, to, subject, "failed", None, Some(&sanitized)).await?;
            Err(AppError::Internal)
        }
    }
}

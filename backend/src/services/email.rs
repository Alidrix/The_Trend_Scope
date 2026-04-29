use crate::{config::SmtpConfig, error::AppError, repositories::email_logs};
use lettre::{
    message::header::ContentType, transport::smtp::authentication::Credentials, AsyncSmtpTransport,
    AsyncTransport, Message, Tokio1Executor,
};

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
    smtp: &SmtpConfig,
    user_id: Option<uuid::Uuid>,
    to: &str,
    subject: &str,
    html: &str,
) -> Result<(), AppError> {
    if !smtp.is_configured() {
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
    let from = smtp
        .from
        .parse()
        .map_err(|_| AppError::Config("SMTP_FROM invalid".into()))?;
    let to_mail = to
        .parse()
        .map_err(|_| AppError::BadRequest("invalid email".into()))?;
    let msg = Message::builder()
        .from(from)
        .to(to_mail)
        .subject(subject)
        .header(ContentType::TEXT_HTML)
        .body(html.to_string())
        .map_err(|_| AppError::Internal)?;
    let creds = Credentials::new(smtp.username.clone(), smtp.password.clone());
    let builder = if smtp.tls {
        AsyncSmtpTransport::<Tokio1Executor>::relay(&smtp.host).map_err(|_| AppError::Internal)?
    } else {
        AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(&smtp.host)
    };
    let mailer = builder.port(smtp.port).credentials(creds).build();
    match mailer.send(msg).await {
        Ok(r) => {
            let mid = format!("{:?}", r.message_id());
            email_logs::log(pool, user_id, to, subject, "sent", Some(&mid), None).await?;
            Ok(())
        }
        Err(e) => {
            let se = sanitize_error(&e.to_string());
            email_logs::log(pool, user_id, to, subject, "failed", None, Some(&se)).await?;
            Err(AppError::Internal)
        }
    }
}

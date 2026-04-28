use std::collections::HashMap;

use hmac::{Hmac, Mac};
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use sha2::Sha256;

use crate::error::AppError;

type HmacSha256 = Hmac<Sha256>;

#[derive(Debug, Clone)]
pub struct StripeConfig {
    pub secret_key: String,
    pub webhook_secret: String,
    pub pro_price_id: String,
    pub studio_price_id: String,
    pub frontend_origin: String,
}

#[derive(Debug, Serialize)]
pub struct BillingMessage {
    pub enabled: bool,
    pub message: &'static str,
    pub checkout_url: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct StripeSessionResponse {
    pub id: String,
    pub url: Option<String>,
    pub customer: Option<String>,
    pub subscription: Option<String>,
}

pub fn billing_enabled() -> bool {
    config_from_env().is_some()
}

pub fn config_from_env() -> Option<StripeConfig> {
    let secret_key = std::env::var("STRIPE_SECRET_KEY").ok()?;
    let pro_price_id = std::env::var("STRIPE_PRICE_PRO_MONTHLY").ok()?;
    let studio_price_id = std::env::var("STRIPE_PRICE_STUDIO_MONTHLY").ok()?;
    if secret_key.is_empty() || pro_price_id.is_empty() || studio_price_id.is_empty() {
        return None;
    }

    Some(StripeConfig {
        secret_key,
        webhook_secret: std::env::var("STRIPE_WEBHOOK_SECRET").unwrap_or_default(),
        pro_price_id,
        studio_price_id,
        frontend_origin: std::env::var("FRONTEND_ORIGIN")
            .unwrap_or_else(|_| "http://localhost:5173".to_string()),
    })
}

pub fn not_configured() -> BillingMessage {
    BillingMessage {
        enabled: false,
        message: "billing is not configured yet",
        checkout_url: None,
        url: None,
    }
}

pub async fn create_customer(
    http: &reqwest::Client,
    secret_key: &str,
    email: &str,
) -> Result<String, AppError> {
    let mut form = HashMap::new();
    form.insert("email", email);

    let res = http
        .post("https://api.stripe.com/v1/customers")
        .header(AUTHORIZATION, format!("Bearer {secret_key}"))
        .form(&form)
        .send()
        .await?;

    if !res.status().is_success() {
        return Err(AppError::BadRequest(
            "stripe customer creation failed".into(),
        ));
    }

    #[derive(Deserialize)]
    struct CreatedCustomer {
        id: String,
    }

    let created = res.json::<CreatedCustomer>().await?;
    Ok(created.id)
}

pub async fn create_checkout_session(
    http: &reqwest::Client,
    cfg: &StripeConfig,
    customer_id: &str,
    price_id: &str,
) -> Result<StripeSessionResponse, AppError> {
    let params = [
        ("customer", customer_id.to_string()),
        ("mode", "subscription".to_string()),
        ("line_items[0][price]", price_id.to_string()),
        ("line_items[0][quantity]", "1".to_string()),
        (
            "success_url",
            format!(
                "{}/checkout/success?session_id={{CHECKOUT_SESSION_ID}}",
                cfg.frontend_origin
            ),
        ),
        (
            "cancel_url",
            format!("{}/checkout/cancel", cfg.frontend_origin),
        ),
    ];

    let res = http
        .post("https://api.stripe.com/v1/checkout/sessions")
        .header(AUTHORIZATION, format!("Bearer {}", cfg.secret_key))
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .form(&params)
        .send()
        .await?;

    if !res.status().is_success() {
        return Err(AppError::BadRequest(
            "stripe checkout session failed".into(),
        ));
    }

    res.json::<StripeSessionResponse>()
        .await
        .map_err(AppError::from)
}

pub async fn create_portal_session(
    http: &reqwest::Client,
    cfg: &StripeConfig,
    customer_id: &str,
) -> Result<String, AppError> {
    let params = [
        ("customer", customer_id.to_string()),
        (
            "return_url",
            format!("{}/subscription", cfg.frontend_origin),
        ),
    ];

    #[derive(Deserialize)]
    struct PortalResponse {
        url: String,
    }

    let res = http
        .post("https://api.stripe.com/v1/billing_portal/sessions")
        .header(AUTHORIZATION, format!("Bearer {}", cfg.secret_key))
        .form(&params)
        .send()
        .await?;

    if !res.status().is_success() {
        return Err(AppError::BadRequest("stripe portal session failed".into()));
    }

    Ok(res.json::<PortalResponse>().await?.url)
}

pub fn validate_signature(payload: &str, signature: &str, webhook_secret: &str) -> bool {
    if webhook_secret.is_empty() {
        return false;
    }

    let mut timestamp = None;
    let mut expected_v1 = None;

    for part in signature.split(',') {
        let mut iter = part.splitn(2, '=');
        let key = iter.next().unwrap_or_default();
        let value = iter.next().unwrap_or_default();
        if key == "t" {
            timestamp = Some(value);
        }
        if key == "v1" {
            expected_v1 = Some(value);
        }
    }

    let Some(ts) = timestamp else {
        return false;
    };
    let Some(sig) = expected_v1 else {
        return false;
    };

    let signed_payload = format!("{ts}.{payload}");
    let mut mac = HmacSha256::new_from_slice(webhook_secret.as_bytes()).expect("hmac key");
    mac.update(signed_payload.as_bytes());
    let computed = hex::encode(mac.finalize().into_bytes());
    computed == sig
}

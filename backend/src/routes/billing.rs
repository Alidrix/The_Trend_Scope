use axum::Json;
use serde::Serialize;

use crate::services::stripe::{not_configured, BillingMessage};

#[derive(Debug, Serialize)]
pub struct BillingStatus {
    pub provider: &'static str,
    pub enabled: bool,
    pub message: &'static str,
}

pub async fn billing_status() -> Json<BillingStatus> {
    Json(BillingStatus {
        provider: "stripe",
        enabled: false,
        message: "billing is not configured yet",
    })
}

pub async fn billing_checkout() -> Json<BillingMessage> {
    Json(not_configured())
}

pub async fn billing_portal() -> Json<BillingMessage> {
    Json(not_configured())
}

pub async fn billing_webhook() -> Json<BillingMessage> {
    Json(not_configured())
}

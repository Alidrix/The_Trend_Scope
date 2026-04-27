use axum::Json;
use serde::Serialize;

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
        message: "Stripe checkout sera activé dans la phase monétisation.",
    })
}

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SubscriptionStatus {
    pub plan: String,
    pub status: String,
}

use serde::Serialize;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct SubscriptionStatus {
    pub plan: String,
    pub status: String,
    pub stripe_customer_id: Option<String>,
    pub stripe_subscription_id: Option<String>,
    pub current_period_end: Option<chrono::DateTime<chrono::Utc>>,
    pub cancel_at_period_end: bool,
}

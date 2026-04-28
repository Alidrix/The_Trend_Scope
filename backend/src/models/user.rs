use serde::Serialize;

use crate::models::plan::PlanTier;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct CurrentUser {
    pub id: uuid::Uuid,
    pub email: Option<String>,
    pub username: String,
    pub display_name: Option<String>,
    pub role: String,
    pub plan: PlanTier,
    pub country: Option<String>,
    pub profile_type: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

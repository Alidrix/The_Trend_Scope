use serde::Serialize;
use sqlx::Type;

#[derive(Debug, Clone, Copy, Serialize, Type, PartialEq, Eq)]
#[sqlx(type_name = "plan_tier", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum PlanTier {
    Free,
    Pro,
    Studio,
}

#[derive(Debug, Clone, Serialize)]
pub struct PlanLimits {
    pub tier: PlanTier,
    pub daily_trend_limit: Option<i32>,
    pub history_days: i32,
    pub exports_enabled: bool,
    pub alerts_enabled: bool,
    pub reports_enabled: bool,
}

impl PlanLimits {
    pub fn from_tier(tier: PlanTier) -> Self {
        match tier {
            PlanTier::Free => Self {
                tier,
                daily_trend_limit: Some(3),
                history_days: 0,
                exports_enabled: false,
                alerts_enabled: false,
                reports_enabled: false,
            },
            PlanTier::Pro => Self {
                tier,
                daily_trend_limit: None,
                history_days: 7,
                exports_enabled: false,
                alerts_enabled: false,
                reports_enabled: false,
            },
            PlanTier::Studio => Self {
                tier,
                daily_trend_limit: None,
                history_days: 90,
                exports_enabled: true,
                alerts_enabled: true,
                reports_enabled: true,
            },
        }
    }
}

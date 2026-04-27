use axum::Json;
use serde::Serialize;

use crate::models::plan::{PlanLimits, PlanTier};

#[derive(Debug, Serialize)]
pub struct PricingPlan {
    pub tier: PlanTier,
    pub price_eur_monthly: i32,
    pub recommended: bool,
    pub limits: PlanLimits,
}

pub async fn list_plans() -> Json<Vec<PricingPlan>> {
    Json(vec![
        PricingPlan {
            tier: PlanTier::Free,
            price_eur_monthly: 0,
            recommended: false,
            limits: PlanLimits::from_tier(PlanTier::Free),
        },
        PricingPlan {
            tier: PlanTier::Pro,
            price_eur_monthly: 10,
            recommended: true,
            limits: PlanLimits::from_tier(PlanTier::Pro),
        },
        PricingPlan {
            tier: PlanTier::Studio,
            price_eur_monthly: 18,
            recommended: false,
            limits: PlanLimits::from_tier(PlanTier::Studio),
        },
    ])
}

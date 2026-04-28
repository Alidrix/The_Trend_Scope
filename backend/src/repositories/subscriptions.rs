use crate::{error::AppError, models::subscription::SubscriptionStatus};
use sqlx::PgPool;

pub async fn current_plan(pool: &PgPool, username: &str) -> Result<Option<String>, AppError> {
    sqlx::query_scalar("SELECT plan::text FROM users WHERE username = $1")
        .bind(username)
        .fetch_optional(pool)
        .await
        .map_err(AppError::from)
}

pub async fn current_status(
    pool: &PgPool,
    user_id: uuid::Uuid,
) -> Result<Option<SubscriptionStatus>, AppError> {
    sqlx::query_as::<_, SubscriptionStatus>(
        "SELECT plan::text as plan, status, stripe_customer_id, stripe_subscription_id, current_period_end, cancel_at_period_end\n         FROM subscriptions WHERE user_id = $1 ORDER BY updated_at DESC NULLS LAST, created_at DESC LIMIT 1",
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await
    .map_err(AppError::from)
}

pub async fn ensure_customer_id(
    pool: &PgPool,
    user_id: uuid::Uuid,
    customer_id: &str,
) -> Result<(), AppError> {
    sqlx::query(
        "INSERT INTO subscriptions (user_id, plan, status, stripe_customer_id) VALUES ($1, 'free', 'inactive', $2)\n         ON CONFLICT (user_id) DO UPDATE SET stripe_customer_id = EXCLUDED.stripe_customer_id, updated_at = NOW()",
    )
    .bind(user_id)
    .bind(customer_id)
    .execute(pool)
    .await?;
    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub async fn upsert_from_webhook(
    pool: &PgPool,
    user_id: uuid::Uuid,
    plan: &str,
    status: &str,
    customer_id: Option<&str>,
    subscription_id: Option<&str>,
    current_period_start: Option<chrono::DateTime<chrono::Utc>>,
    current_period_end: Option<chrono::DateTime<chrono::Utc>>,
    cancel_at_period_end: bool,
) -> Result<(), AppError> {
    sqlx::query(
        "INSERT INTO subscriptions (user_id, plan, status, stripe_customer_id, stripe_subscription_id, current_period_start, current_period_end, cancel_at_period_end)\n         VALUES ($1, $2::plan_tier, $3, $4, $5, $6, $7, $8)\n         ON CONFLICT (stripe_subscription_id) DO UPDATE SET\n           plan = EXCLUDED.plan,\n           status = EXCLUDED.status,\n           stripe_customer_id = COALESCE(EXCLUDED.stripe_customer_id, subscriptions.stripe_customer_id),\n           current_period_start = EXCLUDED.current_period_start,\n           current_period_end = EXCLUDED.current_period_end,\n           cancel_at_period_end = EXCLUDED.cancel_at_period_end,\n           updated_at = NOW()",
    )
    .bind(user_id)
    .bind(plan)
    .bind(status)
    .bind(customer_id)
    .bind(subscription_id)
    .bind(current_period_start)
    .bind(current_period_end)
    .bind(cancel_at_period_end)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn find_user_by_customer_id(
    pool: &PgPool,
    customer_id: &str,
) -> Result<Option<uuid::Uuid>, AppError> {
    sqlx::query_scalar("SELECT user_id FROM subscriptions WHERE stripe_customer_id = $1 ORDER BY updated_at DESC NULLS LAST LIMIT 1")
        .bind(customer_id)
        .fetch_optional(pool)
        .await
        .map_err(AppError::from)
}

use axum::{extract::State, http::HeaderMap, Json};
use serde::{Deserialize, Serialize};

use crate::{
    error::AppError,
    repositories::{stripe_events, subscriptions, users},
    services::{
        rate_limit,
        stripe::{self, billing_enabled, not_configured, BillingMessage},
    },
    state::AppState,
    AuthBearer,
};

#[derive(Debug, Serialize)]
pub struct BillingStatus {
    pub provider: &'static str,
    pub enabled: bool,
    pub plan: Option<String>,
    pub subscription_status: Option<String>,
    pub current_period_end: Option<chrono::DateTime<chrono::Utc>>,
    pub cancel_at_period_end: Option<bool>,
    pub message: Option<&'static str>,
}

#[derive(Debug, Deserialize)]
pub struct CheckoutPayload {
    pub plan: String,
}

pub async fn billing_status(
    auth: Option<AuthBearer>,
    State(state): State<AppState>,
) -> Result<Json<BillingStatus>, AppError> {
    if !billing_enabled() {
        return Ok(Json(BillingStatus {
            provider: "stripe",
            enabled: false,
            plan: None,
            subscription_status: None,
            current_period_end: None,
            cancel_at_period_end: None,
            message: Some("billing is not configured yet"),
        }));
    }

    let Some(auth) = auth else {
        return Ok(Json(BillingStatus {
            provider: "stripe",
            enabled: true,
            plan: Some("free".into()),
            subscription_status: Some("inactive".into()),
            current_period_end: None,
            cancel_at_period_end: Some(false),
            message: None,
        }));
    };

    let user_id = users::find_user_id_by_username(&state.pool, &auth.sub).await?;
    let plan = subscriptions::current_plan(&state.pool, &auth.sub)
        .await?
        .unwrap_or_else(|| "free".into());
    let current = subscriptions::current_status(&state.pool, user_id).await?;

    Ok(Json(BillingStatus {
        provider: "stripe",
        enabled: true,
        plan: Some(plan),
        subscription_status: Some(
            current
                .as_ref()
                .map(|s| s.status.clone())
                .unwrap_or_else(|| "inactive".into()),
        ),
        current_period_end: current.as_ref().and_then(|s| s.current_period_end),
        cancel_at_period_end: Some(
            current
                .as_ref()
                .map(|s| s.cancel_at_period_end)
                .unwrap_or(false),
        ),
        message: None,
    }))
}

pub async fn billing_checkout(
    auth: AuthBearer,
    State(state): State<AppState>,
    Json(payload): Json<CheckoutPayload>,
) -> Result<Json<BillingMessage>, AppError> {
    let Some(cfg) = stripe::config_from_env() else {
        return Ok(Json(not_configured()));
    };

    if payload.plan == "free" || (payload.plan != "pro" && payload.plan != "studio") {
        return Err(AppError::BadRequest("invalid plan".into()));
    }

    let user_id = users::find_user_id_by_username(&state.pool, &auth.sub).await?;
    let limit_key = rate_limit::key("billing:checkout", &user_id.to_string());
    if !rate_limit::check_limit(&state.redis, &limit_key, 10, 3600).await? {
        return Err(AppError::TooManyRequests);
    }

    let current = subscriptions::current_status(&state.pool, user_id).await?;
    let customer_id = if let Some(customer_id) = current.and_then(|s| s.stripe_customer_id) {
        customer_id
    } else {
        let created = stripe::create_customer(&state.http, &cfg.secret_key, &auth.sub).await?;
        subscriptions::ensure_customer_id(&state.pool, user_id, &created).await?;
        created
    };

    let price_id = if payload.plan == "pro" {
        cfg.pro_price_id.as_str()
    } else {
        cfg.studio_price_id.as_str()
    };

    let session = stripe::create_checkout_session(
        &state.http,
        &cfg,
        &customer_id,
        price_id,
        user_id,
        &payload.plan,
    )
    .await?;

    Ok(Json(BillingMessage {
        enabled: true,
        message: "checkout ready",
        checkout_url: session.url,
        url: None,
    }))
}

pub async fn billing_portal(
    auth: AuthBearer,
    State(state): State<AppState>,
) -> Result<Json<BillingMessage>, AppError> {
    let Some(cfg) = stripe::config_from_env() else {
        return Ok(Json(not_configured()));
    };

    let user_id = users::find_user_id_by_username(&state.pool, &auth.sub).await?;
    let current = subscriptions::current_status(&state.pool, user_id).await?;
    let Some(customer_id) = current.and_then(|s| s.stripe_customer_id) else {
        return Err(AppError::BadRequest("no stripe customer for user".into()));
    };

    let portal_url = stripe::create_portal_session(&state.http, &cfg, &customer_id).await?;
    Ok(Json(BillingMessage {
        enabled: true,
        message: "portal ready",
        checkout_url: None,
        url: Some(portal_url),
    }))
}

pub async fn billing_webhook(
    headers: HeaderMap,
    State(state): State<AppState>,
    body: String,
) -> Result<Json<BillingMessage>, AppError> {
    let Some(cfg) = stripe::config_from_env() else {
        return Ok(Json(not_configured()));
    };

    let sig = headers
        .get("stripe-signature")
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default();

    if !stripe::validate_signature(&body, sig, &cfg.webhook_secret) {
        return Err(AppError::Unauthorized);
    }

    let event: serde_json::Value = serde_json::from_str(&body)
        .map_err(|_| AppError::BadRequest("invalid stripe payload".into()))?;
    let event_id = event["id"].as_str().unwrap_or_default();
    let event_type = event["type"].as_str().unwrap_or_default();
    let data_obj = &event["data"]["object"];

    tracing::info!(event_type, "stripe event received");

    if event_id.is_empty() {
        return Err(AppError::BadRequest("missing stripe event id".into()));
    }

    if stripe_events::is_processed(&state.pool, event_id).await? {
        tracing::info!(event_id, "stripe event ignored");
        return Ok(Json(BillingMessage {
            enabled: true,
            message: "webhook already processed",
            checkout_url: None,
            url: None,
        }));
    }

    match event_type {
        "checkout.session.completed" => {
            let customer_id = data_obj["customer"].as_str().unwrap_or_default();
            let subscription_id = data_obj["subscription"].as_str();
            let price_id = data_obj["display_items"]["data"][0]["price"]["id"]
                .as_str()
                .unwrap_or_default();
            let metadata_plan = data_obj["metadata"]["plan"].as_str().unwrap_or_default();
            let plan = if !metadata_plan.is_empty() {
                metadata_plan
            } else {
                stripe::detect_plan(&cfg, price_id)
            };

            if let Some(user_id) =
                subscriptions::find_user_by_customer_id(&state.pool, customer_id).await?
            {
                subscriptions::upsert_from_webhook(
                    &state.pool,
                    user_id,
                    plan,
                    "active",
                    Some(customer_id),
                    subscription_id,
                    None,
                    None,
                    false,
                )
                .await?;
                users::update_user_plan(&state.pool, user_id, plan).await?;
                tracing::info!(plan, "stripe plan updated");
            }
        }
        "customer.subscription.created"
        | "customer.subscription.updated"
        | "customer.subscription.deleted"
        | "invoice.payment_failed"
        | "invoice.payment_succeeded" => {
            let customer_id = data_obj["customer"].as_str().unwrap_or_default();
            let Some(user_id) =
                subscriptions::find_user_by_customer_id(&state.pool, customer_id).await?
            else {
                tracing::info!("stripe event ignored");
                stripe_events::mark_processed(&state.pool, event_id, event_type, &event).await?;
                return Ok(Json(BillingMessage {
                    enabled: true,
                    message: "ignored",
                    checkout_url: None,
                    url: None,
                }));
            };
            let sub_status = data_obj["status"].as_str().unwrap_or("inactive");
            let price_id = data_obj["items"]["data"][0]["price"]["id"]
                .as_str()
                .unwrap_or_default();
            let plan = stripe::detect_plan(&cfg, price_id);
            let is_active = matches!(sub_status, "active" | "trialing" | "past_due");
            let current_period_end = data_obj["current_period_end"]
                .as_i64()
                .and_then(|ts| chrono::DateTime::from_timestamp(ts, 0));
            let current_period_start = data_obj["current_period_start"]
                .as_i64()
                .and_then(|ts| chrono::DateTime::from_timestamp(ts, 0));
            let cancel_at_period_end = data_obj["cancel_at_period_end"].as_bool().unwrap_or(false);
            let final_plan = if is_active { plan } else { "free" };

            subscriptions::upsert_from_webhook(
                &state.pool,
                user_id,
                final_plan,
                sub_status,
                Some(customer_id),
                data_obj["id"].as_str(),
                current_period_start,
                current_period_end,
                cancel_at_period_end,
            )
            .await?;
            users::update_user_plan(&state.pool, user_id, final_plan).await?;
            tracing::info!(final_plan, "stripe plan updated");
        }
        _ => {
            tracing::info!(event_type, "stripe event ignored");
        }
    }

    stripe_events::mark_processed(&state.pool, event_id, event_type, &event).await?;
    tracing::info!(event_id, "stripe event processed");

    Ok(Json(BillingMessage {
        enabled: true,
        message: "webhook processed",
        checkout_url: None,
        url: None,
    }))
}

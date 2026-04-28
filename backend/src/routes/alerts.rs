use axum::{
    extract::{Path, State},
    Json,
};

use crate::{
    error::{ApiMessage, AppError},
    repositories::{
        alerts::{self, AlertRulePayload},
        users,
    },
    state::AppState,
    AuthBearer,
};

fn enforce_studio(plan: &str) -> Result<(), AppError> {
    if plan != "studio" {
        return Err(AppError::Forbidden);
    }
    Ok(())
}

pub async fn list_alerts(
    auth: AuthBearer,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, AppError> {
    let user_id = users::find_user_id_by_username(&state.pool, &auth.sub).await?;
    let plan: String = sqlx::query_scalar("SELECT plan::text FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_one(&state.pool)
        .await?;
    enforce_studio(&plan)?;
    Ok(Json(
        serde_json::json!({ "alerts": alerts::list(&state.pool, user_id).await? }),
    ))
}

pub async fn create_alert(
    auth: AuthBearer,
    State(state): State<AppState>,
    Json(payload): Json<AlertRulePayload>,
) -> Result<Json<ApiMessage>, AppError> {
    let user_id = users::find_user_id_by_username(&state.pool, &auth.sub).await?;
    let plan: String = sqlx::query_scalar("SELECT plan::text FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_one(&state.pool)
        .await?;
    enforce_studio(&plan)?;
    alerts::create(&state.pool, user_id, &payload).await?;
    Ok(Json(ApiMessage {
        message: "alert created".into(),
    }))
}

pub async fn update_alert(
    auth: AuthBearer,
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
    Json(payload): Json<AlertRulePayload>,
) -> Result<Json<ApiMessage>, AppError> {
    let user_id = users::find_user_id_by_username(&state.pool, &auth.sub).await?;
    let plan: String = sqlx::query_scalar("SELECT plan::text FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_one(&state.pool)
        .await?;
    enforce_studio(&plan)?;
    alerts::update(&state.pool, user_id, id, &payload).await?;
    Ok(Json(ApiMessage {
        message: "alert updated".into(),
    }))
}

pub async fn delete_alert(
    auth: AuthBearer,
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
) -> Result<Json<ApiMessage>, AppError> {
    let user_id = users::find_user_id_by_username(&state.pool, &auth.sub).await?;
    let plan: String = sqlx::query_scalar("SELECT plan::text FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_one(&state.pool)
        .await?;
    enforce_studio(&plan)?;
    alerts::delete(&state.pool, user_id, id).await?;
    Ok(Json(ApiMessage {
        message: "alert deleted".into(),
    }))
}

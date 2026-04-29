use crate::{
    error::AppError,
    repositories::{notifications, users},
    state::AppState,
    AuthBearer,
};
use axum::{
    extract::{Path, State},
    Json,
};

pub async fn list_notifications(
    auth: AuthBearer,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, AppError> {
    let user_id = users::find_user_id_by_username(&state.pool, &auth.sub).await?;
    Ok(Json(
        serde_json::json!({"notifications":notifications::list_for_user(&state.pool,user_id).await?}),
    ))
}
pub async fn unread_notifications_count(
    auth: AuthBearer,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, AppError> {
    let user_id = users::find_user_id_by_username(&state.pool, &auth.sub).await?;
    Ok(Json(
        serde_json::json!({"count":notifications::unread_count(&state.pool,user_id).await?}),
    ))
}
pub async fn mark_notification_read(
    auth: AuthBearer,
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
) -> Result<Json<serde_json::Value>, AppError> {
    let user_id = users::find_user_id_by_username(&state.pool, &auth.sub).await?;
    notifications::mark_read(&state.pool, user_id, id).await?;
    Ok(Json(serde_json::json!({"ok":true})))
}
pub async fn mark_all_notifications_read(
    auth: AuthBearer,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, AppError> {
    let user_id = users::find_user_id_by_username(&state.pool, &auth.sub).await?;
    notifications::mark_all_read(&state.pool, user_id).await?;
    Ok(Json(serde_json::json!({"ok":true})))
}

use axum::{
    extract::{Path, State},
    Json,
};
use serde::Deserialize;

use crate::{
    error::{ApiMessage, AppError},
    repositories::{favorites, users},
    state::AppState,
    AuthBearer,
};

#[derive(Debug, Deserialize)]
pub struct FavoritePayload {
    pub platform: String,
    pub trend_id: String,
}

pub async fn list_favorites(
    auth: AuthBearer,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, AppError> {
    let user_id = users::find_user_id_by_username(&state.pool, &auth.sub).await?;
    Ok(Json(
        serde_json::json!({ "favorites": favorites::list(&state.pool, user_id).await? }),
    ))
}

pub async fn add_favorite(
    auth: AuthBearer,
    State(state): State<AppState>,
    Json(payload): Json<FavoritePayload>,
) -> Result<Json<ApiMessage>, AppError> {
    let user_id = users::find_user_id_by_username(&state.pool, &auth.sub).await?;
    favorites::create(&state.pool, user_id, &payload.platform, &payload.trend_id).await?;
    Ok(Json(ApiMessage {
        message: "favorite saved".into(),
    }))
}

pub async fn delete_favorite(
    auth: AuthBearer,
    State(state): State<AppState>,
    Path((platform, trend_id)): Path<(String, String)>,
) -> Result<Json<ApiMessage>, AppError> {
    let user_id = users::find_user_id_by_username(&state.pool, &auth.sub).await?;
    favorites::delete(&state.pool, user_id, &platform, &trend_id).await?;
    Ok(Json(ApiMessage {
        message: "favorite removed".into(),
    }))
}

use axum::{
    extract::{Path, State},
    Json,
};

use crate::{
    error::{ApiMessage, AppError},
    repositories::{
        users,
        watchlists::{self, WatchlistPayload},
    },
    state::AppState,
    AuthBearer,
};

pub async fn list_watchlists(
    auth: AuthBearer,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, AppError> {
    let user_id = users::find_user_id_by_username(&state.pool, &auth.sub).await?;
    let items = watchlists::list(&state.pool, user_id).await?;
    Ok(Json(serde_json::json!({ "watchlists": items })))
}

pub async fn create_watchlist(
    auth: AuthBearer,
    State(state): State<AppState>,
    Json(payload): Json<WatchlistPayload>,
) -> Result<Json<ApiMessage>, AppError> {
    let user_id = users::find_user_id_by_username(&state.pool, &auth.sub).await?;
    let plan: String = sqlx::query_scalar("SELECT plan::text FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_one(&state.pool)
        .await?;
    let count = watchlists::count(&state.pool, user_id).await?;
    if plan == "free" {
        return Err(AppError::Forbidden);
    }
    if plan == "pro" && count >= 3 {
        return Err(AppError::BadRequest(
            "pro plan allows up to 3 watchlists".into(),
        ));
    }
    watchlists::create(&state.pool, user_id, &payload).await?;
    Ok(Json(ApiMessage {
        message: "watchlist created".into(),
    }))
}

pub async fn update_watchlist(
    auth: AuthBearer,
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
    Json(payload): Json<WatchlistPayload>,
) -> Result<Json<ApiMessage>, AppError> {
    let user_id = users::find_user_id_by_username(&state.pool, &auth.sub).await?;
    watchlists::update(&state.pool, user_id, id, &payload).await?;
    Ok(Json(ApiMessage {
        message: "watchlist updated".into(),
    }))
}

pub async fn delete_watchlist(
    auth: AuthBearer,
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
) -> Result<Json<ApiMessage>, AppError> {
    let user_id = users::find_user_id_by_username(&state.pool, &auth.sub).await?;
    watchlists::delete(&state.pool, user_id, id).await?;
    Ok(Json(ApiMessage {
        message: "watchlist deleted".into(),
    }))
}

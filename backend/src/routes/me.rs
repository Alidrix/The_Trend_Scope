use axum::{extract::State, Json};
use serde::Deserialize;

use crate::{error::AppError, repositories::users, AuthBearer};

#[derive(Debug, Deserialize)]
pub struct MePatchPayload {
    pub display_name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SavePreferencesPayload {
    pub primary_goal: String,
    pub platforms: Vec<String>,
    pub categories: Vec<String>,
    pub regions: Vec<String>,
}

pub async fn get_me(
    auth: AuthBearer,
    State(state): State<crate::state::AppState>,
) -> Result<Json<crate::models::user::CurrentUser>, AppError> {
    Ok(Json(users::current_user(&state.pool, &auth.sub).await?))
}

pub async fn patch_me(
    auth: AuthBearer,
    State(state): State<crate::state::AppState>,
    Json(payload): Json<MePatchPayload>,
) -> Result<Json<crate::error::ApiMessage>, AppError> {
    let user_id = users::find_user_id_by_username(&state.pool, &auth.sub).await?;
    sqlx::query(
        "UPDATE user_profiles SET display_name = $2, updated_at = NOW() WHERE user_id = $1",
    )
    .bind(user_id)
    .bind(payload.display_name)
    .execute(&state.pool)
    .await?;

    Ok(Json(crate::error::ApiMessage {
        message: "profile updated".to_string(),
    }))
}

pub async fn save_preferences(
    auth: AuthBearer,
    State(state): State<crate::state::AppState>,
    Json(payload): Json<SavePreferencesPayload>,
) -> Result<Json<crate::error::ApiMessage>, AppError> {
    sqlx::query(
        "INSERT INTO user_preferences (user_id, objective, categories, platforms, regions)
         SELECT id, $2, $3, $4, $5 FROM users WHERE username = $1
         ON CONFLICT (user_id) DO UPDATE
         SET objective = EXCLUDED.objective,
             categories = EXCLUDED.categories,
             platforms = EXCLUDED.platforms,
             regions = EXCLUDED.regions,
             updated_at = NOW()",
    )
    .bind(auth.sub)
    .bind(payload.primary_goal)
    .bind(payload.categories)
    .bind(payload.platforms)
    .bind(payload.regions)
    .execute(&state.pool)
    .await?;

    Ok(Json(crate::error::ApiMessage {
        message: "preferences updated".into(),
    }))
}

pub async fn data_export_request(
    auth: AuthBearer,
    State(state): State<crate::state::AppState>,
) -> Result<Json<crate::error::ApiMessage>, AppError> {
    sqlx::query(
        "INSERT INTO data_export_requests (user_id) SELECT id FROM users WHERE username = $1",
    )
    .bind(auth.sub)
    .execute(&state.pool)
    .await?;

    Ok(Json(crate::error::ApiMessage {
        message: "data export request submitted".into(),
    }))
}

pub async fn delete_request(
    auth: AuthBearer,
    State(state): State<crate::state::AppState>,
) -> Result<Json<crate::error::ApiMessage>, AppError> {
    sqlx::query(
        "INSERT INTO account_deletion_requests (user_id) SELECT id FROM users WHERE username = $1",
    )
    .bind(auth.sub)
    .execute(&state.pool)
    .await?;

    Ok(Json(crate::error::ApiMessage {
        message: "account deletion request submitted".into(),
    }))
}

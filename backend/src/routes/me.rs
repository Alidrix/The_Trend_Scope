use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

use crate::{error::AppError, AuthBearer};

#[derive(Debug, Serialize)]
pub struct MeResponse {
    pub username: String,
}

#[derive(Debug, Deserialize)]
pub struct MePatchPayload {
    pub display_name: Option<String>,
}

pub async fn get_me(auth: AuthBearer) -> Json<MeResponse> {
    Json(MeResponse { username: auth.sub })
}

pub async fn patch_me(
    _auth: AuthBearer,
    State(_state): State<crate::state::AppState>,
    Json(_payload): Json<MePatchPayload>,
) -> Result<Json<crate::error::ApiMessage>, AppError> {
    Ok(Json(crate::error::ApiMessage {
        message: "profile updated".to_string(),
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

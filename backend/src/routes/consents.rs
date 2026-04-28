use axum::{extract::State, Json};
use serde::Deserialize;

use crate::{
    error::AppError, repositories::consents, services::privacy, state::AppState, AuthBearer,
};

#[derive(Debug, Deserialize)]
pub struct ConsentPayload {
    pub consent_type: String,
    pub granted: bool,
    pub version: String,
}

pub async fn get_consents(
    auth: AuthBearer,
    State(state): State<AppState>,
) -> Result<Json<Vec<consents::ConsentRow>>, AppError> {
    let user_id: uuid::Uuid = sqlx::query_scalar("SELECT id FROM users WHERE username = $1")
        .bind(auth.sub)
        .fetch_one(&state.pool)
        .await?;
    let rows = consents::list_consents(&state.pool, user_id).await?;
    Ok(Json(rows))
}

pub async fn post_consent(
    auth: AuthBearer,
    State(state): State<AppState>,
    Json(payload): Json<ConsentPayload>,
) -> Result<Json<crate::error::ApiMessage>, AppError> {
    let user_id: uuid::Uuid = sqlx::query_scalar("SELECT id FROM users WHERE username = $1")
        .bind(auth.sub)
        .fetch_one(&state.pool)
        .await?;

    privacy::record_consent(
        &state.pool,
        user_id,
        &payload.consent_type,
        payload.granted,
        &payload.version,
    )
    .await?;

    Ok(Json(crate::error::ApiMessage {
        message: "consent saved".into(),
    }))
}

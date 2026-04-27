use axum::{extract::State, Json};
use serde::Deserialize;

use crate::{
    error::{ApiMessage, AppError},
    state::AppState,
};

#[derive(Debug, Deserialize)]
pub struct NotePayload {
    pub video_id: uuid::Uuid,
    pub notes: String,
}

pub async fn update_note(
    State(state): State<AppState>,
    Json(payload): Json<NotePayload>,
) -> Result<Json<ApiMessage>, AppError> {
    sqlx::query("UPDATE videos SET notes = $1, updated_at = NOW() WHERE id = $2")
        .bind(&payload.notes)
        .bind(payload.video_id)
        .execute(&state.pool)
        .await?;

    Ok(Json(ApiMessage {
        message: "note updated".into(),
    }))
}

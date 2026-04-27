use axum::Json;

use crate::error::ApiMessage;

pub async fn health() -> Json<ApiMessage> {
    Json(ApiMessage {
        message: "ok".into(),
    })
}

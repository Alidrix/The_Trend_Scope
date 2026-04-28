use axum::{extract::State, Json};
use serde::Serialize;

use crate::{error::ApiMessage, state::AppState};

pub async fn health() -> Json<ApiMessage> {
    Json(ApiMessage {
        message: "ok".into(),
    })
}

#[derive(Debug, Serialize)]
pub struct ReadyResponse {
    pub status: String,
    pub checks: serde_json::Value,
}

pub async fn ready(State(state): State<AppState>) -> Json<ReadyResponse> {
    let postgres = sqlx::query_scalar::<_, i32>("SELECT 1")
        .fetch_one(&state.pool)
        .await
        .is_ok();
    let redis = state.redis.get_connection().is_ok();
    let nats = state
        .nats
        .publish("health.ready", "ping".as_bytes().to_vec().into())
        .await
        .is_ok();
    let clickhouse = state
        .clickhouse
        .query("SELECT 1")
        .fetch_one::<u8>()
        .await
        .is_ok();

    let checks = serde_json::json!({
        "postgres": if postgres {"ok"} else {"error"},
        "redis": if redis {"ok"} else {"error"},
        "nats": if nats {"ok"} else {"error"},
        "clickhouse": if clickhouse {"ok"} else {"error"}
    });
    let status = if postgres && redis && nats && clickhouse {
        "ready"
    } else {
        "degraded"
    };

    Json(ReadyResponse {
        status: status.to_string(),
        checks,
    })
}

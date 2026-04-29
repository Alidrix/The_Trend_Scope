use axum::{
    extract::{Path, State},
    Json,
};
use serde::Deserialize;

use crate::{
    error::{ApiMessage, AppError},
    repositories::{reports, users},
    state::AppState,
    AuthBearer,
};

#[derive(Debug, Deserialize)]
pub struct GenerateReportPayload {
    pub title: Option<String>,
    pub period_start: Option<chrono::NaiveDate>,
    pub period_end: Option<chrono::NaiveDate>,
    pub platforms: Option<Vec<String>>,
    pub categories: Option<Vec<String>>,
    pub format: Option<String>,
}

fn enforce_studio(plan: &str) -> Result<(), AppError> {
    if plan != "studio" {
        return Err(AppError::Forbidden);
    }
    Ok(())
}

pub async fn list_reports(
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
        serde_json::json!({"reports": reports::list(&state.pool, user_id).await?}),
    ))
}

pub async fn generate_report(
    auth: AuthBearer,
    State(state): State<AppState>,
    Json(payload): Json<GenerateReportPayload>,
) -> Result<Json<ApiMessage>, AppError> {
    let user_id = users::find_user_id_by_username(&state.pool, &auth.sub).await?;
    let plan: String = sqlx::query_scalar("SELECT plan::text FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_one(&state.pool)
        .await?;
    enforce_studio(&plan)?;
    let end = payload
        .period_end
        .unwrap_or_else(|| chrono::Utc::now().date_naive());
    let start = payload.period_start.unwrap_or(end - chrono::Days::new(7));
    let platforms = payload.platforms.unwrap_or_else(|| vec!["youtube".into()]);
    let categories = payload.categories.unwrap_or_default();
    let format = payload.format.unwrap_or_else(|| "json".into());
    if !["json", "csv", "pdf"].contains(&format.as_str()) {
        return Err(AppError::BadRequest("Unsupported report format".into()));
    }
    let new_report = reports::NewReport {
        user_id,
        title: payload
            .title
            .clone()
            .unwrap_or_else(|| "Weekly trend report".into()),
        period_start: start,
        period_end: end,
        platforms,
        categories,
        format,
    };
    reports::create(&state.pool, &new_report).await?;
    Ok(Json(ApiMessage {
        message: "report generation queued".into(),
    }))
}

pub async fn get_report(
    auth: AuthBearer,
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
) -> Result<Json<serde_json::Value>, AppError> {
    let user_id = users::find_user_id_by_username(&state.pool, &auth.sub).await?;
    let plan: String = sqlx::query_scalar("SELECT plan::text FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_one(&state.pool)
        .await?;
    enforce_studio(&plan)?;
    let report = reports::find_one(&state.pool, user_id, id).await?;
    Ok(Json(serde_json::json!({"report": report})))
}

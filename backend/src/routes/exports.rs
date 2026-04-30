use crate::{
    error::AppError,
    repositories::{reports, users},
    services::storage::sanitize_export_filename,
    state::AppState,
    AuthBearer,
};
use axum::{
    extract::{Path, State},
    response::IntoResponse,
};
use tokio::fs;

pub async fn download_export(
    auth: AuthBearer,
    State(state): State<AppState>,
    Path(filename): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let safe = sanitize_export_filename(&filename)
        .ok_or_else(|| AppError::BadRequest("invalid filename".into()))?;
    let user = users::find_access_by_username(&state.pool, &auth.sub)
        .await?
        .ok_or(AppError::Unauthorized)?;
    let can =
        reports::user_can_access_export(&state.pool, user.id, user.role == "admin", &safe).await?;
    if !can {
        return Err(AppError::Forbidden);
    }
    let p = std::path::Path::new(&state.config.storage.local_exports_dir).join(&safe);
    let bytes = fs::read(p)
        .await
        .map_err(|_| AppError::BadRequest("export file not found".into()))?;
    Ok(([("content-type", "text/csv")], bytes))
}

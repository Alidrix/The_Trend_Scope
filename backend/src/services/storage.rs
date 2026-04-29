use std::path::Path;

use tokio::fs;

use crate::{config::StorageConfig, error::AppError};

pub fn sanitize_export_filename(filename: &str) -> Option<String> {
    if filename.contains("..") || filename.contains('/') || filename.contains('\') {
        return None;
    }

    if !filename.ends_with(".csv") {
        return None;
    }

    Some(filename.to_string())
}

pub async fn store_local_export(
    config: &StorageConfig,
    filename: &str,
    content: &[u8],
) -> Result<String, AppError> {
    let safe = sanitize_export_filename(filename)
        .ok_or_else(|| AppError::BadRequest("invalid filename".into()))?;

    fs::create_dir_all(&config.local_exports_dir)
        .await
        .map_err(|_| AppError::Internal)?;

    let path = Path::new(&config.local_exports_dir).join(&safe);

    fs::write(path, content)
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(format!("/api/v1/exports/{safe}"))
}

#[cfg(test)]
mod tests {
    use super::sanitize_export_filename;

    #[test]
    fn accepts_safe_csv_filename() {
        assert_eq!(
            sanitize_export_filename("report-123.csv"),
            Some("report-123.csv".to_string())
        );
    }

    #[test]
    fn rejects_path_traversal() {
        assert_eq!(sanitize_export_filename("../secret.csv"), None);
        assert_eq!(sanitize_export_filename("nested/report.csv"), None);
        assert_eq!(sanitize_export_filename("nested\report.csv"), None);
    }

    #[test]
    fn rejects_non_csv() {
        assert_eq!(sanitize_export_filename("report.pdf"), None);
    }
}

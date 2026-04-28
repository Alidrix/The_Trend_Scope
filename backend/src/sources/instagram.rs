use crate::{error::AppError, models::source::RawTrend};

pub struct InstagramSource;

#[async_trait::async_trait]
impl super::TrendSource for InstagramSource {
    async fn scan(&self, _region: &str, _category: &str) -> Result<Vec<RawTrend>, AppError> {
        tracing::warn!("Instagram source not configured yet");
        Err(AppError::BadRequest("NotImplemented".into()))
    }

    fn platform(&self) -> &'static str {
        "instagram"
    }
}

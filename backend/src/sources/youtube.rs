use crate::{error::AppError, models::source::RawTrend};

pub struct YoutubeSource;

#[async_trait::async_trait]
impl super::TrendSource for YoutubeSource {
    async fn scan(&self, _region: &str, _category: &str) -> Result<Vec<RawTrend>, AppError> {
        Ok(vec![])
    }

    fn platform(&self) -> &'static str {
        "youtube"
    }
}

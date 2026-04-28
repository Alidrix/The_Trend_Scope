use crate::{error::AppError, models::source::RawTrend};

pub mod instagram;
pub mod tiktok;
pub mod youtube;

#[async_trait::async_trait]
pub trait TrendSource {
    async fn scan(&self, region: &str, category: &str) -> Result<Vec<RawTrend>, AppError>;
    fn platform(&self) -> &'static str;
}

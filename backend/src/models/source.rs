use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct RawTrend {
    pub trend_id: String,
    pub title: String,
    pub region: String,
    pub category: String,
    pub views_per_hour: i64,
    pub published_at: DateTime<Utc>,
}

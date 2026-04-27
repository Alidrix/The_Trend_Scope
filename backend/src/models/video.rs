use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, Row};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Video {
    pub id: uuid::Uuid,
    pub youtube_id: String,
    pub title: String,
    pub category: String,
    pub region: Option<String>,
    pub thumbnail_url: Option<String>,
    pub channel_title: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub views_per_hour: i64,
    pub duration_seconds: i32,
    pub published_at: DateTime<Utc>,
    pub is_short: bool,
    pub notes: Option<String>,
}

impl Video {
    pub fn from_row(row: PgRow) -> Self {
        let duration_seconds: i32 = row.get("duration_seconds");
        Self {
            id: row.get("id"),
            youtube_id: row.get("youtube_id"),
            title: row.get("title"),
            category: row.get("category"),
            region: row.get("region"),
            thumbnail_url: row.get("thumbnail_url"),
            channel_title: row.get("channel_title"),
            description: row.get("description"),
            url: row.get("url"),
            views_per_hour: row.get("views_per_hour"),
            duration_seconds,
            published_at: row.get("published_at"),
            is_short: duration_seconds <= 60,
            notes: row.get("notes"),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct VideoPayload {
    pub youtube_id: String,
    pub title: String,
    pub category: String,
    pub views_per_hour: i64,
    pub duration_seconds: i32,
    pub published_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct ApiVideosResponse {
    pub videos: Vec<Video>,
}

#[derive(Debug, Serialize)]
pub struct ScanResponse {
    pub message: String,
    pub inserted: usize,
    pub updated: usize,
    pub total: usize,
}

#[derive(Debug, Clone)]
pub struct NewVideo {
    pub youtube_id: String,
    pub title: String,
    pub category: String,
    pub region: String,
    pub thumbnail_url: Option<String>,
    pub channel_title: Option<String>,
    pub description: Option<String>,
    pub url: String,
    pub views_per_hour: i64,
    pub duration_seconds: i32,
    pub published_at: DateTime<Utc>,
}

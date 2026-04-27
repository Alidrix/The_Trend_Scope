use crate::{error::AppError, models::video::NewVideo};
use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::Deserialize;
use tracing::warn;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SearchResponse {
    items: Vec<SearchItem>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SearchItem {
    id: SearchId,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SearchId {
    video_id: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct VideosResponse {
    items: Vec<VideoItem>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct VideoItem {
    id: String,
    snippet: Snippet,
    content_details: Option<ContentDetails>,
    statistics: Option<Statistics>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Snippet {
    title: String,
    description: Option<String>,
    channel_title: Option<String>,
    published_at: String,
    thumbnails: Option<Thumbnails>,
}

#[derive(Debug, Deserialize)]
struct Thumbnails {
    high: Option<Thumb>,
    medium: Option<Thumb>,
    default: Option<Thumb>,
}

#[derive(Debug, Deserialize)]
struct Thumb {
    url: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ContentDetails {
    duration: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Statistics {
    view_count: Option<String>,
}

pub async fn scan_theme_region(
    client: &Client,
    api_key: &str,
    region: &str,
    theme: &str,
) -> Result<Vec<NewVideo>, AppError> {
    let published_after = (Utc::now() - chrono::Duration::days(3)).to_rfc3339();
    let search = client
        .get("https://www.googleapis.com/youtube/v3/search")
        .query(&[
            ("part", "snippet"),
            ("type", "video"),
            ("order", "date"),
            ("maxResults", "10"),
            ("regionCode", region),
            ("q", theme),
            ("publishedAfter", &published_after),
            ("key", api_key),
        ])
        .send()
        .await?
        .error_for_status()?
        .json::<SearchResponse>()
        .await?;

    let ids: Vec<String> = search
        .items
        .into_iter()
        .filter_map(|item| item.id.video_id)
        .collect();

    if ids.is_empty() {
        return Ok(vec![]);
    }

    let details = client
        .get("https://www.googleapis.com/youtube/v3/videos")
        .query(&[
            ("part", "snippet,contentDetails,statistics"),
            ("id", &ids.join(",")),
            ("maxResults", "50"),
            ("key", api_key),
        ])
        .send()
        .await?
        .error_for_status()?
        .json::<VideosResponse>()
        .await?;

    let mut out = Vec::new();
    for item in details.items {
        let Some(stats) = item.statistics else {
            continue;
        };
        let Some(view_count) = stats.view_count else {
            continue;
        };
        let Ok(total_views) = view_count.parse::<i64>() else {
            continue;
        };
        let Some(content_details) = item.content_details else {
            continue;
        };

        let published_at: DateTime<Utc> =
            match DateTime::parse_from_rfc3339(&item.snippet.published_at) {
                Ok(dt) => dt.with_timezone(&Utc),
                Err(err) => {
                    warn!(?err, "failed to parse published_at");
                    continue;
                }
            };

        let age_in_hours = (Utc::now() - published_at).num_hours().max(1);
        let views_per_hour = total_views / age_in_hours;
        let duration_seconds = parse_iso8601_duration_seconds(&content_details.duration);
        if duration_seconds <= 0 {
            continue;
        }

        let thumb = item
            .snippet
            .thumbnails
            .and_then(|t| t.high.or(t.medium).or(t.default))
            .map(|t| t.url);

        out.push(NewVideo {
            youtube_id: item.id.clone(),
            title: item.snippet.title,
            category: theme.to_string(),
            region: region.to_string(),
            thumbnail_url: thumb,
            channel_title: item.snippet.channel_title,
            description: item.snippet.description,
            url: format!("https://www.youtube.com/watch?v={}", item.id),
            views_per_hour,
            duration_seconds,
            published_at,
        });
    }

    Ok(out)
}

fn parse_iso8601_duration_seconds(duration: &str) -> i32 {
    let mut hours = 0;
    let mut minutes = 0;
    let mut seconds = 0;
    let mut current = String::new();

    for ch in duration.chars() {
        if ch.is_ascii_digit() {
            current.push(ch);
            continue;
        }

        if current.is_empty() {
            continue;
        }

        let value = current.parse::<i32>().unwrap_or(0);
        match ch {
            'H' => hours = value,
            'M' => minutes = value,
            'S' => seconds = value,
            _ => {}
        }
        current.clear();
    }

    hours * 3600 + minutes * 60 + seconds
}

#[cfg(test)]
mod tests {
    use super::parse_iso8601_duration_seconds;

    #[test]
    fn parse_duration() {
        assert_eq!(parse_iso8601_duration_seconds("PT45S"), 45);
        assert_eq!(parse_iso8601_duration_seconds("PT1M30S"), 90);
        assert_eq!(parse_iso8601_duration_seconds("PT1H2M3S"), 3723);
    }
}

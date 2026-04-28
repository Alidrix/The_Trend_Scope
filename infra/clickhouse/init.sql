CREATE TABLE IF NOT EXISTS trend_events (
    platform String,
    trend_id String,
    title String,
    category String,
    region String,
    channel_name String,
    url String,
    thumbnail_url String,
    published_at DateTime,
    collected_at DateTime,
    views UInt64,
    likes UInt64,
    comments UInt64,
    views_per_hour Float64,
    velocity_score Float64,
    trend_score Float64,
    freshness_score Float64,
    saturation_score Float64,
    opportunity_score Float64
)
ENGINE = MergeTree
PARTITION BY toYYYYMM(collected_at)
ORDER BY (platform, region, category, collected_at, trend_id);

CREATE TABLE IF NOT EXISTS trend_rankings_hourly (
    hour DateTime,
    platform String,
    region String,
    category String,
    trend_id String,
    rank UInt32,
    trend_score Float64,
    views_per_hour Float64
)
ENGINE = MergeTree
PARTITION BY toYYYYMM(hour)
ORDER BY (hour, platform, region, category, rank);

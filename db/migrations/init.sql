CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS videos (
    id UUID PRIMARY KEY,
    youtube_id TEXT UNIQUE NOT NULL,
    title TEXT NOT NULL,
    category TEXT NOT NULL,
    views_per_hour BIGINT NOT NULL DEFAULT 0,
    duration_seconds INT NOT NULL,
    published_at TIMESTAMP WITH TIME ZONE NOT NULL,
    notes TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

ALTER TABLE videos ADD COLUMN IF NOT EXISTS region TEXT;
ALTER TABLE videos ADD COLUMN IF NOT EXISTS thumbnail_url TEXT;
ALTER TABLE videos ADD COLUMN IF NOT EXISTS channel_title TEXT;
ALTER TABLE videos ADD COLUMN IF NOT EXISTS description TEXT;
ALTER TABLE videos ADD COLUMN IF NOT EXISTS url TEXT;
ALTER TABLE videos ADD COLUMN IF NOT EXISTS last_seen_at TIMESTAMP WITH TIME ZONE DEFAULT NOW();
ALTER TABLE videos ADD COLUMN IF NOT EXISTS updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW();

CREATE TABLE IF NOT EXISTS video_stats (
    id UUID PRIMARY KEY,
    video_id UUID REFERENCES videos(id) ON DELETE CASCADE,
    views_per_hour BIGINT NOT NULL,
    collected_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_video_stats_video ON video_stats(video_id);
CREATE INDEX IF NOT EXISTS idx_videos_region ON videos(region);
CREATE INDEX IF NOT EXISTS idx_videos_category ON videos(category);
CREATE INDEX IF NOT EXISTS idx_videos_views_per_hour ON videos(views_per_hour DESC);
CREATE INDEX IF NOT EXISTS idx_videos_published_at ON videos(published_at DESC);

CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY,
    username TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

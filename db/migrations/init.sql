-- Extensions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Types
DO $$ BEGIN
    CREATE TYPE plan_tier AS ENUM ('free', 'pro', 'studio');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

-- Plans
CREATE TABLE IF NOT EXISTS plans (
    tier plan_tier PRIMARY KEY,
    name TEXT NOT NULL,
    monthly_price_cents INT NOT NULL,
    daily_trend_limit INT,
    history_days INT NOT NULL,
    stats_level TEXT NOT NULL,
    exports_enabled BOOLEAN NOT NULL DEFAULT false,
    alerts_enabled BOOLEAN NOT NULL DEFAULT false,
    reports_enabled BOOLEAN NOT NULL DEFAULT false
);

INSERT INTO plans (tier, name, monthly_price_cents, daily_trend_limit, history_days, stats_level, exports_enabled, alerts_enabled, reports_enabled)
VALUES
('free', 'Free', 0, 3, 0, 'basic', false, false, false),
('pro', 'Pro', 1000, NULL, 7, 'standard', false, false, false),
('studio', 'Studio', 1800, NULL, 90, 'advanced', true, true, true)
ON CONFLICT (tier) DO NOTHING;

-- Users
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY,
    username TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    plan plan_tier NOT NULL DEFAULT 'free',
    role TEXT NOT NULL DEFAULT 'user',
    email_verified BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

ALTER TABLE users ADD COLUMN IF NOT EXISTS role TEXT NOT NULL DEFAULT 'user';
ALTER TABLE users ADD COLUMN IF NOT EXISTS plan plan_tier NOT NULL DEFAULT 'free';
ALTER TABLE users ADD COLUMN IF NOT EXISTS email_verified BOOLEAN NOT NULL DEFAULT false;

-- Profiles and preferences
CREATE TABLE IF NOT EXISTS user_profiles (
    user_id UUID PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    display_name TEXT,
    country TEXT,
    timezone TEXT,
    profile_type TEXT,
    account_type TEXT,
    company_name TEXT,
    primary_goal TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS user_preferences (
    user_id UUID PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    objective TEXT,
    categories TEXT[] NOT NULL DEFAULT '{}',
    platforms TEXT[] NOT NULL DEFAULT '{}',
    regions TEXT[] NOT NULL DEFAULT '{}',
    followed_keywords TEXT[] NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

ALTER TABLE user_profiles ADD COLUMN IF NOT EXISTS account_type TEXT;
ALTER TABLE user_profiles ADD COLUMN IF NOT EXISTS company_name TEXT;
ALTER TABLE user_profiles ADD COLUMN IF NOT EXISTS primary_goal TEXT;
ALTER TABLE user_preferences ADD COLUMN IF NOT EXISTS regions TEXT[] NOT NULL DEFAULT '{}';
ALTER TABLE user_preferences ADD COLUMN IF NOT EXISTS followed_keywords TEXT[] NOT NULL DEFAULT '{}';

-- Billing
CREATE TABLE IF NOT EXISTS subscriptions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    stripe_customer_id TEXT,
    stripe_subscription_id TEXT,
    plan plan_tier NOT NULL DEFAULT 'free',
    status TEXT NOT NULL DEFAULT 'inactive',
    current_period_start TIMESTAMPTZ,
    current_period_end TIMESTAMPTZ,
    cancel_at_period_end BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

ALTER TABLE subscriptions ADD COLUMN IF NOT EXISTS stripe_customer_id TEXT;
ALTER TABLE subscriptions ADD COLUMN IF NOT EXISTS stripe_subscription_id TEXT;
ALTER TABLE subscriptions ADD COLUMN IF NOT EXISTS current_period_start TIMESTAMPTZ;
ALTER TABLE subscriptions ADD COLUMN IF NOT EXISTS current_period_end TIMESTAMPTZ;
ALTER TABLE subscriptions ADD COLUMN IF NOT EXISTS cancel_at_period_end BOOLEAN NOT NULL DEFAULT false;
ALTER TABLE subscriptions ADD COLUMN IF NOT EXISTS updated_at TIMESTAMPTZ DEFAULT NOW();
ALTER TABLE subscriptions ALTER COLUMN status SET DEFAULT 'inactive';

CREATE TABLE IF NOT EXISTS stripe_events (
    id TEXT PRIMARY KEY,
    event_type TEXT NOT NULL,
    processed_at TIMESTAMPTZ DEFAULT NOW(),
    payload JSONB NOT NULL DEFAULT '{}'
);

-- Videos and stats
CREATE TABLE IF NOT EXISTS videos (
    id UUID PRIMARY KEY,
    youtube_id TEXT UNIQUE NOT NULL,
    title TEXT NOT NULL,
    category TEXT NOT NULL,
    views_per_hour BIGINT NOT NULL DEFAULT 0,
    duration_seconds INT NOT NULL,
    published_at TIMESTAMPTZ NOT NULL,
    notes TEXT,
    region TEXT,
    thumbnail_url TEXT,
    channel_title TEXT,
    description TEXT,
    url TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    last_seen_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS video_stats (
    id UUID PRIMARY KEY,
    video_id UUID REFERENCES videos(id) ON DELETE CASCADE,
    views_per_hour BIGINT NOT NULL,
    collected_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Usage and quotas
CREATE TABLE IF NOT EXISTS user_usage_daily (
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    usage_date DATE NOT NULL,
    trends_viewed BIGINT NOT NULL DEFAULT 0,
    PRIMARY KEY (user_id, usage_date)
);

CREATE TABLE IF NOT EXISTS trend_views (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    trend_id TEXT NOT NULL,
    platform TEXT NOT NULL,
    viewed_date DATE NOT NULL DEFAULT CURRENT_DATE,
    viewed_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_trend_views_unique_daily
ON trend_views(user_id, trend_id, platform, viewed_date);

-- Favorites
CREATE TABLE IF NOT EXISTS favorites (
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    platform TEXT NOT NULL,
    trend_id TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (user_id, platform, trend_id)
);

ALTER TABLE favorites ADD COLUMN IF NOT EXISTS platform TEXT;
ALTER TABLE favorites ADD COLUMN IF NOT EXISTS trend_id TEXT;

-- Watchlists
CREATE TABLE IF NOT EXISTS watchlists (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    keywords TEXT[] NOT NULL DEFAULT '{}',
    categories TEXT[] NOT NULL DEFAULT '{}',
    platforms TEXT[] NOT NULL DEFAULT '{}',
    regions TEXT[] NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Alerts
CREATE TABLE IF NOT EXISTS alerts (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    body TEXT NOT NULL,
    channel TEXT NOT NULL DEFAULT 'web',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS alert_rules (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    platform TEXT,
    region TEXT,
    category TEXT,
    keyword TEXT,
    min_views_per_hour BIGINT,
    min_trend_score DOUBLE PRECISION,
    channel TEXT NOT NULL DEFAULT 'web',
    enabled BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS alert_deliveries (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    alert_rule_id UUID REFERENCES alert_rules(id) ON DELETE CASCADE,
    status TEXT NOT NULL DEFAULT 'pending',
    payload JSONB NOT NULL DEFAULT '{}',
    delivered_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Reports
CREATE TABLE IF NOT EXISTS reports (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    title TEXT NOT NULL DEFAULT 'Weekly trend report',
    period_start DATE NOT NULL DEFAULT CURRENT_DATE,
    period_end DATE NOT NULL DEFAULT CURRENT_DATE,
    status TEXT NOT NULL DEFAULT 'pending',
    file_url TEXT,
    summary JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    completed_at TIMESTAMPTZ
);

ALTER TABLE reports ADD COLUMN IF NOT EXISTS title TEXT DEFAULT 'Weekly trend report';
ALTER TABLE reports ADD COLUMN IF NOT EXISTS period_start DATE DEFAULT CURRENT_DATE;
ALTER TABLE reports ADD COLUMN IF NOT EXISTS period_end DATE DEFAULT CURRENT_DATE;
ALTER TABLE reports ADD COLUMN IF NOT EXISTS status TEXT DEFAULT 'pending';
ALTER TABLE reports ADD COLUMN IF NOT EXISTS file_url TEXT;
ALTER TABLE reports ADD COLUMN IF NOT EXISTS summary JSONB NOT NULL DEFAULT '{}';
ALTER TABLE reports ADD COLUMN IF NOT EXISTS created_at TIMESTAMPTZ DEFAULT NOW();
ALTER TABLE reports ADD COLUMN IF NOT EXISTS completed_at TIMESTAMPTZ;

-- GDPR
CREATE TABLE IF NOT EXISTS consents (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    consent_type TEXT NOT NULL,
    granted BOOLEAN NOT NULL,
    version TEXT NOT NULL,
    ip_hash TEXT,
    user_agent_hash TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS audit_logs (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    action TEXT NOT NULL,
    resource TEXT,
    metadata JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS data_export_requests (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    status TEXT NOT NULL DEFAULT 'pending',
    file_url TEXT,
    requested_at TIMESTAMPTZ DEFAULT NOW(),
    completed_at TIMESTAMPTZ
);

CREATE TABLE IF NOT EXISTS account_deletion_requests (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    status TEXT NOT NULL DEFAULT 'pending',
    requested_at TIMESTAMPTZ DEFAULT NOW(),
    scheduled_deletion_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ
);

CREATE TABLE IF NOT EXISTS email_logs (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    recipient TEXT NOT NULL,
    subject TEXT NOT NULL,
    status TEXT NOT NULL,
    provider_message_id TEXT,
    error_message TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS email_verification_tokens (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token_hash TEXT NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    used_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Password reset
CREATE TABLE IF NOT EXISTS password_reset_tokens (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token_hash TEXT NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    used_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Indexes
CREATE INDEX IF NOT EXISTS idx_video_stats_video ON video_stats(video_id);
CREATE INDEX IF NOT EXISTS idx_videos_region ON videos(region);
CREATE INDEX IF NOT EXISTS idx_videos_category ON videos(category);
CREATE INDEX IF NOT EXISTS idx_videos_views_per_hour ON videos(views_per_hour DESC);
CREATE INDEX IF NOT EXISTS idx_videos_published_at ON videos(published_at DESC);
CREATE INDEX IF NOT EXISTS idx_trend_views_user_id ON trend_views(user_id);
CREATE INDEX IF NOT EXISTS idx_subscriptions_user_id ON subscriptions(user_id);
CREATE INDEX IF NOT EXISTS idx_subscriptions_stripe_customer_id ON subscriptions(stripe_customer_id);
CREATE UNIQUE INDEX IF NOT EXISTS idx_subscriptions_stripe_subscription_id
ON subscriptions(stripe_subscription_id)
WHERE stripe_subscription_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_watchlists_user_id ON watchlists(user_id);
CREATE INDEX IF NOT EXISTS idx_alert_rules_user_id ON alert_rules(user_id);
CREATE INDEX IF NOT EXISTS idx_reports_user_id ON reports(user_id);
CREATE INDEX IF NOT EXISTS idx_password_reset_tokens_hash ON password_reset_tokens(token_hash);
CREATE INDEX IF NOT EXISTS idx_consents_user_id ON consents(user_id);
CREATE INDEX IF NOT EXISTS idx_audit_logs_user_id ON audit_logs(user_id);
CREATE INDEX IF NOT EXISTS idx_email_verification_tokens_hash ON email_verification_tokens(token_hash);

ALTER TABLE videos ADD COLUMN IF NOT EXISTS platform TEXT NOT NULL DEFAULT 'youtube';
CREATE INDEX IF NOT EXISTS idx_videos_platform ON videos(platform);
CREATE UNIQUE INDEX IF NOT EXISTS idx_videos_platform_external_id ON videos(platform, youtube_id);

ALTER TABLE video_stats ADD COLUMN IF NOT EXISTS platform TEXT NOT NULL DEFAULT 'youtube';
CREATE INDEX IF NOT EXISTS idx_video_stats_platform ON video_stats(platform);

ALTER TABLE alert_deliveries ADD COLUMN IF NOT EXISTS trend_id TEXT;
ALTER TABLE alert_deliveries ADD COLUMN IF NOT EXISTS platform TEXT;
CREATE UNIQUE INDEX IF NOT EXISTS idx_alert_delivery_dedup ON alert_deliveries(alert_rule_id, platform, trend_id);

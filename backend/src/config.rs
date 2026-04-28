use crate::error::AppError;

const MIN_PASSWORD_LENGTH: usize = 10;

#[derive(Clone)]
pub struct SeedUser {
    pub username: String,
    pub password: String,
}

#[derive(Clone)]
pub struct AuthConfig {
    pub jwt_secret: String,
    pub seed_user: Option<SeedUser>,
}

impl AuthConfig {
    pub fn from_env() -> Result<Self, AppError> {
        let jwt_secret = std::env::var("SECRET_KEY")
            .or_else(|_| std::env::var("APP_SECRET"))
            .unwrap_or_else(|_| "dev-secret-change-me".into());

        let seed_user = match (
            std::env::var("APP_USERNAME").ok(),
            std::env::var("APP_PASSWORD").ok(),
        ) {
            (Some(username), Some(password)) => {
                if password.chars().count() < MIN_PASSWORD_LENGTH {
                    return Err(AppError::Config(format!(
                        "APP_PASSWORD must be at least {MIN_PASSWORD_LENGTH} characters"
                    )));
                }
                Some(SeedUser { username, password })
            }
            _ => None,
        };

        Ok(Self {
            jwt_secret,
            seed_user,
        })
    }
}

#[derive(Clone)]
pub struct DatabaseConfig {
    pub database_url: String,
}

#[derive(Clone)]
pub struct RedisConfig {
    pub redis_url: String,
}

#[derive(Clone)]
pub struct NatsConfig {
    pub nats_url: String,
}

#[derive(Clone)]
pub struct ClickHouseConfig {
    pub url: String,
    pub database: String,
    pub user: String,
    pub password: String,
}

#[derive(Clone)]
pub struct YoutubeConfig {
    pub api_key: String,
    pub regions: Vec<String>,
    pub themes: Vec<String>,
}

#[derive(Clone)]
pub struct ScanConfig {
    pub interval_minutes: u64,
}

#[derive(Clone)]
pub struct AppConfig {
    pub env: String,
    pub frontend_origin: String,
    pub auth: AuthConfig,
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
    pub nats: NatsConfig,
    pub clickhouse: ClickHouseConfig,
    pub youtube: YoutubeConfig,
    pub scan: ScanConfig,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, AppError> {
        Ok(Self {
            env: std::env::var("APP_ENV").unwrap_or_else(|_| "local".to_string()),
            frontend_origin: std::env::var("FRONTEND_ORIGIN")
                .unwrap_or_else(|_| "http://localhost:5173".to_string()),
            auth: AuthConfig::from_env()?,
            database: DatabaseConfig {
                database_url: normalize_database_url()?,
            },
            redis: RedisConfig {
                redis_url: std::env::var("REDIS_URL")
                    .unwrap_or_else(|_| "redis://localhost:6379".to_string()),
            },
            nats: NatsConfig {
                nats_url: std::env::var("NATS_URL")
                    .unwrap_or_else(|_| "nats://localhost:4222".to_string()),
            },
            clickhouse: ClickHouseConfig {
                url: std::env::var("CLICKHOUSE_URL")
                    .unwrap_or_else(|_| "http://localhost:8123".to_string()),
                database: std::env::var("CLICKHOUSE_DATABASE")
                    .unwrap_or_else(|_| "viral_analytics".to_string()),
                user: std::env::var("CLICKHOUSE_USER").unwrap_or_else(|_| "viral".to_string()),
                password: std::env::var("CLICKHOUSE_PASSWORD")
                    .unwrap_or_else(|_| "viral".to_string()),
            },
            youtube: YoutubeConfig::from_env(),
            scan: ScanConfig {
                interval_minutes: std::env::var("SCAN_INTERVAL_MINUTES")
                    .ok()
                    .and_then(|value| value.parse::<u64>().ok())
                    .unwrap_or(30),
            },
        })
    }
}

impl YoutubeConfig {
    pub fn from_env() -> Self {
        let api_key = std::env::var("YOUTUBE_API_KEY").unwrap_or_default();
        let regions = std::env::var("REGIONS")
            .unwrap_or_else(|_| "FR,US,ES".to_string())
            .split(',')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(ToString::to_string)
            .collect();
        let themes = std::env::var("THEMES")
            .unwrap_or_else(|_| "business,food,humor".to_string())
            .split(',')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(ToString::to_string)
            .collect();

        Self {
            api_key,
            regions,
            themes,
        }
    }
}

pub fn normalize_database_url() -> Result<String, AppError> {
    let mut url = std::env::var("DATABASE_URL")
        .map_err(|_| AppError::Config("DATABASE_URL is required".into()))?;

    if !url.contains("sslmode=") {
        let separator = if url.contains('?') { '&' } else { '?' };
        url.push(separator);
        url.push_str("sslmode=require");
    }

    Ok(url)
}

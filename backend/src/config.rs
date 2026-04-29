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
pub struct TiktokConfig {
    pub api_key: String,
    pub api_base: String,
}

#[derive(Clone)]
pub struct InstagramConfig {
    pub api_key: String,
    pub api_base: String,
}

#[derive(Clone)]
pub struct TelegramConfig {
    pub bot_token: String,
    pub default_chat_id: String,
}

#[derive(Debug, Clone)]
pub struct SmtpConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub from: String,
    pub tls: bool,
}

impl SmtpConfig {
    pub fn is_configured(&self) -> bool {
        !self.host.is_empty()
            && !self.username.is_empty()
            && !self.password.is_empty()
            && !self.from.is_empty()
    }
}

#[derive(Debug, Clone)]
pub struct StorageConfig {
    pub s3_endpoint: String,
    pub s3_region: String,
    pub s3_bucket: String,
    pub s3_access_key_id: String,
    pub s3_secret_access_key: String,
    pub s3_force_path_style: bool,
    pub local_exports_dir: String,
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
    pub tiktok: TiktokConfig,
    pub instagram: InstagramConfig,
    pub telegram: TelegramConfig,
    pub smtp: SmtpConfig,
    pub storage: StorageConfig,
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
            tiktok: TiktokConfig {
                api_key: std::env::var("TIKTOK_API_KEY").unwrap_or_default(),
                api_base: std::env::var("TIKTOK_API_BASE").unwrap_or_default(),
            },
            instagram: InstagramConfig {
                api_key: std::env::var("INSTAGRAM_API_KEY").unwrap_or_default(),
                api_base: std::env::var("INSTAGRAM_API_BASE").unwrap_or_default(),
            },
            telegram: TelegramConfig {
                bot_token: std::env::var("TELEGRAM_BOT_TOKEN").unwrap_or_default(),
                default_chat_id: std::env::var("TELEGRAM_DEFAULT_CHAT_ID").unwrap_or_default(),
            },
            smtp: SmtpConfig {
                host: std::env::var("SMTP_HOST").unwrap_or_default(),
                port: std::env::var("SMTP_PORT")
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(587),
                username: std::env::var("SMTP_USERNAME").unwrap_or_default(),
                password: std::env::var("SMTP_PASSWORD").unwrap_or_default(),
                from: std::env::var("SMTP_FROM").unwrap_or_default(),
                tls: std::env::var("SMTP_TLS")
                    .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
                    .unwrap_or(true),
            },
            storage: StorageConfig {
                s3_endpoint: std::env::var("S3_ENDPOINT").unwrap_or_default(),
                s3_region: std::env::var("S3_REGION").unwrap_or_default(),
                s3_bucket: std::env::var("S3_BUCKET").unwrap_or_default(),
                s3_access_key_id: std::env::var("S3_ACCESS_KEY_ID").unwrap_or_default(),
                s3_secret_access_key: std::env::var("S3_SECRET_ACCESS_KEY").unwrap_or_default(),
                s3_force_path_style: std::env::var("S3_FORCE_PATH_STYLE")
                    .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
                    .unwrap_or(true),
                local_exports_dir: std::env::var("LOCAL_EXPORTS_DIR")
                    .unwrap_or_else(|_| "exports".to_string()),
            },
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

impl TiktokConfig {
    pub fn is_configured(&self) -> bool {
        !self.api_key.is_empty() && !self.api_base.is_empty()
    }
}
impl InstagramConfig {
    pub fn is_configured(&self) -> bool {
        !self.api_key.is_empty() && !self.api_base.is_empty()
    }
}

impl TelegramConfig {
    pub fn is_configured(&self) -> bool {
        !self.bot_token.is_empty()
    }
    pub fn fallback_chat_id(&self) -> Option<&str> {
        if self.default_chat_id.is_empty() {
            None
        } else {
            Some(&self.default_chat_id)
        }
    }
}

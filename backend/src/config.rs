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
pub struct YoutubeConfig {
    pub api_key: String,
    pub regions: Vec<String>,
    pub themes: Vec<String>,
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

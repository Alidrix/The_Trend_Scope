use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts, Json, State},
    http::{header, request::Parts, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use chrono::{DateTime, Utc};
use dotenvy::dotenv;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, postgres::PgRow, PgPool, Row};
use std::net::SocketAddr;
use thiserror::Error;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::{error, info};

#[derive(Clone)]
struct AppState {
    pool: PgPool,
    auth: AuthConfig,
    youtube: YoutubeConfig,
    http: Client,
}

impl FromRef<AppState> for PgPool {
    fn from_ref(app: &AppState) -> PgPool {
        app.pool.clone()
    }
}

impl FromRef<AppState> for AuthConfig {
    fn from_ref(app: &AppState) -> AuthConfig {
        app.auth.clone()
    }
}

impl FromRef<AppState> for YoutubeConfig {
    fn from_ref(app: &AppState) -> YoutubeConfig {
        app.youtube.clone()
    }
}

#[derive(Clone)]
struct AuthConfig {
    username: String,
    password_hash: String,
    jwt_secret: String,
}

impl AuthConfig {
    fn from_env() -> Result<Self, AppError> {
        let username = std::env::var("APP_USERNAME").map_err(|_| AppError::Config("APP_USERNAME".into()))?;
        let password = std::env::var("APP_PASSWORD").map_err(|_| AppError::Config("APP_PASSWORD".into()))?;
        if password.chars().count() < 16 {
            return Err(AppError::Config("APP_PASSWORD must be at least 16 characters".into()));
        }
        let password_hash = bcrypt::hash(password, bcrypt::DEFAULT_COST)?;
        let jwt_secret = std::env::var("APP_SECRET").unwrap_or_else(|_| "dev-secret-change-me".into());
        Ok(Self {
            username,
            password_hash,
            jwt_secret,
        })
    }

    fn verify(&self, username: &str, password: &str) -> bool {
        username == self.username && bcrypt::verify(password, &self.password_hash).unwrap_or(false)
    }

    fn encoding_key(&self) -> EncodingKey {
        EncodingKey::from_secret(self.jwt_secret.as_bytes())
    }

    fn decoding_key(&self) -> DecodingKey {
        DecodingKey::from_secret(self.jwt_secret.as_bytes())
    }
}

#[derive(Clone)]
struct YoutubeConfig {
    api_key: String,
}

impl YoutubeConfig {
    fn from_env() -> Result<Self, AppError> {
        let api_key = std::env::var("YOUTUBE_API_KEY").unwrap_or_else(|_| "demo-key".into());
        Ok(Self { api_key })
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

#[derive(Debug, Deserialize)]
struct Credentials {
    username: String,
    password: String,
}

#[derive(Debug, Serialize)]
struct AuthResponse {
    token: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Video {
    id: uuid::Uuid,
    youtube_id: String,
    title: String,
    category: String,
    views_per_hour: i64,
    duration_seconds: i32,
    published_at: DateTime<Utc>,
    is_short: bool,
    notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct VideoPayload {
    youtube_id: String,
    title: String,
    category: String,
    views_per_hour: i64,
    duration_seconds: i32,
    published_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
struct ApiVideosResponse {
    videos: Vec<Video>,
}

#[derive(Debug, Serialize)]
struct ApiMessage {
    message: String,
}

#[derive(Debug, Deserialize)]
struct NotePayload {
    video_id: uuid::Uuid,
    notes: String,
}

#[derive(Debug, Error)]
enum AppError {
    #[error("configuration error: {0}")]
    Config(String),
    #[error("unauthorized")]
    Unauthorized,
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("internal error")]
    Internal,
    #[error("hashing error: {0}")]
    Hash(#[from] bcrypt::BcryptError),
    #[error("network error: {0}")]
    Network(#[from] reqwest::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let status = match self {
            AppError::Unauthorized => StatusCode::UNAUTHORIZED,
            AppError::Config(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Internal => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Hash(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Network(_) => StatusCode::BAD_GATEWAY,
        };
        let msg = self.to_string();
        (status, Json(ApiMessage { message: msg })).into_response()
    }
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@db:5432/viral".into());
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    let auth = AuthConfig::from_env()?;
    let youtube = YoutubeConfig::from_env()?;
    let state = AppState {
        pool,
        auth,
        youtube,
        http: Client::new(),
    };

    let app = Router::new()
        .route("/api/v1/health", get(health))
        .route("/api/v1/auth/login", post(login))
        .route("/api/v1/videos", get(list_videos).post(refresh_videos))
        .route("/api/v1/notes", post(update_note))
        .layer(
            CorsLayer::very_permissive()
                .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
                .allow_methods([http::Method::GET, http::Method::POST]),
        )
        .layer(TraceLayer::new_for_http())
        .with_state(state.clone());

    let addr: SocketAddr = "0.0.0.0:4443".parse().unwrap();
    info!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .map_err(|_| AppError::Internal)?;
    axum::serve(listener, app)
        .await
        .map_err(|_| AppError::Internal)
}

async fn health() -> Json<ApiMessage> {
    Json(ApiMessage {
        message: "ok".into(),
    })
}

async fn login(State(state): State<AppState>, Json(payload): Json<Credentials>) -> Result<Json<AuthResponse>, AppError> {
    if !state.auth.verify(&payload.username, &payload.password) {
        return Err(AppError::Unauthorized);
    }

    let claims = Claims {
        sub: payload.username,
        exp: (chrono::Utc::now() + chrono::Duration::hours(12)).timestamp() as usize,
    };
    let token = encode(&Header::default(), &claims, &state.auth.encoding_key())
        .map_err(|_| AppError::Internal)?;

    Ok(Json(AuthResponse { token }))
}

async fn list_videos(_auth: AuthBearer, State(pool): State<PgPool>) -> Result<Json<ApiVideosResponse>, AppError> {
    let records = sqlx::query(
        "SELECT id, youtube_id, title, category, views_per_hour, duration_seconds, published_at, notes FROM videos ORDER BY views_per_hour DESC",
    )
    .map(|row: PgRow| Video {
        id: row.get("id"),
        youtube_id: row.get("youtube_id"),
        title: row.get("title"),
        category: row.get("category"),
        views_per_hour: row.get("views_per_hour"),
        duration_seconds: row.get("duration_seconds"),
        published_at: row.get("published_at"),
        is_short: row.get::<i32, _>("duration_seconds") <= 60,
        notes: row.get("notes"),
    })
    .fetch_all(&pool)
    .await?;

    Ok(Json(ApiVideosResponse { videos: records }))
}

async fn refresh_videos(
    _auth: AuthBearer,
    State(state): State<AppState>,
    Json(payload): Json<Vec<VideoPayload>>,
) -> Result<Json<ApiMessage>, AppError> {
    for item in payload {
        let is_short = item.duration_seconds <= 60;
        let id = uuid::Uuid::new_v4();
        sqlx::query(
            r#"INSERT INTO videos (id, youtube_id, title, category, views_per_hour, duration_seconds, published_at, notes)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            ON CONFLICT (youtube_id) DO UPDATE SET views_per_hour = EXCLUDED.views_per_hour, title = EXCLUDED.title, category = EXCLUDED.category, duration_seconds = EXCLUDED.duration_seconds"#,
        )
        .bind(id)
        .bind(&item.youtube_id)
        .bind(&item.title)
        .bind(&item.category)
        .bind(item.views_per_hour)
        .bind(item.duration_seconds)
        .bind(item.published_at)
        .bind(Option::<String>::None)
        .execute(&state.pool)
        .await?;

        sqlx::query(
            r#"INSERT INTO video_stats (id, video_id, views_per_hour, collected_at) VALUES ($1, $2, $3, $4)"#,
        )
        .bind(uuid::Uuid::new_v4())
        .bind(id)
        .bind(item.views_per_hour)
        .bind(chrono::Utc::now())
        .execute(&state.pool)
        .await?;

        info!("upserted video {} short={}", item.youtube_id, is_short);
    }

    Ok(Json(ApiMessage {
        message: "videos refreshed".into(),
    }))
}

async fn update_note(
    _auth: AuthBearer,
    State(pool): State<PgPool>,
    Json(payload): Json<NotePayload>,
) -> Result<Json<ApiMessage>, AppError> {
    sqlx::query("UPDATE videos SET notes = $1 WHERE id = $2")
        .bind(&payload.notes)
        .bind(payload.video_id)
        .execute(&pool)
        .await?;

    Ok(Json(ApiMessage {
        message: "note updated".into(),
    }))
}

#[derive(Debug, Serialize, Deserialize)]
struct YoutubeSearchItem {
    id: YoutubeVideoId,
    snippet: YoutubeSnippet,
    contentDetails: Option<YoutubeContentDetails>,
    statistics: Option<YoutubeStatistics>,
}

#[derive(Debug, Serialize, Deserialize)]
struct YoutubeVideoId {
    videoId: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct YoutubeSnippet {
    title: String,
    categoryId: Option<String>,
    publishedAt: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct YoutubeContentDetails {
    duration: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct YoutubeStatistics {
    viewCount: Option<String>,
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthBearer
where
    AuthConfig: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get(header::AUTHORIZATION)
            .cloned()
            .ok_or(AppError::Unauthorized)?;
        let value = auth_header
            .to_str()
            .map_err(|_| AppError::Unauthorized)?
            .to_string();

        if !value.to_ascii_lowercase().starts_with("bearer ") {
            return Err(AppError::Unauthorized);
        }
        let token = value[7..].trim();
        let config = AuthConfig::from_ref(state);
        decode::<Claims>(token, &config.decoding_key(), &Validation::default())
            .map_err(|_| AppError::Unauthorized)?;
        Ok(AuthBearer)
    }
}

struct AuthBearer;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn auth_rejects_short_password() {
        std::env::set_var("APP_USERNAME", "demo");
        std::env::set_var("APP_PASSWORD", "short");
        let result = AuthConfig::from_env();
        assert!(result.is_err());
    }

    #[test]
    fn video_short_flag_detection() {
        let video = Video {
            id: uuid::Uuid::new_v4(),
            youtube_id: "abc".into(),
            title: "Test".into(),
            category: "News".into(),
            views_per_hour: 100,
            duration_seconds: 45,
            published_at: Utc::now(),
            is_short: true,
            notes: None,
        };
        assert!(video.is_short);
    }

    #[tokio::test]
    async fn login_succeeds_for_valid_credentials() {
        std::env::set_var("APP_USERNAME", "demo");
        std::env::set_var("APP_PASSWORD", "averylongpassword!!");
        std::env::set_var("APP_SECRET", "secret");
        let auth = AuthConfig::from_env().unwrap();
        let state = AppState {
            pool: PgPoolOptions::new()
                .max_connections(1)
                .connect_lazy("postgres://postgres:postgres@localhost:5432/postgres")
                .expect("lazy pool"),
            auth: auth.clone(),
            youtube: YoutubeConfig { api_key: "demo".into() },
            http: Client::new(),
        };

        let creds = Credentials {
            username: "demo".into(),
            password: "averylongpassword!!".into(),
        };
        let response = login(State(state), Json(creds)).await.unwrap();
        assert!(!response.token.is_empty());
    }
}

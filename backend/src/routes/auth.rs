use axum::{extract::State, Json};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

use crate::{error::AppError, models::plan::PlanTier, state::AppState};

const MIN_PASSWORD_LENGTH: usize = 10;

#[derive(Debug, Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct RegisterPayload {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub plan: PlanTier,
}

#[derive(Debug, Serialize)]
pub struct AuthStatus {
    pub needs_setup: bool,
    pub has_api_key: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub async fn auth_status(State(state): State<AppState>) -> Result<Json<AuthStatus>, AppError> {
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
        .fetch_one(&state.pool)
        .await?;

    Ok(Json(AuthStatus {
        needs_setup: count == 0,
        has_api_key: !state.youtube.api_key.is_empty(),
    }))
}

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterPayload>,
) -> Result<Json<crate::error::ApiMessage>, AppError> {
    if payload.password.chars().count() < MIN_PASSWORD_LENGTH {
        return Err(AppError::BadRequest(format!(
            "Le mot de passe doit contenir au moins {MIN_PASSWORD_LENGTH} caractères"
        )));
    }

    let username_exists: Option<String> =
        sqlx::query_scalar("SELECT username FROM users WHERE username = $1")
            .bind(&payload.username)
            .fetch_optional(&state.pool)
            .await?;
    if username_exists.is_some() {
        return Err(AppError::Conflict);
    }

    let password_hash = bcrypt::hash(&payload.password, bcrypt::DEFAULT_COST)?;
    sqlx::query(
        "INSERT INTO users (id, username, password_hash, plan) VALUES ($1, $2, $3, 'free')",
    )
    .bind(uuid::Uuid::new_v4())
    .bind(&payload.username)
    .bind(password_hash)
    .execute(&state.pool)
    .await?;

    Ok(Json(crate::error::ApiMessage {
        message: "Compte créé, vous pouvez vous connecter".into(),
    }))
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<Credentials>,
) -> Result<Json<AuthResponse>, AppError> {
    let Some((password_hash, plan)) = sqlx::query_as::<_, (String, PlanTier)>(
        "SELECT password_hash, plan FROM users WHERE username = $1",
    )
    .bind(&payload.username)
    .fetch_optional(&state.pool)
    .await?
    else {
        return Err(AppError::Unauthorized);
    };

    if !bcrypt::verify(&payload.password, &password_hash).unwrap_or(false) {
        return Err(AppError::Unauthorized);
    }

    let claims = Claims {
        sub: payload.username,
        exp: (chrono::Utc::now() + chrono::Duration::hours(12)).timestamp() as usize,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.auth.jwt_secret.as_bytes()),
    )
    .map_err(|_| AppError::Internal)?;

    Ok(Json(AuthResponse { token, plan }))
}

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
    pub display_name: Option<String>,
    pub country: Option<String>,
    pub timezone: Option<String>,
    pub profile_type: Option<String>,
    pub objective: Option<String>,
    pub platforms: Option<Vec<String>>,
    pub categories: Option<Vec<String>>,
    pub accept_terms: bool,
    pub accept_privacy: bool,
    pub marketing_opt_in: Option<bool>,
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
        has_api_key: !state.config.youtube.api_key.is_empty(),
    }))
}

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterPayload>,
) -> Result<Json<crate::error::ApiMessage>, AppError> {
    if !payload.accept_terms || !payload.accept_privacy {
        return Err(AppError::BadRequest(
            "L'acceptation des CGU et de la politique de confidentialité est obligatoire".into(),
        ));
    }

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

    let user_id = uuid::Uuid::new_v4();
    let password_hash = bcrypt::hash(&payload.password, bcrypt::DEFAULT_COST)?;
    sqlx::query(
        "INSERT INTO users (id, username, password_hash, plan, role) VALUES ($1, $2, $3, 'free', 'user')",
    )
    .bind(user_id)
    .bind(&payload.username)
    .bind(password_hash)
    .execute(&state.pool)
    .await?;

    sqlx::query(
        "INSERT INTO user_profiles (user_id, display_name, country, timezone, profile_type)
         VALUES ($1, $2, $3, $4, $5)",
    )
    .bind(user_id)
    .bind(payload.display_name)
    .bind(payload.country)
    .bind(payload.timezone)
    .bind(payload.profile_type)
    .execute(&state.pool)
    .await?;

    sqlx::query(
        "INSERT INTO user_preferences (user_id, objective, categories, platforms)
         VALUES ($1, $2, $3, $4)",
    )
    .bind(user_id)
    .bind(payload.objective)
    .bind(payload.categories.unwrap_or_default())
    .bind(payload.platforms.unwrap_or_default())
    .execute(&state.pool)
    .await?;

    let marketing = payload.marketing_opt_in.unwrap_or(false);
    sqlx::query(
        "INSERT INTO consents (user_id, consent_type, granted, version)
         VALUES ($1, 'terms', true, 'v1'), ($1, 'privacy', true, 'v1'), ($1, 'marketing', $2, 'v1')",
    )
    .bind(user_id)
    .bind(marketing)
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
        &EncodingKey::from_secret(state.config.auth.jwt_secret.as_bytes()),
    )
    .map_err(|_| AppError::Internal)?;

    Ok(Json(AuthResponse { token, plan }))
}

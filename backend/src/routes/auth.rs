use axum::{extract::State, Json};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{
    error::AppError,
    models::plan::PlanTier,
    repositories::{email_verification, password_reset},
    services::{email, rate_limit},
    state::AppState,
};

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

#[derive(Debug, Deserialize)]
pub struct ForgotPasswordPayload {
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct VerifyEmailPayload {
    pub token: String,
}

#[derive(Debug, Deserialize)]
pub struct ResetPasswordPayload {
    pub token: String,
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
        has_api_key: !state.config.youtube.api_key.is_empty(),
    }))
}

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterPayload>,
) -> Result<Json<crate::error::ApiMessage>, AppError> {
    let limit_key = rate_limit::key("auth:register", "global");
    if !rate_limit::check_limit(&state.redis, &limit_key, 5, 3600).await? {
        return Err(AppError::TooManyRequests);
    }

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

    let verify_token = uuid::Uuid::new_v4().to_string();
    let token_hash = hex::encode(Sha256::digest(verify_token.as_bytes()));
    let expires_at = chrono::Utc::now() + chrono::Duration::hours(24);
    email_verification::create_token(&state.pool, user_id, &token_hash, expires_at).await?;
    let verify_url = format!(
        "{}/verify-email?token={verify_token}",
        state.config.frontend_origin
    );
    let body = email::render_template("verify-email", Some(&verify_url));
    email::send_email(
        &state.pool,
        Some(user_id),
        &payload.username,
        "Verify your email",
        &body,
    )
    .await?;

    Ok(Json(crate::error::ApiMessage {
        message: "Compte créé, vous pouvez vous connecter".into(),
    }))
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<Credentials>,
) -> Result<Json<AuthResponse>, AppError> {
    let limit_key = rate_limit::key("auth:login", &payload.username);
    if !rate_limit::check_limit(&state.redis, &limit_key, 10, 600).await? {
        return Err(AppError::TooManyRequests);
    }

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

pub async fn forgot_password(
    State(state): State<AppState>,
    Json(payload): Json<ForgotPasswordPayload>,
) -> Result<Json<crate::error::ApiMessage>, AppError> {
    let limit_key = rate_limit::key("auth:forgot-password", &payload.email);
    if !rate_limit::check_limit(&state.redis, &limit_key, 5, 1800).await? {
        return Err(AppError::TooManyRequests);
    }

    let maybe_user: Option<(uuid::Uuid, String)> =
        sqlx::query_as("SELECT id, username FROM users WHERE username = $1")
            .bind(&payload.email)
            .fetch_optional(&state.pool)
            .await?;

    if let Some((user_id, username)) = maybe_user {
        let token_raw = uuid::Uuid::new_v4().to_string();
        let token_hash = hex::encode(Sha256::digest(token_raw.as_bytes()));
        let expires_at = chrono::Utc::now() + chrono::Duration::minutes(30);
        password_reset::create_token(&state.pool, user_id, &token_hash, expires_at).await?;
        let reset_url = format!(
            "{}/reset-password?token={}",
            state.config.frontend_origin, token_raw
        );
        let body = email::render_template("reset-password", Some(&reset_url));
        email::send_email(
            &state.pool,
            Some(user_id),
            &username,
            "Reset password",
            &body,
        )
        .await?;
    }

    Ok(Json(crate::error::ApiMessage {
        message: "If an account exists, a reset link has been sent".into(),
    }))
}

pub async fn resend_verification(
    State(state): State<AppState>,
    Json(payload): Json<ForgotPasswordPayload>,
) -> Result<Json<crate::error::ApiMessage>, AppError> {
    let maybe_user: Option<(uuid::Uuid, bool)> =
        sqlx::query_as("SELECT id, email_verified FROM users WHERE username = $1")
            .bind(&payload.email)
            .fetch_optional(&state.pool)
            .await?;

    if let Some((user_id, false)) = maybe_user {
        let token_raw = uuid::Uuid::new_v4().to_string();
        let token_hash = hex::encode(Sha256::digest(token_raw.as_bytes()));
        let expires_at = chrono::Utc::now() + chrono::Duration::hours(24);
        email_verification::create_token(&state.pool, user_id, &token_hash, expires_at).await?;
        let verify_url = format!(
            "{}/verify-email?token={token_raw}",
            state.config.frontend_origin
        );
        let body = email::render_template("verify-email", Some(&verify_url));
        email::send_email(
            &state.pool,
            Some(user_id),
            &payload.email,
            "Verify your email",
            &body,
        )
        .await?;
    }

    Ok(Json(crate::error::ApiMessage {
        message: "verification email sent if account exists".into(),
    }))
}

pub async fn verify_email(
    State(state): State<AppState>,
    Json(payload): Json<VerifyEmailPayload>,
) -> Result<Json<crate::error::ApiMessage>, AppError> {
    let token_hash = hex::encode(Sha256::digest(payload.token.as_bytes()));
    let Some(user_id) = email_verification::consume_token(&state.pool, &token_hash).await? else {
        return Err(AppError::BadRequest(
            "invalid or expired verification token".into(),
        ));
    };

    sqlx::query("UPDATE users SET email_verified = true WHERE id = $1")
        .bind(user_id)
        .execute(&state.pool)
        .await?;

    Ok(Json(crate::error::ApiMessage {
        message: "email verified".into(),
    }))
}

pub async fn reset_password(
    State(state): State<AppState>,
    Json(payload): Json<ResetPasswordPayload>,
) -> Result<Json<crate::error::ApiMessage>, AppError> {
    if payload.password.chars().count() < MIN_PASSWORD_LENGTH {
        return Err(AppError::BadRequest(format!(
            "Le mot de passe doit contenir au moins {MIN_PASSWORD_LENGTH} caractères"
        )));
    }

    let token_hash = hex::encode(Sha256::digest(payload.token.as_bytes()));
    let Some(user_id) = password_reset::consume_token(&state.pool, &token_hash).await? else {
        return Err(AppError::BadRequest("invalid or expired token".into()));
    };

    let password_hash = bcrypt::hash(&payload.password, bcrypt::DEFAULT_COST)?;
    sqlx::query("UPDATE users SET password_hash = $2 WHERE id = $1")
        .bind(user_id)
        .bind(password_hash)
        .execute(&state.pool)
        .await?;

    Ok(Json(crate::error::ApiMessage {
        message: "password has been reset".into(),
    }))
}

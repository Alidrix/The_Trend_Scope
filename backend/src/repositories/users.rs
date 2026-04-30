use crate::{config::AuthConfig, error::AppError, models::user::CurrentUser};
use sqlx::PgPool;

pub async fn ensure_seed_user(pool: &PgPool, config: &AuthConfig) -> Result<(), AppError> {
    let Some(seed) = &config.seed_user else {
        return Ok(());
    };

    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
        .fetch_one(pool)
        .await?;

    if count == 0 {
        let password_hash = bcrypt::hash(&seed.password, bcrypt::DEFAULT_COST)?;
        sqlx::query(
            "INSERT INTO users (id, username, password_hash, plan, role) VALUES ($1, $2, $3, 'studio', 'admin')",
        )
        .bind(uuid::Uuid::new_v4())
        .bind(&seed.username)
        .bind(password_hash)
        .execute(pool)
        .await?;
    }

    Ok(())
}

pub async fn find_user_id_by_username(
    pool: &PgPool,
    username: &str,
) -> Result<uuid::Uuid, AppError> {
    sqlx::query_scalar("SELECT id FROM users WHERE username = $1")
        .bind(username)
        .fetch_one(pool)
        .await
        .map_err(AppError::from)
}

pub async fn current_user(pool: &PgPool, username: &str) -> Result<CurrentUser, AppError> {
    sqlx::query_as::<_, CurrentUser>(
        "SELECT u.id, NULL::TEXT as email, u.username, p.display_name, u.role, u.plan, p.country, p.profile_type, u.created_at, u.email_verified\n         FROM users u\n         LEFT JOIN user_profiles p ON p.user_id = u.id\n         WHERE u.username = $1",
    )
    .bind(username)
    .fetch_one(pool)
    .await
    .map_err(AppError::from)
}

pub async fn update_user_plan(
    pool: &PgPool,
    user_id: uuid::Uuid,
    plan: &str,
) -> Result<(), AppError> {
    sqlx::query("UPDATE users SET plan = $2::plan_tier WHERE id = $1")
        .bind(user_id)
        .bind(plan)
        .execute(pool)
        .await?;
    Ok(())
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct UserAccessRow {
    pub id: uuid::Uuid,
    pub role: String,
}

pub async fn find_access_by_username(
    pool: &PgPool,
    username: &str,
) -> Result<Option<UserAccessRow>, AppError> {
    sqlx::query_as::<_, UserAccessRow>("SELECT id, role FROM users WHERE username = $1")
        .bind(username)
        .fetch_optional(pool)
        .await
        .map_err(AppError::from)
}

use crate::{config::AuthConfig, error::AppError};
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
        sqlx::query("INSERT INTO users (id, username, password_hash) VALUES ($1, $2, $3)")
            .bind(uuid::Uuid::new_v4())
            .bind(&seed.username)
            .bind(password_hash)
            .execute(pool)
            .await?;
    }

    Ok(())
}

use redis::AsyncCommands;

use crate::error::AppError;

pub fn key(scope: &str, identifier: &str) -> String {
    format!("rate_limit:{scope}:{identifier}")
}

pub async fn check_limit(
    redis: &redis::Client,
    key: &str,
    max: u64,
    window_seconds: u64,
) -> Result<bool, AppError> {
    let mut conn = redis
        .get_multiplexed_async_connection()
        .await
        .map_err(|_| AppError::Internal)?;

    let count: u64 = conn.incr(key, 1_u8).await.map_err(|_| AppError::Internal)?;
    if count == 1 {
        let _: () = conn
            .expire(key, window_seconds as i64)
            .await
            .map_err(|_| AppError::Internal)?;
    }

    Ok(count <= max)
}

#[cfg(test)]
mod tests {
    use super::key;

    #[test]
    fn key_generation() {
        assert_eq!(
            key("auth:login", "127.0.0.1"),
            "rate_limit:auth:login:127.0.0.1"
        );
    }
}

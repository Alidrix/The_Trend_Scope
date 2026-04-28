use redis::AsyncCommands;

use crate::error::AppError;

pub async fn set_json(
    client: &redis::Client,
    key: &str,
    value: &str,
    ttl_seconds: u64,
) -> Result<(), AppError> {
    let mut conn = client
        .get_multiplexed_async_connection()
        .await
        .map_err(|_| AppError::Internal)?;
    let _: () = conn
        .set_ex(key, value, ttl_seconds)
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(())
}

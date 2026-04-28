use redis::AsyncCommands;

use crate::error::AppError;

pub async fn increment_route_counter(
    client: &redis::Client,
    key: &str,
    ttl_seconds: u64,
) -> Result<i64, AppError> {
    let mut con = client.get_multiplexed_async_connection().await?;
    let current: i64 = con.incr(key, 1).await?;
    if current == 1 {
        let _: () = con.expire(key, ttl_seconds as i64).await?;
    }
    Ok(current)
}

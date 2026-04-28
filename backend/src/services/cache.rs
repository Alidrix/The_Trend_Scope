use redis::AsyncCommands;

use crate::error::AppError;

#[derive(Clone)]
pub struct CacheService {
    pub client: redis::Client,
}

impl CacheService {
    pub fn new(redis_url: &str) -> Result<Self, AppError> {
        let client = redis::Client::open(redis_url)
            .map_err(|err| AppError::Config(format!("invalid REDIS_URL: {err}")))?;
        Ok(Self { client })
    }

    pub async fn get_json(&self, key: &str) -> Result<Option<String>, AppError> {
        let mut con = self.client.get_multiplexed_async_connection().await?;
        let val: Option<String> = con.get(key).await?;
        Ok(val)
    }

    pub async fn set_json_with_ttl(
        &self,
        key: &str,
        value: &str,
        ttl_seconds: u64,
    ) -> Result<(), AppError> {
        let mut con = self.client.get_multiplexed_async_connection().await?;
        let _: () = con.set_ex(key, value, ttl_seconds).await?;
        Ok(())
    }
}

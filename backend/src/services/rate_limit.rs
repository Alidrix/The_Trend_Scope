use crate::error::AppError;

pub async fn check_limit(_redis: &redis::Client, _key: &str, _max: u64) -> Result<bool, AppError> {
    Ok(true)
}

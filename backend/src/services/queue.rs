use crate::error::AppError;

pub async fn publish_scan_tick(client: &async_nats::Client) -> Result<(), AppError> {
    client
        .publish("jobs.scan.tick", "tick".into())
        .await
        .map_err(|_| AppError::Internal)
}

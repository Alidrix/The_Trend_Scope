use crate::error::AppError;

#[derive(Clone)]
pub struct QueueService {
    pub client: async_nats::Client,
}

impl QueueService {
    pub async fn connect(url: &str) -> Result<Self, AppError> {
        let client = async_nats::connect(url)
            .await
            .map_err(|err| AppError::Config(format!("NATS connection failed: {err}")))?;
        Ok(Self { client })
    }

    pub async fn publish(&self, subject: &str, payload: Vec<u8>) -> Result<(), AppError> {
        self.client
            .publish(subject.to_string(), payload.into())
            .await
            .map_err(|err| AppError::Config(format!("NATS publish failed: {err}")))?;
        Ok(())
    }
}

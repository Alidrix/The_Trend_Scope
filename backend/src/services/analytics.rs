use crate::error::AppError;

pub async fn ensure_schema(client: &clickhouse::Client) -> Result<(), AppError> {
    client
        .query(
            "CREATE TABLE IF NOT EXISTS trend_rankings (
                at DateTime,
                platform String,
                trend_id String,
                trend_score Float64
            ) ENGINE = MergeTree ORDER BY (platform, trend_id, at)",
        )
        .execute()
        .await
        .map_err(|_| AppError::Internal)
}

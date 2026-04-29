use crate::error::AppError;
use serde::Serialize;
use sqlx::PgPool;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Notification {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub title: String,
    pub body: String,
    #[sqlx(rename = "type")]
    pub notification_type: String,
    pub payload: serde_json::Value,
    pub read_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

pub async fn create(
    pool: &PgPool,
    user_id: uuid::Uuid,
    title: &str,
    body: &str,
    notification_type: &str,
    payload: serde_json::Value,
) -> Result<uuid::Uuid, AppError> {
    let id = uuid::Uuid::new_v4();
    sqlx::query(
        "INSERT INTO notifications (id,user_id,title,body,type,payload) VALUES ($1,$2,$3,$4,$5,$6)",
    )
    .bind(id)
    .bind(user_id)
    .bind(title)
    .bind(body)
    .bind(notification_type)
    .bind(payload)
    .execute(pool)
    .await?;
    Ok(id)
}
pub async fn list_for_user(
    pool: &PgPool,
    user_id: uuid::Uuid,
) -> Result<Vec<Notification>, AppError> {
    sqlx::query_as::<_,Notification>("SELECT id,user_id,title,body,type,payload,read_at,created_at FROM notifications WHERE user_id=$1 ORDER BY created_at DESC LIMIT 100").bind(user_id).fetch_all(pool).await.map_err(AppError::from)
}
pub async fn unread_count(pool: &PgPool, user_id: uuid::Uuid) -> Result<i64, AppError> {
    Ok(sqlx::query_scalar(
        "SELECT COUNT(*) FROM notifications WHERE user_id=$1 AND read_at IS NULL",
    )
    .bind(user_id)
    .fetch_one(pool)
    .await?)
}
pub async fn mark_read(
    pool: &PgPool,
    user_id: uuid::Uuid,
    notification_id: uuid::Uuid,
) -> Result<(), AppError> {
    sqlx::query("UPDATE notifications SET read_at=NOW() WHERE id=$1 AND user_id=$2")
        .bind(notification_id)
        .bind(user_id)
        .execute(pool)
        .await?;
    Ok(())
}
pub async fn mark_all_read(pool: &PgPool, user_id: uuid::Uuid) -> Result<(), AppError> {
    sqlx::query("UPDATE notifications SET read_at=NOW() WHERE user_id=$1 AND read_at IS NULL")
        .bind(user_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn admin_snapshot(pool: &PgPool) -> Result<serde_json::Value, AppError> {
    let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM notifications")
        .fetch_one(pool)
        .await
        .unwrap_or(0);
    let unread: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM notifications WHERE read_at IS NULL")
            .fetch_one(pool)
            .await
            .unwrap_or(0);
    let latest = sqlx::query_as::<_, Notification>("SELECT id,user_id,title,body,type,payload,read_at,created_at FROM notifications ORDER BY created_at DESC LIMIT 20").fetch_all(pool).await.unwrap_or_default();
    Ok(serde_json::json!({"total": total, "unread": unread, "latest": latest}))
}

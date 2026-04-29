use crate::error::AppError;
use sqlx::PgPool;

pub async fn process_pending_reports(pool: &PgPool) -> Result<u64, AppError> {
    let rows = sqlx::query("UPDATE reports SET status='completed', completed_at=NOW(), summary = jsonb_build_object('top_platforms', platforms, 'kpis', jsonb_build_object('total_trends',0,'average_views_per_hour',0,'strong_opportunities',0)) WHERE status='pending'")
        .execute(pool)
        .await?;
    Ok(rows.rows_affected())
}

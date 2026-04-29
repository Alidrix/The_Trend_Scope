use crate::error::AppError;
use sqlx::{PgPool, Row};

pub async fn process_pending_reports(pool: &PgPool) -> Result<u64, AppError> {
    let reports = sqlx::query("SELECT id, period_start, period_end, platforms, categories, format FROM reports WHERE status='pending' ORDER BY created_at ASC LIMIT 10").fetch_all(pool).await?;
    let mut done = 0;
    for r in reports {
        let id: uuid::Uuid = r.get(0);
        let start: chrono::NaiveDate = r.get(1);
        let end: chrono::NaiveDate = r.get(2);
        let platforms: Vec<String> = r.get(3);
        let categories: Vec<String> = r.get(4);
        let format: String = r.get(5);
        let q=sqlx::query("SELECT title, platform, category, COALESCE(region,''), views_per_hour FROM videos WHERE published_at::date BETWEEN $1 AND $2 AND ($3::text[]='{}' OR platform = ANY($3)) AND ($4::text[]='{}' OR category = ANY($4)) ORDER BY views_per_hour DESC LIMIT 20")
        .bind(start).bind(end).bind(&platforms).bind(&categories).fetch_all(pool).await;
        match q {
            Ok(rows) => {
                let total = rows.len() as i64;
                let avg = if total > 0 {
                    rows.iter().map(|x| x.get::<i64, _>(4)).sum::<i64>() / total
                } else {
                    0
                };
                let summary = serde_json::json!({"top_platforms":platforms,"top_categories":categories,"top_trends":rows.iter().map(|x|serde_json::json!({"title":x.get::<String,_>(0),"platform":x.get::<String,_>(1),"category":x.get::<String,_>(2),"region":x.get::<String,_>(3),"views_per_hour":x.get::<i64,_>(4)})).collect::<Vec<_>>(),"kpis":{"total_trends":total,"average_views_per_hour":avg,"strong_opportunities":rows.iter().filter(|x| x.get::<i64,_>(4)>=10000).count()},"recommendations":["Surveiller les tendances business en forte accélération.","Préparer des formats courts autour des catégories dominantes."],"file_generation": if format=="json" {serde_json::Value::Null} else {serde_json::json!("planned")} });
                sqlx::query("UPDATE reports SET status='completed', summary=$2, file_url=NULL, completed_at=NOW(), error_message=NULL WHERE id=$1").bind(id).bind(summary).execute(pool).await?;
                done += 1;
            }
            Err(e) => {
                sqlx::query("UPDATE reports SET status='failed', error_message=$2 WHERE id=$1")
                    .bind(id)
                    .bind(e.to_string())
                    .execute(pool)
                    .await?;
            }
        }
    }
    Ok(done)
}

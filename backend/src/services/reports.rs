use crate::{config::StorageConfig, error::AppError, services::storage};
use sqlx::{PgPool, Row};

#[derive(Debug, Clone)]
pub struct ReportTrend {
    pub title: String,
    pub platform: String,
    pub category: String,
    pub region: String,
    pub views_per_hour: i64,
}

pub fn build_report_summary(
    trends: &[ReportTrend],
    platforms: &[String],
    categories: &[String],
    format: &str,
) -> serde_json::Value {
    let total = trends.len() as i64;
    let total_views: i64 = trends.iter().map(|t| t.views_per_hour).sum();
    let avg = if total > 0 { total_views / total } else { 0 };
    serde_json::json!({
      "top_platforms": platforms,
      "top_categories": categories,
      "top_trends": trends.iter().map(|x|serde_json::json!({"title":x.title,"platform":x.platform,"category":x.category,"region":x.region,"views_per_hour":x.views_per_hour})).collect::<Vec<_>>(),
      "kpis": {
        "total_trends": total,
        "average_views_per_hour": avg,
        "strong_opportunities": trends.iter().filter(|x| x.views_per_hour >= 10000).count()
      },
      "recommendations": [
        "Surveiller les tendances business en forte accélération.",
        "Préparer des formats courts autour des catégories dominantes.",
        format!("format={format}")
      ]
    })
}

pub fn build_csv_export(trends: &[ReportTrend]) -> Result<Vec<u8>, AppError> {
    let mut w = csv::Writer::from_writer(vec![]);
    w.write_record(["title", "platform", "category", "region", "views_per_hour"])
        .map_err(|_| AppError::Internal)?;
    for t in trends {
        w.write_record([
            &t.title,
            &t.platform,
            &t.category,
            &t.region,
            &t.views_per_hour.to_string(),
        ])
        .map_err(|_| AppError::Internal)?;
    }
    w.into_inner().map_err(|_| AppError::Internal)
}

pub async fn process_pending_reports(
    pool: &PgPool,
    storage_config: &StorageConfig,
) -> Result<u64, AppError> {
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
                let trends: Vec<ReportTrend> = rows
                    .iter()
                    .map(|x| ReportTrend {
                        title: x.get(0),
                        platform: x.get(1),
                        category: x.get(2),
                        region: x.get(3),
                        views_per_hour: x.get(4),
                    })
                    .collect();
                let mut summary = build_report_summary(&trends, &platforms, &categories, &format);
                let mut file_url = None;
                if format == "csv" {
                    let data = build_csv_export(&trends)?;
                    let filename = format!("report-{}.csv", id);
                    file_url =
                        Some(storage::store_local_export(storage_config, &filename, &data).await?);
                } else if format == "pdf" {
                    summary["file_generation"] = serde_json::json!("pdf_planned");
                }
                sqlx::query("UPDATE reports SET status='completed', summary=$2, file_url=$3, completed_at=NOW(), error_message=NULL WHERE id=$1").bind(id).bind(summary).bind(file_url).execute(pool).await?;
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

#[cfg(test)]
mod tests {
    use super::{build_csv_export, build_report_summary, ReportTrend};

    #[test]
    fn summary_kpis_are_correct() {
        let trends = vec![
            ReportTrend {
                title: "A".into(),
                platform: "youtube".into(),
                category: "biz".into(),
                region: "FR".into(),
                views_per_hour: 1000,
            },
            ReportTrend {
                title: "B".into(),
                platform: "youtube".into(),
                category: "biz".into(),
                region: "US".into(),
                views_per_hour: 15000,
            },
        ];
        let summary = build_report_summary(&trends, &["youtube".into()], &["biz".into()], "csv");
        assert_eq!(summary["kpis"]["total_trends"], 2);
        assert_eq!(summary["kpis"]["average_views_per_hour"], 8000);
        assert_eq!(summary["kpis"]["strong_opportunities"], 1);
    }

    #[test]
    fn csv_export_contains_headers_and_row() {
        let trends = vec![ReportTrend {
            title: "A".into(),
            platform: "youtube".into(),
            category: "biz".into(),
            region: "FR".into(),
            views_per_hour: 1000,
        }];
        let csv = String::from_utf8(build_csv_export(&trends).expect("csv")).expect("utf8");
        assert!(csv.contains("title,platform,category,region,views_per_hour"));
        assert!(csv.contains("A,youtube,biz,FR,1000"));
    }
}

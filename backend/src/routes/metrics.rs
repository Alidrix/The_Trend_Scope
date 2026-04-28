use axum::response::IntoResponse;

pub async fn metrics() -> impl IntoResponse {
    let body = [
        "# HELP http_requests_total Total HTTP requests",
        "# TYPE http_requests_total counter",
        "http_requests_total 0",
        "# HELP radar_requests_total Total radar requests",
        "# TYPE radar_requests_total counter",
        "radar_requests_total 0",
        "# HELP worker_jobs_failed_total Worker failed jobs",
        "# TYPE worker_jobs_failed_total counter",
        "worker_jobs_failed_total 0",
    ]
    .join("\n");

    ([("content-type", "text/plain; version=0.0.4")], body)
}

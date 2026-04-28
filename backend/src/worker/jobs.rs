#[derive(Debug, Clone)]
pub enum JobTopic {
    ScanYoutubeRequested,
    ScanYoutubeCompleted,
    ScanTiktokRequested,
    ScanInstagramRequested,
    ScoreTrendsRequested,
    ReportsGenerateRequested,
    AlertsDispatchRequested,
}

impl JobTopic {
    pub fn subject(&self) -> &'static str {
        match self {
            JobTopic::ScanYoutubeRequested => "scan.youtube.requested",
            JobTopic::ScanYoutubeCompleted => "scan.youtube.completed",
            JobTopic::ScanTiktokRequested => "scan.tiktok.requested",
            JobTopic::ScanInstagramRequested => "scan.instagram.requested",
            JobTopic::ScoreTrendsRequested => "score.trends.requested",
            JobTopic::ReportsGenerateRequested => "reports.generate.requested",
            JobTopic::AlertsDispatchRequested => "alerts.dispatch.requested",
        }
    }
}

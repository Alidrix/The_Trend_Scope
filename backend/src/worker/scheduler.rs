use std::time::Duration;

use tokio::time::interval;

use super::scanner::run_scanner_cycle;

pub async fn run_scheduler(regions: Vec<String>, themes: Vec<String>, every_minutes: u64) {
    let mut ticker = interval(Duration::from_secs(every_minutes.max(1) * 60));
    loop {
        ticker.tick().await;
        run_scanner_cycle(&regions, &themes).await;
    }
}

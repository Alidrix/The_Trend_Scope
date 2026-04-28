use tracing::info;

pub async fn run_scanner_cycle(regions: &[String], themes: &[String]) {
    info!(?regions, ?themes, "worker scanner cycle executed");
}

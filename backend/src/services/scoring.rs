pub fn trend_score(views_per_hour: f64, freshness_hours: f64) -> f64 {
    let freshness = (72.0 - freshness_hours).max(1.0) / 72.0;
    (views_per_hour.max(0.0).ln_1p() * 25.0 * freshness).min(100.0)
}

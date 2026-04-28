use youtube_tiktok_backend::services::scoring::trend_score;

#[test]
fn scoring_stays_in_range() {
    let score = trend_score(10_000.0, 2.0);
    assert!((0.0..=100.0).contains(&score));
}

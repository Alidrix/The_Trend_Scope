pub fn views_per_hour(total_views: f64, age_hours: f64) -> f64 {
    total_views / age_hours.max(1.0)
}

pub fn freshness_score(age_hours: f64) -> f64 {
    if age_hours <= 6.0 {
        1.0
    } else if age_hours <= 24.0 {
        0.8
    } else if age_hours <= 72.0 {
        0.5
    } else {
        0.2
    }
}

pub fn velocity_score(vph: f64, max_vph: f64) -> f64 {
    if max_vph <= 0.0 {
        return 0.0;
    }
    (vph / max_vph).clamp(0.0, 1.0)
}

pub fn trend_score(velocity: f64, freshness: f64, engagement: f64) -> f64 {
    (velocity * 0.5 + freshness * 0.3 + engagement * 0.2) * 100.0
}

pub fn opportunity_score(trend_score: f64, saturation_score: f64) -> f64 {
    trend_score - saturation_score
}

use youtube_tiktok_backend::services::alerts::{
    alert_matches_rule, approximate_trend_score, AlertRuleMatchInput, TrendMatchInput,
};

fn base_trend() -> TrendMatchInput {
    TrendMatchInput {
        platform: "youtube".into(),
        region: "FR".into(),
        category: "business".into(),
        title: "AI for startups".into(),
        description: "Practical growth tips".into(),
        views_per_hour: 5000,
    }
}

#[test]
fn match_par_plateforme() {
    let rule = AlertRuleMatchInput {
        platform: Some("youtube".into()),
        region: None,
        category: None,
        keyword: None,
        min_views_per_hour: None,
        min_trend_score: None,
    };
    assert!(alert_matches_rule(&rule, &base_trend()));
}
#[test]
fn refuse_mauvaise_plateforme() {
    let rule = AlertRuleMatchInput {
        platform: Some("tiktok".into()),
        region: None,
        category: None,
        keyword: None,
        min_views_per_hour: None,
        min_trend_score: None,
    };
    assert!(!alert_matches_rule(&rule, &base_trend()));
}
#[test]
fn match_keyword_case_insensitive() {
    let rule = AlertRuleMatchInput {
        platform: None,
        region: None,
        category: None,
        keyword: Some("STARTUPS".into()),
        min_views_per_hour: None,
        min_trend_score: None,
    };
    assert!(alert_matches_rule(&rule, &base_trend()));
}
#[test]
fn refuse_views_trop_faible() {
    let rule = AlertRuleMatchInput {
        platform: None,
        region: None,
        category: None,
        keyword: None,
        min_views_per_hour: Some(6000),
        min_trend_score: None,
    };
    assert!(!alert_matches_rule(&rule, &base_trend()));
}
#[test]
fn approximate_score_plafonne_a_100() {
    assert_eq!(approximate_trend_score(200_000), 100.0);
}

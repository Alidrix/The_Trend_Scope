use youtube_tiktok_backend::services::stripe::{detect_plan, StripeConfig};

#[test]
fn detects_plan_from_price() {
    let cfg = StripeConfig {
        secret_key: "sk".into(),
        webhook_secret: "wh".into(),
        pro_price_id: "price_pro".into(),
        studio_price_id: "price_studio".into(),
        frontend_origin: "http://localhost".into(),
    };
    assert_eq!(detect_plan(&cfg, "price_pro"), "pro");
    assert_eq!(detect_plan(&cfg, "price_studio"), "studio");
    assert_eq!(detect_plan(&cfg, "other"), "free");
}

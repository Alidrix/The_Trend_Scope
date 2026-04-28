use youtube_tiktok_backend::services::rate_limit::key;

#[test]
fn rate_limit_key_generation() {
    assert_eq!(key("auth:login", "ip-1"), "rate_limit:auth:login:ip-1");
}

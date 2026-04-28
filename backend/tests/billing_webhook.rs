use youtube_tiktok_backend::services::stripe::{config_from_env, validate_signature};

#[test]
fn stripe_signature_rejects_invalid() {
    assert!(!validate_signature("{}", "t=1,v1=invalid", "secret"));
}

#[test]
fn config_can_be_absent_in_test() {
    let _ = config_from_env();
}

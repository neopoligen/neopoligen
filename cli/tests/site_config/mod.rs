use neopoligengine::site_config::SiteConfig;
use pretty_assertions::assert_eq;

#[test]
fn base_url_remove_trialing_slash() {
    let config = SiteConfig::mock1();
    let left = "https://www.example.com".to_string();
    let right = config.base_url().unwrap();
    assert_eq!(left, right)
}

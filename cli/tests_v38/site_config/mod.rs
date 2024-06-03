use neopoligengine::site_config::SiteConfig;
use pretty_assertions::assert_eq;

#[test]
fn base_url_remove_trialing_slash() {
    let config = SiteConfig::mock1();
    let left = "https://www.example.com".to_string();
    let right = config.base_url();
    assert_eq!(left, right)
}

#[test]
fn image_widths() {
    let config = SiteConfig::mock1();
    let left = vec![100, 180, 300, 500, 750, 1000, 1500, 2400];
    let right = config.image_widths();
    assert_eq!(left, right)
}

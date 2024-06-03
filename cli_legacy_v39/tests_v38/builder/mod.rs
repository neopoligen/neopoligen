// use neopoligengine::builder::Builder;
// use neopoligengine::site_config::SiteConfig;
use pretty_assertions::assert_eq;

#[test]
fn ping() {
    //let b = Builder::new(SiteConfig::mock1());
    let left = 1;
    let right = 1;
    assert_eq!(left, right);
}

use neopoligengine::site_v39::SiteV39;
// use pretty_assertions::assert_eq;

#[test]
fn config_getter() {
    let site = SiteV39::mock1();
    assert!(site.config().is_some());
}

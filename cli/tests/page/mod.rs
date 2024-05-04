// use neopoligengine::ast::mocks::ast_mock1;
use neopoligengine::page::Page;
// use neopoligengine::section::*;
// use neopoligengine::section_attr::SectionAttr;
// use neopoligengine::site_sections::SiteSections;
// use neopoligengine::span::*;
use neopoligengine::site_config::SiteConfigV2;
use pretty_assertions::assert_eq;
// use serde_json::Value;
use std::path::PathBuf;

#[test]
fn get_id() {
    let source_text = "-- metadata\n-- id: some-test-id".to_string();
    let config = SiteConfigV2::mock1();
    let p = Page::new(source_text, &config);
    let left = "some-test-id".to_string();
    let right = p.id.unwrap();
    assert_eq!(left, right);
}

#[test]
fn get_default_path() {
    let source_text = "-- metadata\n-- id: some-test-id".to_string();
    let config = SiteConfigV2::mock1();
    let p = Page::new(source_text, &config);
    let left = PathBuf::from("/mock/output/root/en/some-test-id/index.html");
    let right = p.output_path.unwrap();
    assert_eq!(left, right);
}

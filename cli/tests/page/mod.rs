use neopoligengine::{page::Page, site_config::SiteConfig};
use pretty_assertions::assert_eq;
use std::path::PathBuf;

#[test]
fn title_from_metadata() {
    let source_text = r#"
-- metadata
-- id: abcd1234
-- date: 2024-05-20
-- title: Some Title
"#
    .trim_start()
    .to_string();
    let source_path = PathBuf::from("/mock/root/content/test-page.neo");
    let config = SiteConfig::mock1();
    let page = Page::new(source_text, source_path, &config);
    let expect = "Some Title".to_string();
    let got = page.title_as_plain_text.unwrap();
    assert_eq!(expect, got);
}

#[test]
fn title_from_id() {
    let source_text = r#"
-- metadata
-- id: abcd1234
-- date: 2024-05-20
"#
    .trim_start()
    .to_string();
    let source_path = PathBuf::from("/mock/root/content/test-page.neo");
    let config = SiteConfig::mock1();
    let page = Page::new(source_text, source_path, &config);
    let expect = "abcd1234".to_string();
    let got = page.title_as_plain_text.unwrap();
    assert_eq!(expect, got);
}

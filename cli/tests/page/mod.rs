use neopoligengine::span::Span;
use neopoligengine::{page::Page, site_config::SiteConfig};
use pretty_assertions::assert_eq;
use std::path::PathBuf;

#[test]
fn plain_text_from_spans_basic() {
    let spans = vec![
        Span::WordPart {
            text: "alfa".to_string(),
            r#type: "word-part".to_string(),
        },
        Span::Space {
            text: " ".to_string(),
            r#type: "space".to_string(),
        },
        Span::WordPart {
            text: "bravo".to_string(),
            r#type: "word-part".to_string(),
        },
    ];
    let expect = "alfa bravo".to_string();
    let got = Page::plain_text_from_spans(&spans).unwrap();
    assert_eq!(expect, got);
}

#[test]
fn title_from_metadata() {
    let source_text = r#"
-- metadata
-- id: abcd1234
-- date: 2024-05-20
-- title: Alfa Title
"#
    .trim_start()
    .to_string();
    let source_path = PathBuf::from("/mock/root/content/test-page.neo");
    let config = SiteConfig::mock1();
    let page = Page::new(source_text, source_path, &config);
    let expect = "Alfa Title".to_string();
    let got = page.title_as_plain_text.unwrap();
    assert_eq!(expect, got);
}

#[test]
fn title_from_title_section() {
    let source_text = r#"
-- title

Bravo Title

-- metadata
-- id: abcd1234
-- date: 2024-05-20
"#
    .trim_start()
    .to_string();
    let source_path = PathBuf::from("/mock/root/content/test-page.neo");
    let config = SiteConfig::mock1();
    let page = Page::new(source_text, source_path, &config);
    let expect = "Bravo Title".to_string();
    let got = page.title_as_plain_text.unwrap();
    assert_eq!(expect, got);
}

#[test]
fn title_from_any_section() {
    let source_text = r#"
-- p 
-- title: Charlie Title

Some content

-- metadata
-- id: abcd1234
-- date: 2024-05-20
"#
    .trim_start()
    .to_string();
    let source_path = PathBuf::from("/mock/root/content/test-page.neo");
    let config = SiteConfig::mock1();
    let page = Page::new(source_text, source_path, &config);
    let expect = "Charlie Title".to_string();
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

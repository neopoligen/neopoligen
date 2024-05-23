use neopoligengine::span::Span;
use neopoligengine::{page::Page, site_config::SiteConfig};
use pretty_assertions::assert_eq;
use std::collections::BTreeMap;
use std::path::PathBuf;

#[test]
fn href_basic() {
    let source_text = r#"
-- metadata
-- id: abcd1234
-- date: 2024-05-20
-- title: Foxtrot Href Test 
"#
    .trim_start()
    .to_string();
    let source_path = PathBuf::from("/mock/root/content/test-page.neo");
    let config = SiteConfig::mock1();
    let page = Page::new(source_text, source_path, &config);
    let expect = "/en/abcd1234/?foxtrot-href-test".to_string();
    let got = page.href.unwrap();
    assert_eq!(expect, got);
}

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
fn plain_text_from_spans_recursive() {
    let spans = vec![
        Span::WordPart {
            text: "charlie".to_string(),
            r#type: "word-part".to_string(),
        },
        Span::Space {
            text: " ".to_string(),
            r#type: "space".to_string(),
        },
        Span::KnownSpan {
            attrs: BTreeMap::new(),
            flags: vec![],
            spans: vec![Span::WordPart {
                text: "delta".to_string(),
                r#type: "word-part".to_string(),
            }],
            r#type: "span".to_string(),
        },
    ];
    let expect = "charlie delta".to_string();
    let got = Page::plain_text_from_spans(&spans).unwrap();
    assert_eq!(expect, got);
}

#[test]
fn plain_text_from_spans_with_empty_vec_returns_none() {
    let spans = vec![];
    let expect = None;
    let got = Page::plain_text_from_spans(&spans);
    assert_eq!(expect, got);
}

#[test]
fn title_for_url_basic() {
    let source_text = r#"
-- metadata
-- id: abcd1234
-- date: 2024-05-20
-- title: Title For URL 
"#
    .trim_start()
    .to_string();
    let source_path = PathBuf::from("/mock/root/content/test-page.neo");
    let config = SiteConfig::mock1();
    let page = Page::new(source_text, source_path, &config);
    let expect = "title-for-url".to_string();
    let got = page.title_for_url.unwrap();
    assert_eq!(expect, got);
}

#[test]
fn title_for_url_deal_with_chars_and_multi_spaces() {
    let source_text = r#"
-- metadata
-- id: abcd1234
-- date: 2024-05-20
-- title: - Another ' URL 42 ~ title -
"#
    .trim_start()
    .to_string();
    let source_path = PathBuf::from("/mock/root/content/test-page.neo");
    let config = SiteConfig::mock1();
    let page = Page::new(source_text, source_path, &config);
    let expect = "another-url-42-title".to_string();
    let got = page.title_for_url.unwrap();
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

Bravo <<em|Title>>

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
fn title_from_first_few_words() {
    let source_text = r#"
-- p

This is the <<em|first>> few words from a paragraph
section that will end up being in the title. But,
only part of them. Not the full paragraph. 


-- metadata
-- id: abcd1234
-- date: 2024-05-20
"#
    .trim_start()
    .to_string();
    let source_path = PathBuf::from("/mock/root/content/test-page.neo");
    let config = SiteConfig::mock1();
    let page = Page::new(source_text, source_path, &config);
    let expect = "This is the first few words from a paragraph".to_string();
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

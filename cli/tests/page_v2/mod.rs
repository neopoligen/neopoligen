use minijinja::Value;
use neopoligengine::{page_filters::PageFilterOrSet, page_v2::PageV2};
use pretty_assertions::assert_eq;
use std::path::PathBuf;

#[test]
fn id_that_exists() {
    let p = PageV2::mock_1_with_ast();
    let left = Value::from("alfa1234");
    let right = p.id_v2().unwrap();
    assert_eq!(left, right);
}

#[test]
fn all_sections_basic_result_is_ok() {
    let p = PageV2::mock_1_with_ast();
    assert!(p.all_sections().is_ok());
}

#[test]
fn all_sections_except_basic_is_ok() {
    let p = PageV2::mock_1_with_ast();
    assert!(p.all_sections_except(&[Value::from("metadata")]).is_ok());
}

#[test]
fn only_sections_basic_is_ok() {
    let p = PageV2::mock_1_with_ast();
    assert!(p.only_sections(&[Value::from("title")]).is_ok());
}

// #[test]
// fn date_basic() {
//     let p = PageV2::mock_1_with_ast();
//     let left = "2024-05-20T10:11:12-04:00".to_string();
//     let right = p
//         .date()
//         .unwrap()
//         .to_rfc3339_opts(chrono::SecondsFormat::Secs, false);
//     assert_eq!(left, right)
// }

// #[test]
// fn default_status() {
//     let p = PageV2::mock_1_with_ast();
//     let left = "published".to_string();
//     let right = p.status().unwrap();
//     assert_eq!(left, right)
// }

// #[test]
// fn defatul_type() {
//     let p = PageV2::mock_1_with_ast();
//     let left = "post".to_string();
//     let right = p.r#type().unwrap();
//     assert_eq!(left, right)
// }

// #[test]
// fn explicit_status() {
//     let p = PageV2::mock_3_bookmark_section();
//     let left = "draft".to_string();
//     let right = p.status().unwrap();
//     assert_eq!(left, right)
// }

// #[test]
// fn explicit_type() {
//     let p = PageV2::mock_2_home_page();
//     let left = "home-page".to_string();
//     let right = p.r#type().unwrap();
//     assert_eq!(left, right)
// }

// #[test]
// fn feed_date_basic() {
//     let p = PageV2::mock_1_with_ast();
//     let left = "2024-05-20T10:11:12-04:00".to_string();
//     let right = p.feed_date().unwrap();
//     assert_eq!(left, right)
// }

// #[test]
// fn filter_test_basic() {
//     let p = PageV2::mock_1_with_ast();
//     let filters = PageFilterOrSet::mock1_status_published();
//     let left = true;
//     let right = p.passes(&filters);
//     assert_eq!(left, right);
// }

// #[test]
// fn format_created_date_basic() {
//     let p = PageV2::mock_1_with_ast();
//     let left = "May 2024".to_string();
//     let right = p.format_created_date("%B %Y").unwrap();
//     assert_eq!(left, right)
// }

// #[test]
// fn format_date_from_created() {
//     let p = PageV2::mock_1_with_ast();
//     let left = "May 2024".to_string();
//     let right = p.format_date("%B %Y").unwrap();
//     assert_eq!(left, right)
// }

// #[test]
// fn format_date_from_updated() {
//     let p = PageV2::mock_3_bookmark_section();
//     let left = "February 2022".to_string();
//     let right = p.format_date("%B %Y").unwrap();
//     assert_eq!(left, right)
// }

// #[test]
// fn format_updated_date_none() {
//     let p = PageV2::mock_1_with_ast();
//     let left = None;
//     let right = p.format_updated_date("%B %Y");
//     assert_eq!(left, right)
// }

// #[test]
// fn get_metadata_attr_basic() {
//     let p = PageV2::mock_6_url_title_parsing();
//     let left = "2023-01-02T14:15:16-04:00".to_string();
//     let right = p.get_metadata_attr("created").unwrap();
//     assert_eq!(left, right)
// }

// #[test]
// fn href_basic() {
//     let p = PageV2::mock_6_url_title_parsing();
//     let left = "/en/foxtrot1/?another-url-42-title".to_string();
//     let right = p.href().unwrap();
//     assert_eq!(left, right)
// }

// TODO: href when there's a metadata path that has an HTML file
// TODO: href when there's a metadata path without an HTML file

// #[test]
// fn id_basic() {
//     let p = PageV2::mock_1_with_ast();
//     let left = "alfa1234".to_string();
//     let right = p.id().unwrap();
//     assert_eq!(left, right)
// }

// #[test]
// fn og_image_basic() {
//     let p = PageV2::mock_1_with_ast();
//     let left = "https://www.example.com/og-images/alfa1234.jpg".to_string();
//     let right = p.og_image().unwrap();
//     assert_eq!(left, right)
// }

// #[test]
// fn permalink_basic() {
//     let p = PageV2::mock_1_with_ast();
//     let left = "https://www.example.com/en/alfa1234/?mock-file-1-with-ast".to_string();
//     let right = p.permalink().unwrap();
//     assert_eq!(left, right)
// }

// #[test]
// fn rel_file_path_basic() {
//     let p = PageV2::mock_1_with_ast();
//     let left = PathBuf::from("en/alfa1234/index.html");
//     let right = p.rel_file_path().unwrap();
//     assert_eq!(left, right)
// }

// #[test]
// fn rel_file_path_for_home_page() {
//     let p = PageV2::mock_2_home_page();
//     let left = PathBuf::from("index.html");
//     let right = p.rel_file_path().unwrap();
//     assert_eq!(left, right)
// }

// #[test]
// fn title_as_plain_text_from_title_section() {
//     let p = PageV2::mock_1_with_ast();
//     let left = "Mock File 1 With AST".to_string();
//     let right = p.title_as_plain_text().unwrap();
//     assert_eq!(left, right)
// }

// #[test]
// fn title_as_plain_text_from_metadata() {
//     let p = PageV2::mock_2_home_page();
//     let left = "Title From Metadata".to_string();
//     let right = p.title_as_plain_text().unwrap();
//     assert_eq!(left, right)
// }

// #[test]
// fn title_as_plain_text_from_any_section() {
//     let p = PageV2::mock_3_bookmark_section();
//     let left = "Title From Bookmark Attribute".to_string();
//     let right = p.title_as_plain_text().unwrap();
//     assert_eq!(left, right)
// }

// #[test]
// fn title_as_plain_text_from_first_few_words() {
//     let p = PageV2::mock_4_title_from_text();
//     let left = "This is to test the title that comes in".to_string();
//     let right = p.title_as_plain_text().unwrap();
//     assert_eq!(left, right)
// }

// #[test]
// fn title_as_plain_text_from_id() {
//     let p = PageV2::mock_5_no_title();
//     let left = "echo1234".to_string();
//     let right = p.title_as_plain_text().unwrap();
//     assert_eq!(left, right)
// }

// #[test]
// fn title_for_url_basic() {
//     let p = PageV2::mock_1_with_ast();
//     let left = "mock-file-1-with-ast".to_string();
//     let right = p.title_for_url().unwrap();
//     assert_eq!(left, right)
// }

// #[test]
// fn title_for_url_deal_with_chars_and_multi_spaces() {
//     let p = PageV2::mock_6_url_title_parsing();
//     let left = "another-url-42-title".to_string();
//     let right = p.title_for_url().unwrap();
//     assert_eq!(left, right)
// }

// #[test]
// fn uuid_basic() {
//     let p = PageV2::mock_1_with_ast();
//     let left = "82ad7694-cfb5-5a3e-b025-3f1ecae6adbc".to_string();
//     let right = p.uuid().unwrap();
//     assert_eq!(left, right)
// }

// The below tests are from the original page instance
//
// use neopoligengine::span::Span;
// use neopoligengine::{page::Page, site_config::SiteConfig};
// use pretty_assertions::assert_eq;
// use std::collections::BTreeMap;
// use std::path::PathBuf;

// DEPRECATED: Remove when this is moved to page_v2
// #[test]
// fn href_basic() {
//     let source_text = r#"
// -- metadata
// -- id: abcd1234
// -- date: 2024-05-20
// -- title: Foxtrot Href Test
// "#
//     .trim_start()
//     .to_string();
//     let source_path = PathBuf::from("/mock/root/content/test-page.neo");
//     let config = SiteConfig::mock1();
//     let page = Page::new(source_text, source_path, &config);
//     let expect = "/en/abcd1234/?foxtrot-href-test".to_string();
//     let got = page.href.unwrap();
//     assert_eq!(expect, got);
// }

// DEPRECATED: Remove when this is moved to page_v2
// #[test]
// fn href_from_path() {
//     let source_text = r#"
// -- metadata
// -- id: abcd1234
// -- date: 2024-05-20
// -- path: /
// -- title: This Should Not Be In The URL
// "#
//     .trim_start()
//     .to_string();
//     let source_path = PathBuf::from("/mock/root/content/test-page.neo");
//     let config = SiteConfig::mock1();
//     let page = Page::new(source_text, source_path, &config);
//     let expect = "/".to_string();
//     let got = page.href.unwrap();
//     assert_eq!(expect, got);
// }

// DEPRECATED: Remove when this is moved to page_v2
// #[test]
// fn plain_text_from_spans_basic() {
//     let spans = vec![
//         Span::WordPart {
//             text: "alfa".to_string(),
//             r#type: "word-part".to_string(),
//         },
//         Span::Space {
//             text: " ".to_string(),
//             r#type: "space".to_string(),
//         },
//         Span::WordPart {
//             text: "bravo".to_string(),
//             r#type: "word-part".to_string(),
//         },
//     ];
//     let expect = "alfa bravo".to_string();
//     let got = Page::plain_text_from_spans(&spans).unwrap();
//     assert_eq!(expect, got);
// }

// DEPRECATED: Remove when this is moved to page_v2
// #[test]
// fn plain_text_from_spans_recursive() {
//     let spans = vec![
//         Span::WordPart {
//             text: "charlie".to_string(),
//             r#type: "word-part".to_string(),
//         },
//         Span::Space {
//             text: " ".to_string(),
//             r#type: "space".to_string(),
//         },
//         Span::KnownSpan {
//             attrs: BTreeMap::new(),
//             flags: vec![],
//             spans: vec![Span::WordPart {
//                 text: "delta".to_string(),
//                 r#type: "word-part".to_string(),
//             }],
//             r#type: "span".to_string(),
//         },
//     ];
//     let expect = "charlie delta".to_string();
//     let got = Page::plain_text_from_spans(&spans).unwrap();
//     assert_eq!(expect, got);
// }

// DEPRECATED: Remove when this is moved to page_v2
// #[test]
// fn plain_text_from_spans_with_empty_vec_returns_none() {
//     let spans = vec![];
//     let expect = None;
//     let got = Page::plain_text_from_spans(&spans);
//     assert_eq!(expect, got);
// }

// DEPRECATED: Remove when this is moved to page_v2
// #[test]
// fn title_for_url_basic() {
//     let source_text = r#"
// -- metadata
// -- id: abcd1234
// -- date: 2024-05-20
// -- title: Title For URL
// "#
//     .trim_start()
//     .to_string();
//     let source_path = PathBuf::from("/mock/root/content/test-page.neo");
//     let config = SiteConfig::mock1();
//     let page = Page::new(source_text, source_path, &config);
//     let expect = "title-for-url".to_string();
//     let got = page.title_for_url.unwrap();
//     assert_eq!(expect, got);
// }

// DEPRECATED: Remove when this is moved to page_v2
// #[test]
// fn title_for_url_deal_with_chars_and_multi_spaces() {
//     let source_text = r#"
// -- metadata
// -- id: abcd1234
// -- date: 2024-05-20
// -- title: - Another ' URL 42 ~ title -
// "#
//     .trim_start()
//     .to_string();
//     let source_path = PathBuf::from("/mock/root/content/test-page.neo");
//     let config = SiteConfig::mock1();
//     let page = Page::new(source_text, source_path, &config);
//     let expect = "another-url-42-title".to_string();
//     let got = page.title_for_url.unwrap();
//     assert_eq!(expect, got);
// }

// DEPRECATED: Remove when this is moved to page_v2
// #[test]
// fn title_from_metadata() {
//     let source_text = r#"
// -- metadata
// -- id: abcd1234
// -- date: 2024-05-20
// -- title: Alfa Title
// "#
//     .trim_start()
//     .to_string();
//     let source_path = PathBuf::from("/mock/root/content/test-page.neo");
//     let config = SiteConfig::mock1();
//     let page = Page::new(source_text, source_path, &config);
//     let expect = "Alfa Title".to_string();
//     let got = page.title_as_plain_text.unwrap();
//     assert_eq!(expect, got);
// }

// DEPRECATED: Remove when this is moved to page_v2
// #[test]
// fn title_from_title_section() {
//     let source_text = r#"
// -- title

// Bravo <<em|Title>>

// -- metadata
// -- id: abcd1234
// -- date: 2024-05-20
// "#
//     .trim_start()
//     .to_string();
//     let source_path = PathBuf::from("/mock/root/content/test-page.neo");
//     let config = SiteConfig::mock1();
//     let page = Page::new(source_text, source_path, &config);
//     let expect = "Bravo Title".to_string();
//     let got = page.title_as_plain_text.unwrap();
//     assert_eq!(expect, got);
// }

// DEPRECATED: Remove when this is moved to page_v2
// #[test]
// fn title_from_any_section() {
//     let source_text = r#"
// -- p
// -- title: Charlie Title

// Some content

// -- metadata
// -- id: abcd1234
// -- date: 2024-05-20
// "#
//     .trim_start()
//     .to_string();
//     let source_path = PathBuf::from("/mock/root/content/test-page.neo");
//     let config = SiteConfig::mock1();
//     let page = Page::new(source_text, source_path, &config);
//     let expect = "Charlie Title".to_string();
//     let got = page.title_as_plain_text.unwrap();
//     assert_eq!(expect, got);
// }

// DEPRECATED: Remove when this is moved to page_v2
// #[test]
// fn title_from_first_few_words() {
//     let source_text = r#"
// -- p

// This is the <<em|first>> few words from a paragraph
// section that will end up being in the title. But,
// only part of them. Not the full paragraph.

// -- metadata
// -- id: abcd1234
// -- date: 2024-05-20
// "#
//     .trim_start()
//     .to_string();
//     let source_path = PathBuf::from("/mock/root/content/test-page.neo");
//     let config = SiteConfig::mock1();
//     let page = Page::new(source_text, source_path, &config);
//     let expect = "This is the first few words from a paragraph".to_string();
//     let got = page.title_as_plain_text.unwrap();
//     assert_eq!(expect, got);
// }

// DEPRECATED: Remove when this is moved to page_v2
// #[test]
// fn title_from_id() {
//     let source_text = r#"
// -- metadata
// -- id: abcd1234
// -- date: 2024-05-20
// "#
//     .trim_start()
//     .to_string();
//     let source_path = PathBuf::from("/mock/root/content/test-page.neo");
//     let config = SiteConfig::mock1();
//     let page = Page::new(source_text, source_path, &config);
//     let expect = "abcd1234".to_string();
//     let got = page.title_as_plain_text.unwrap();
//     assert_eq!(expect, got);
// }

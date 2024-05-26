use neopoligengine::{page_v2::PageV2, site_config::SiteConfig};
use pretty_assertions::assert_eq;
use std::path::PathBuf;

#[test]
fn id_basic() {
    let p = PageV2::mock_1_with_ast();
    let left = "abcd1234".to_string();
    let right = p.id().unwrap();
    assert_eq!(left, right)
}

#[test]
fn rel_file_path_basic() {
    let p = PageV2::mock_1_with_ast();
    let left = PathBuf::from("en/abcd1234/index.html");
    let right = p.rel_file_path().unwrap();
    assert_eq!(left, right)
}

#[test]
fn rel_file_path_for_home_page() {
    let p = PageV2::mock_2_home_page();
    let left = PathBuf::from("index.html");
    let right = p.rel_file_path().unwrap();
    assert_eq!(left, right)
}

#[test]
fn title_as_plain_text_from_title_section() {
    let p = PageV2::mock_1_with_ast();
    let left = "Mock File 1 With AST".to_string();
    let right = p.title_as_plain_text().unwrap();
    assert_eq!(left, right)
}

#[test]
fn title_as_plain_text_from_metadata() {
    let p = PageV2::mock_2_home_page();
    let left = "Title From Metadata".to_string();
    let right = p.title_as_plain_text().unwrap();
    assert_eq!(left, right)
}

#[test]
fn title_as_plain_text_from_any_section() {
    let p = PageV2::mock_3_bookmark_section();
    let left = "Title From Bookmark Attribute".to_string();
    let right = p.title_as_plain_text().unwrap();
    assert_eq!(left, right)
}

#[test]
fn title_as_plain_text_from_first_few_words() {
    let p = PageV2::mock_4_title_from_text();
    let left = "This is to test the title that comes in".to_string();
    let right = p.title_as_plain_text().unwrap();
    assert_eq!(left, right)
}

#[test]
fn title_as_plain_text_from_id() {
    let p = PageV2::mock_5_no_title();
    let left = "echo8171".to_string();
    let right = p.title_as_plain_text().unwrap();
    assert_eq!(left, right)
}

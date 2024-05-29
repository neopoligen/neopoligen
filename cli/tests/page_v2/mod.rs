use neopoligengine::page_v2::PageV2;
use pretty_assertions::assert_eq;
use std::path::PathBuf;

#[test]
fn defatul_status() {
    let p = PageV2::mock_1_with_ast();
    let left = "published".to_string();
    let right = p.status().unwrap();
    assert_eq!(left, right)
}

#[test]
fn defatul_type() {
    let p = PageV2::mock_1_with_ast();
    let left = "post".to_string();
    let right = p.r#type().unwrap();
    assert_eq!(left, right)
}

#[test]
fn explicit_status() {
    let p = PageV2::mock_3_bookmark_section();
    let left = "draft".to_string();
    let right = p.status().unwrap();
    assert_eq!(left, right)
}

#[test]
fn explicit_type() {
    let p = PageV2::mock_2_home_page();
    let left = "home-page".to_string();
    let right = p.r#type().unwrap();
    assert_eq!(left, right)
}

#[test]
fn format_created_date_basic() {
    let p = PageV2::mock_1_with_ast();
    let left = "May 2024".to_string();
    let right = p.format_created_date("%B %Y").unwrap();
    assert_eq!(left, right)
}

#[test]
fn format_date_from_created() {
    let p = PageV2::mock_1_with_ast();
    let left = "May 2024".to_string();
    let right = p.format_date("%B %Y").unwrap();
    assert_eq!(left, right)
}

#[test]
fn format_date_from_updated() {
    let p = PageV2::mock_2_home_page();
    let left = "October 2022".to_string();
    let right = p.format_date("%B %Y").unwrap();
    assert_eq!(left, right)
}

#[test]
fn format_updated_date_none() {
    let p = PageV2::mock_1_with_ast();
    let left = None;
    let right = p.format_updated_date("%B %Y");
    assert_eq!(left, right)
}

#[test]
fn get_metadata_attr_basic() {
    let p = PageV2::mock_6_url_title_parsing();
    let left = "2023-01-02".to_string();
    let right = p.get_metadata_attr("created").unwrap();
    assert_eq!(left, right)
}

#[test]
fn href_basic() {
    let p = PageV2::mock_6_url_title_parsing();
    let left = "/en/foxtrot1/?another-url-42-title".to_string();
    let right = p.href().unwrap();
    assert_eq!(left, right)
}

// TODO: href when there's a metadata path that has an HTML file
// TODO: href when there's a metadata path without an HTML file

#[test]
fn id_basic() {
    let p = PageV2::mock_1_with_ast();
    let left = "alfa1234".to_string();
    let right = p.id().unwrap();
    assert_eq!(left, right)
}

#[test]
fn og_image_basic() {
    let p = PageV2::mock_1_with_ast();
    let left = "https://www.example.com/og-images/alfa1234.jpg".to_string();
    let right = p.og_image().unwrap();
    assert_eq!(left, right)
}

#[test]
fn permalink_basic() {
    let p = PageV2::mock_1_with_ast();
    let left = "https://www.example.com/en/alfa1234/?mock-file-1-with-ast".to_string();
    let right = p.permalink().unwrap();
    assert_eq!(left, right)
}

#[test]
fn rel_file_path_basic() {
    let p = PageV2::mock_1_with_ast();
    let left = PathBuf::from("en/alfa1234/index.html");
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
    let left = "echo1234".to_string();
    let right = p.title_as_plain_text().unwrap();
    assert_eq!(left, right)
}

#[test]
fn title_for_url_basic() {
    let p = PageV2::mock_1_with_ast();
    let left = "mock-file-1-with-ast".to_string();
    let right = p.title_for_url().unwrap();
    assert_eq!(left, right)
}

#[test]
fn title_for_url_deal_with_chars_and_multi_spaces() {
    let p = PageV2::mock_6_url_title_parsing();
    let left = "another-url-42-title".to_string();
    let right = p.title_for_url().unwrap();
    assert_eq!(left, right)
}

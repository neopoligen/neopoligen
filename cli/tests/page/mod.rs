use neopoligengine::page::Page;
use neopoligengine::site_config::SiteConfigV2;
use pretty_assertions::assert_eq;
use rstest::rstest;
use std::path::PathBuf;

#[test]
fn get_id() {
    let source_text = "-- metadata\n-- id: some-test-id".to_string();
    let source_path = PathBuf::from("/mock/root/content/some-folder/1234.neo");
    let config = SiteConfigV2::mock1();
    let p = Page::new(source_text, source_path, &config);
    let left = "some-test-id".to_string();
    let right = p.id.unwrap();
    assert_eq!(left, right);
}

#[test]
fn watch_for_no_id() {
    let source_text = "-- title\ntest with no id".to_string();
    let source_path = PathBuf::from("/mock/root/content/some-folder/1234.neo");
    let config = SiteConfigV2::mock1();
    let p = Page::new(source_text, source_path, &config);
    let left = PathBuf::from("/mock/root/status/errors/some-folder/1234.txt");
    let right = p.output_path.unwrap();
    assert_eq!(left, right);
}

// #[test]
// fn get_default_path() {
//     let source_text = "-- metadata\n-- id: some-test-id".to_string();
//     let source_path = PathBuf::from("/mock/root/content/some-folder/1234.neo");
//     let config = SiteConfigV2::mock1();
//     let p = Page::new(source_text, source_path, &config);
//     let left = PathBuf::from("/mock/root/docs/en/some-test-id/index.html");
//     let right = p.output_path.unwrap();
//     assert_eq!(left, right);
// }

#[rstest]
#[case(
    "-- metadata\n-- id: some-test-id",
    "/mock/root/docs/en/some-test-id/index.html"
)]
#[case(
    "-- metadata\n-- id: test-id\n-- path: /some-path",
    "/mock/root/docs/some-path/index.html"
)]
fn run_test(#[case] input: &str, #[case] expected: &str) {
    let source_path = PathBuf::from("/mock/root/content/some-folder/1234.neo");
    let config = SiteConfigV2::mock1();
    let p = Page::new(input.to_string(), source_path, &config);
    let left = PathBuf::from(expected);
    let right = p.output_path.unwrap();
    assert_eq!(left, right);
}

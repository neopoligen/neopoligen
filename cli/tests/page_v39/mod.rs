use neopoligengine::page_v39::PageV39;
use pretty_assertions::assert_eq;
use std::path::PathBuf;

#[test]
fn rel_output_path_basic() {
    let p = PageV39::mock_1_20240101_basic_page();
    let left = PathBuf::from("en/20240101alfa/index.html");
    let right = p.rel_output_path().unwrap();
    assert_eq!(left, right);
}

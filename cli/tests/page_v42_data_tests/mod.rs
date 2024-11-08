use neopoligengine::page_v42::PageV42b;
use pretty_assertions::assert_eq;
use std::path::PathBuf;

#[test]
fn solo_id_basic_check() {
    let page = PageV42b::test_from_sample_file("001-pagedata-only.neo");
    let left = Some("001-pagedata-only.neo".to_string());
    let right = page.id;
    assert_eq!(left, right);
}

// #[test]
// fn solo_rel_output_path_basic_check() {
//     let page = PageV42b::test_from_sample_file("001-pagedata-only.neo");
//     let left = Some(PathBuf::from("en/001-pagedata-only.neo/index.html"));
//     let right = page.rel_output_path;
//     assert_eq!(left, right);
// }


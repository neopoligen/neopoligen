use neopoligengine::page_v39::PageV39;
use neopoligengine::section_v39::SectionV39;
use pretty_assertions::assert_eq;
use std::path::PathBuf;

#[test]
fn error_on_invalid_ast() {
    let p = PageV39::mock_invalid_ast_1();
    assert_eq!(p.errors.len(), 1);
}

#[test]
fn id_basic() {
    let p = PageV39::mock_1_20240101_basic_page();
    let left = "20240101alfa".to_string();
    let right = p.id().unwrap();
    assert_eq!(left, right);
}

#[test]
fn rel_output_path_basic() {
    let p = PageV39::mock_1_20240101_basic_page();
    let left = PathBuf::from("en/20240101alfa/index.html");
    let right = p.rel_output_path().unwrap();
    assert_eq!(left, right);
}

#[test]
fn all_sections() {
    let p = PageV39::mock_1_20240101_basic_page();
    let sections: Vec<SectionV39> = p.all_sections().unwrap();
    assert_eq!(sections.len(), 2);
}

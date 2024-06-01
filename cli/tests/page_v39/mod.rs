use neopoligengine::page_v39::PageV39;
use pretty_assertions::assert_eq;
use std::path::PathBuf;

#[test]
fn all_sections() {
    let p = PageV39::mock_1_20240101_basic_page();
    dbg!(p.all_sections().unwrap().kind());
    //let sections:  = p.all_sections().unwrap();
    //assert_eq!(sections.len(), 2);
}

#[test]
fn get_metadata_attr() {
    let p = PageV39::mock_1_20240101_basic_page();
    let left = "20240101alfa1234";
    let right = p.get_metadata_attr("id").unwrap();
    assert_eq!(left, right);
}

#[test]
fn error_on_invalid_ast() {
    let p = PageV39::mock_invalid_ast_1();
    assert_eq!(p.errors.len(), 1);
}

#[test]
fn id_basic() {
    let p = PageV39::mock_1_20240101_basic_page();
    let left = "20240101alfa1234".to_string();
    let right = p.id().unwrap();
    assert_eq!(left, right);
}

#[test]
fn rel_output_path_basic() {
    let p = PageV39::mock_1_20240101_basic_page();
    let left = PathBuf::from("en/20240101alfa1234/index.html");
    let right = p.rel_output_path().unwrap();
    assert_eq!(left, right);
}

#[test]
fn type_defaults_to_post() {
    let p = PageV39::mock_1_20240101_basic_page();
    let left = "post".to_string();
    let right = p.r#type().unwrap();
    assert_eq!(left, right);
}

#[test]
#[ignore]
fn type_from_metadata() {
    let p = PageV39::mock_1_20240101_basic_page();
    let left = "example".to_string();
    let right = p.r#type().unwrap();
    assert_eq!(left, right);
}

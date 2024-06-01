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
fn rel_output_path_default_to_lang_id() {
    let p = PageV39::mock_1_20240101_basic_page();
    let left = PathBuf::from("en/20240101alfa1234/index.html");
    let right = p.rel_output_path().unwrap();
    assert_eq!(left, right);
}

#[test]
#[ignore]
fn rel_output_path_from_metadata_with_no_extension() {
    let p = PageV39::mock_2_20240102_with_type_and_status();
    let left = PathBuf::from("custom-path/index.html");
    let right = p.rel_output_path().unwrap();
    assert_eq!(left, right);
}

#[test]
fn rel_output_path_scrubber_root_path() {
    let p = PageV39::mock_1_20240101_basic_page();
    let source = "/";
    let left = PathBuf::from("index.html");
    let right = p.rel_output_path_scrubber(source).unwrap();
    assert_eq!(left, right);
}

#[test]
fn rel_output_path_scrubber_with_dir_name() {
    let p = PageV39::mock_1_20240101_basic_page();
    let source = "/alfa-path";
    let left = PathBuf::from("alfa-path/index.html");
    let right = p.rel_output_path_scrubber(source).unwrap();
    assert_eq!(left, right);
}

#[test]
fn rel_output_path_scrubber_with_file_name() {
    let p = PageV39::mock_1_20240101_basic_page();
    let source = "/bravo.txt";
    let left = PathBuf::from("bravo.txt");
    let right = p.rel_output_path_scrubber(source).unwrap();
    assert_eq!(left, right);
}

#[test]
fn status_defaults_to_published() {
    let p = PageV39::mock_1_20240101_basic_page();
    let left = "published".to_string();
    let right = p.status().unwrap();
    assert_eq!(left, right);
}

#[test]
fn status_from_metadata() {
    let p = PageV39::mock_2_20240102_with_type_and_status();
    let left = "draft".to_string();
    let right = p.status().unwrap();
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
fn type_from_metadata() {
    let p = PageV39::mock_2_20240102_with_type_and_status();
    let left = "example".to_string();
    let right = p.r#type().unwrap();
    assert_eq!(left, right);
}

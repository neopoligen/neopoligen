use neopoligengine::page_v42::PageV42b;
use pretty_assertions::assert_eq;
use std::path::PathBuf;

#[test]
fn id_basic_test() {
    let page = PageV42b::test_from_sample_file("001-only-page-section.neo");
    let left = Some("001-pagedata-only.neo".to_string());
    let right = page.id;
    assert_eq!(left, right);
}

#[test]
fn id_via_fallback_test() {
    let page = PageV42b::test_from_sample_file("002-fallback-id.neo");
    let left = Some("002-fallback-id.neo".to_string());
    let right = page.id;
    assert_eq!(left, right);
}

#[test]
fn no_id_in_page() {
    let page = PageV42b::test_from_sample_file("003-no-id-available.neo");
    let left = None;
    let right = page.id;
    assert_eq!(left, right);
}

#[test]
fn rel_output_path_basic_from_id_test() {
    let page = PageV42b::test_from_sample_file("001-only-page-section.neo");
    let left = Some(PathBuf::from(
        "en/001-pagedata-only.neo/index.html".to_string(),
    ));
    let right = page.rel_output_path;
    assert_eq!(left, right);
}

#[test]
fn rel_output_path_basic_from_custom_path_with_file() {
    let page = PageV42b::test_from_sample_file("004-homepage-path-with-file.neo");
    let left = Some(PathBuf::from("index.html".to_string()));
    let right = page.rel_output_path;
    assert_eq!(left, right);
}

#[test]
fn rel_output_path_basic_from_custom_path_without_file() {
    let page = PageV42b::test_from_sample_file("005-homepage-path-without-file.neo");
    let left = Some(PathBuf::from("index.html".to_string()));
    let right = page.rel_output_path;
    assert_eq!(left, right);
}

#[test]
fn rel_output_path_basic_from_custom_path_not_homepage_without_file() {
    let page = PageV42b::test_from_sample_file("006-custom-path-without-file.neo");
    let left = Some(PathBuf::from("some/path/index.html".to_string()));
    let right = page.rel_output_path;
    assert_eq!(left, right);
}

#[test]
fn rel_output_path_basic_from_custom_path_not_homepage_without_slash() {
    let page = PageV42b::test_from_sample_file("007-custom-path-no-slash.neo");
    let left = Some(PathBuf::from("custom/path/alfa/index.html".to_string()));
    let right = page.rel_output_path;
    assert_eq!(left, right);
}

#[test]
fn rel_output_path_basic_from_custom_path_with_non_html_extension() {
    let page = PageV42b::test_from_sample_file("008-custom-txt-path.neo");
    let left = Some(PathBuf::from("somewhere/file.txt".to_string()));
    let right = page.rel_output_path;
    assert_eq!(left, right);
}

#[test]
fn solo_href_basic_test() {
    let page = PageV42b::test_from_sample_file("001-only-page-section.neo");
    let left = Some("/en/001-pagedata-only.neo/".to_string());
    let right = page.href();
    assert_eq!(left, right);
}

// #[test]
// fn rel_output_path_basic_check() {
//     let page = PageV42b::test_from_sample_file("001-pagedata-only.neo");
//     let left = Some(PathBuf::from("en/001-pagedata-only.neo/index.html"));
//     let right = page.rel_output_path;
//     assert_eq!(left, right);
// }

// Tests copied over to check

// #[test]
// fn rel_output_path_basic_check() {
//     let page = PageV42b::test_from_sample_file("001-pagedata-only.neo");
//     let left = Some(PathBuf::from("en/001-pagedata-only.neo/index.html"));
//     let right = page.rel_output_path;
//     assert_eq!(left, right);
// }

// #[test]
// fn id_basic_check() {
//     let page = PageV42b::test_from_sample_file("001-pagedata-only.neo");
//     let left = "001-pagedata-only.neo".to_string();
//     let right = page.id.unwrap();
//     assert_eq!(left, right);
// }

// #[test]
// fn title_from_pagedata_id() {
//     let page = PageV42b::test_from_sample_file("001-pagedata-only.neo");
//     let left = Value::from_serialize(vec![SpanV42::PlainText(PlainTextSpan {
//         text: "001-pagedata-only.neo".to_string(),
//     })]);
//     let right = page.title().unwrap();
//     assert_eq!(left, right);
// }

// #[test]
// fn title_from_pagedata_title() {
//     let page = PageV42b::test_from_sample_file("005-title-in-pagedata.neo");
//     let left = Value::from_serialize(vec![SpanV42::PlainText(PlainTextSpan {
//         text: "Title In Pagedata".to_string(),
//     })]);
//     let right = page.title().unwrap();
//     assert_eq!(left, right);
// }

// #[test]
// fn title_from_title_section() {
//     let page = PageV42b::test_from_sample_file("002-basic-sample.neo");
//     let left = Value::from_serialize(vec![SpanV42::PlainText(PlainTextSpan {
//         text: "Basic Sample".to_string(),
//     })]);
//     let right = page.title().unwrap();
//     assert_eq!(left, right);
// }

// #[test]
// #[ignore] // Reminder: This isn't currently needed since the page won't have rendered
// fn title_from_invalid_page() {
//     let page = PageV42b::test_from_sample_file("006-unclosed-start-section.neo");
//     let left = Value::from_serialize(vec![SpanV42::PlainText(PlainTextSpan {
//         text: "06-unclosed-start-section.neo".to_string(),
//     })]);
//     let right = page.title().unwrap();
//     assert_eq!(left, right);
// }

// #[test]
// fn default_html_file_system_relative_path_from_id() {
//     let page = PageV42::test_from_sample_file("001-pagedata-only.neo");
//     let left = PathBuf::from("en/001-pagedata-only.neo/index.html");
//     let right = page.html_output_path.unwrap();
//     assert_eq!(left, right);
// }

// #[test]
// fn custom_path_in_pagedata() {
//     let page = PageV42::test_from_sample_file("003-custom-href-in-pagedata.neo");
//     let left = PathBuf::from("index.html");
//     let right = page.html_output_path.unwrap();
//     assert_eq!(left, right);
// }

// #[test]
// fn href_from_id_default() {
//     let page = PageV42::test_from_sample_file("001-pagedata-only.neo");
//     let left = Value::from("/en/001-pagedata-only.neo/".to_string());
//     let right = page.href().unwrap();
//     assert_eq!(left, right);
// }

// #[test]
// fn href_from_pagedata() {
//     let page = PageV42::test_from_sample_file("003-custom-href-in-pagedata.neo");
//     let left = Value::from("/".to_string());
//     let right = page.href().unwrap();
//     assert_eq!(left, right);
// }

// #[test]
// fn href_for_non_index_html_file() {
//     let page = PageV42::test_from_sample_file("009-non-index-custom-href.neo");
//     let left = Value::from("/somewhere/custom.html".to_string());
//     let right = page.href().unwrap();
//     assert_eq!(left, right);
// }

//

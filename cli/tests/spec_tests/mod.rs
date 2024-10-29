use include_dir::{include_dir, Dir};
use neopoligengine::helpers::get_include_str_file;
use neopoligengine::section_v42::SectionV42;
use neopoligengine::span_v42::named_span::named_span;
use neopoligengine::{page_v42::PageV42, site_config::SiteConfigV42};
use serde::Deserialize;
use std::path::PathBuf;

const TEST_FILES: Dir = include_dir!("tests/spec_tests/tests");

#[derive(Clone, Debug)]
pub struct TestItem {
    description: String,
    source: String,
}

fn load_tests(file_name: &str) -> (Vec<TestItem>, Vec<TestItem>) {
    let source = get_include_str_file(TEST_FILES, file_name);
    let path = PathBuf::from("mock/path.neo");
    let site_config = SiteConfigV42::config_for_spec_tests();
    let page = PageV42::new_from_str(&source, &path, &site_config).unwrap();
    if let Some(data) = page
        .raw_ast
        .iter()
        .filter_map(|section| match section {
            SectionV42::Json5(sec) => Some(sec.clone()),
            _ => None,
        })
        .find_map(|section| match section.kind.clone().as_str() {
            "neopoligen-spec-test" => Some(section.data),
            _ => None,
        })
    {
        let v: serde_json::Value = serde_json::Value::deserialize(data).unwrap();
        let valid_tests = &v.as_object().unwrap()["tests"]["valid"]
            .as_array()
            .unwrap()
            .iter()
            .map(|obj| TestItem {
                description: obj["description"].as_str().unwrap().to_string(),
                source: obj["source"].as_str().unwrap().to_string(),
            })
            .collect::<Vec<TestItem>>();
        let invalid_tests = &v.as_object().unwrap()["tests"]["invalid"]
            .as_array()
            .unwrap()
            .iter()
            .map(|obj| TestItem {
                description: obj["description"].as_str().unwrap().to_string(),
                source: obj["source"].as_str().unwrap().to_string(),
            })
            .collect::<Vec<TestItem>>();
        (valid_tests.to_vec(), invalid_tests.to_vec())
    } else {
        (vec![], vec![])
    }
}

#[test]
fn solo_test_named_spans() {
    let (valid, invalid) = load_tests("named-spans.neo");
    valid.iter().for_each(|ti| {
        let result = named_span(&ti.source);
        if result.is_ok() {
            assert!(true)
        } else {
            dbg!(&ti.description);
            assert!(false);
        }
    });
    invalid.iter().for_each(|ti| {
        let result = named_span(&ti.source);
        if result.is_err() {
            assert!(true)
        } else {
            dbg!(&ti.description);
            assert!(false);
        }
    });
}

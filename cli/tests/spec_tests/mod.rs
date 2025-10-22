// TODO: Deprecate in favor of tests/span_v42_tests
//
//
use include_dir::{include_dir, Dir};
// use minijinja::Value;
// use neopoligengine::helpers::get_include_str_file;
// use neopoligengine::section_v42::SectionV42;
// use neopoligengine::span_v42::named_span::named_span;
// use neopoligengine::span_v42::shorthand_code::code_shorthand_span;
// use neopoligengine::span_v42::shorthand_emphasis::emphasis_shorthand_span;
// use neopoligengine::span_v42::shorthand_footnote::footnote_shorthand_span;
// use neopoligengine::span_v42::shorthand_strong::strong_shorthand_span;
// use neopoligengine::{page_v42::PageV42b, site_config::SiteConfigV42};
// use pretty_assertions::assert_eq;
// use serde::Deserialize;
// use std::path::PathBuf;

const _TEST_FILES: Dir = include_dir!("tests/spec_tests/tests");

// #[derive(Clone, Debug)]
// pub struct TestItem {
//     description: String,
//     source: String,
//     result: Option<String>,
// }

// fn load_tests(file_name: &str) -> (Vec<TestItem>, Vec<TestItem>) {
//     let source = get_include_str_file(TEST_FILES, file_name);
//     let path = PathBuf::from("mock/path.neo");
//     let site_config = SiteConfigV42::config_for_spec_tests();
//     let page = PageV42b::new_from_str(&source, &path, &site_config).unwrap();
//     if let Some(data) = page
//         .raw_ast
//         .iter()
//         .filter_map(|section| match section {
//             SectionV42::Json5(sec) => Some(sec.clone()),
//             _ => None,
//         })
//         .find_map(|section| match section.kind.clone().as_str() {
//             "neopoligen-spec-test" => Some(section.data),
//             _ => None,
//         })
//     {
//         let v: serde_json::Value = serde_json::Value::deserialize(data).unwrap();
//         let valid_tests = &v.as_object().unwrap()["tests"]["valid"]
//             .as_array()
//             .unwrap()
//             .iter()
//             .map(|obj| TestItem {
//                 description: obj["description"].as_str().unwrap().to_string(),
//                 source: obj["source"].as_str().unwrap().to_string(),
//                 result: Some(obj["result"].as_str().unwrap().to_string()),
//             })
//             .collect::<Vec<TestItem>>();
//         let invalid_tests = &v.as_object().unwrap()["tests"]["invalid"]
//             .as_array()
//             .unwrap()
//             .iter()
//             .map(|obj| TestItem {
//                 description: obj["description"].as_str().unwrap().to_string(),
//                 source: obj["source"].as_str().unwrap().to_string(),
//                 result: None,
//             })
//             .collect::<Vec<TestItem>>();
//         (valid_tests.to_vec(), invalid_tests.to_vec())
//     } else {
//         (vec![], vec![])
//     }
// }

// #[test]
// #[ignore]
// fn test_named_spans() {
//     let (valid, invalid) = load_tests("named-spans.neo");
//     valid.iter().for_each(|ti| {
//         dbg!(&ti.description);
//         let parsed = named_span(&ti.source);
//         if parsed.is_ok() {
//             let expected =
//                 serde_json5::from_str::<Value>(ti.result.clone().unwrap().as_str()).unwrap();
//             let got = Value::from_serialize(&parsed.unwrap().1);
//             assert_eq!(expected, got);
//         } else {
//             dbg!(&ti.description);
//             assert!(false);
//         }
//     });
//     invalid.iter().for_each(|ti| {
//         let result = named_span(&ti.source);
//         if result.is_err() {
//             assert!(true)
//         } else {
//             dbg!(&ti.description);
//             assert!(false);
//         }
//     });
// }

// #[test]
// fn solo_test_code_shorthands() {
//     let (valid, invalid) = load_tests("code-shorthand.neo");
//     valid.iter().for_each(|ti| {
//         dbg!(&ti.description);
//         let parsed = code_shorthand_span(&ti.source);
//         if parsed.is_ok() {
//             let expected =
//                 serde_json5::from_str::<Value>(ti.result.clone().unwrap().as_str()).unwrap();
//             let got = Value::from_serialize(&parsed.unwrap().1);
//             assert_eq!(expected, got);
//         } else {
//             dbg!(&ti.description);
//             assert!(false);
//         }
//     });
//     invalid.iter().for_each(|ti| {
//         let result = code_shorthand_span(&ti.source);
//         if result.is_err() {
//             assert!(true)
//         } else {
//             dbg!(&ti.description);
//             assert!(false);
//         }
//     });
// }

// #[test]
// #[ignore]
// fn test_footnote_shorthands() {
//     let (valid, invalid) = load_tests("footnote-shorthand.neo");
//     valid.iter().for_each(|ti| {
//         let parsed = footnote_shorthand_span(&ti.source);
//         if parsed.is_ok() {
//             let expected =
//                 serde_json5::from_str::<Value>(ti.result.clone().unwrap().as_str()).unwrap();
//             let got = Value::from_serialize(&parsed.unwrap().1);
//             assert_eq!(expected, got);
//         } else {
//             dbg!(&ti.description);
//             assert!(false);
//         }
//     });
//     invalid.iter().for_each(|ti| {
//         let result = footnote_shorthand_span(&ti.source);
//         if result.is_err() {
//             assert!(true)
//         } else {
//             dbg!(&ti.description);
//             assert!(false);
//         }
//     });
// }

// #[test]
// #[ignore]
// fn test_strong_shorthands() {
//     let (valid, invalid) = load_tests("strong-shorthand.neo");
//     valid.iter().for_each(|ti| {
//         let parsed = strong_shorthand_span(&ti.source);
//         if parsed.is_ok() {
//             let expected =
//                 serde_json5::from_str::<Value>(ti.result.clone().unwrap().as_str()).unwrap();
//             let got = Value::from_serialize(&parsed.unwrap().1);
//             assert_eq!(expected, got, "{}", &ti.description);
//         } else {
//             dbg!(&ti.description);
//             assert!(false);
//         }
//     });
//     invalid.iter().for_each(|ti| {
//         let result = strong_shorthand_span(&ti.source);
//         if result.is_err() {
//             assert!(true, "{}", ti.description);
//         } else {
//             assert!(false, "{}", ti.description);
//         }
//     });
// }

// #[test]
// #[ignore]
// fn test_emphasis_shorthands() {
//     let (valid, invalid) = load_tests("emphasis-shorthand.neo");
//     valid.iter().for_each(|ti| {
//         let parsed = emphasis_shorthand_span(&ti.source);
//         if parsed.is_ok() {
//             let expected =
//                 serde_json5::from_str::<Value>(ti.result.clone().unwrap().as_str()).unwrap();
//             let got = Value::from_serialize(&parsed.unwrap().1);
//             assert_eq!(expected, got, "{}", &ti.description);
//         } else {
//             dbg!(&ti.description);
//             assert!(false);
//         }
//     });
//     invalid.iter().for_each(|ti| {
//         let result = emphasis_shorthand_span(&ti.source);
//         if result.is_err() {
//             assert!(true)
//         } else {
//             dbg!(&ti.description);
//             assert!(false);
//         }
//     });
// }

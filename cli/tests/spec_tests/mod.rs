use include_dir::{include_dir, Dir};
use neopoligengine::helpers::get_include_str_file;
use neopoligengine::section_v42::SectionV42;
use neopoligengine::{page_v42::PageV42, site_config::SiteConfigV42};
use serde::Deserialize;
use std::path::PathBuf;

const TEST_FILES: Dir = include_dir!("tests/spec_tests/tests");

#[derive(Debug)]
pub struct TestItem {}

fn load_tests(file_name: &str) -> (Vec<TestItem>, Vec<TestItem>) {
    let source = get_include_str_file(TEST_FILES, file_name);
    let path = PathBuf::from("mock/path.neo");
    let site_config = SiteConfigV42::config_for_spec_tests();
    let page = PageV42::new_from_str(&source, &path, &site_config).unwrap();
    dbg!(&page.raw_ast);
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
        dbg!(&data);

        let v: serde_json::Value = serde_json::Value::deserialize(data).unwrap();
        dbg!(&v.as_object().unwrap()["tests"]["valid"].as_array().unwrap());
        let valid_tests = &v.as_object().unwrap()["tests"]["valid"]
            .as_array()
            .unwrap()
            .iter()
            .map(|obj| TestItem {})
            .collect::<Vec<TestItem>>();
        dbg!(valid_tests);
        //let t = v[

        //let valid_items =
        //   &<serde_json::Value as TryInto<Vec<TestItem>>>::try_into(v["tests"]["valid"]);
        // .try_into()
        // .unwrap()
        // .map(|i| TestItem {})
        // .collect();
        //dbg!(valid_items);
        //iter().map(|i| TestItem {}).collect();
        //let data_iter = data.try_iter();
        //dbg!(data.get_item("tests"));

        // dbg!(data.try_iter()?.find_map(|k, v| {
        //     dbg!(k);
        //     true
        // }));
    }
    //.collect::<Vec<Value>>()
    //.iter()
    //.for_each(|data| {
    //  dbg!(data);
    //});

    //dbg!(valid_tests);

    (vec![], vec![])
}

#[test]
fn solo_test_named_spans() {
    let (valid, invalid) = load_tests("named-spans.neo");
    // let source_path = PathBuf::from(format!("mocks/{}", file));
    // let site_config = SiteConfigV42::mock1_basic();
    // match PageV42::new_from_str(&source, &source_path, &site_config) {
    //     Ok(page) => match page.error {
    //         Some(e) => {
    //             dbg!(e);
    //             assert!(false)
    //         }
    //         None => assert!(true),
    //     },
    //     Err(e) => {
    //         dbg!(e);
    //         assert!(false)
    //     }
    // }
}

// #[test]
// pub fn run_sample_13() {
//     let site_config = SiteConfigV42::mock_for_files_in_test_samples_dir();
//     let mut p = PageV42::mock_from_test_directory_sample(include_str!("samples/13.neo")).unwrap();
//     let parsed = p.generate_ast(&site_config);
//     if parsed.is_ok() {
//         assert!(true)
//     } else {
//         dbg!(&parsed);
//         assert!(false);
//     }
// }

// #[test]
// pub fn run_sample_12() {
//     let site_config = SiteConfigV42::mock_for_files_in_test_samples_dir();
//     let mut p = PageV42::mock_from_test_directory_sample(include_str!("samples/12.neo")).unwrap();
//     let parsed = p.generate_ast(&site_config);
//     if parsed.is_ok() {
//         assert!(true)
//     } else {
//         dbg!(&parsed);
//         assert!(false);
//     }
// }

// #[test]
// pub fn run_sample_11() {
//     let site_config = SiteConfigV42::mock_for_files_in_test_samples_dir();
//     let mut p = PageV42::mock_from_test_directory_sample(include_str!("samples/11.neo")).unwrap();
//     let parsed = p.generate_ast(&site_config);
//     if parsed.is_ok() {
//         assert!(true)
//     } else {
//         dbg!(&parsed);
//         assert!(false);
//     }
// }

// #[test]
// pub fn run_sample_10() {
//     let site_config = SiteConfigV42::mock_for_files_in_test_samples_dir();
//     let mut p = PageV42::mock_from_test_directory_sample(include_str!("samples/10.neo")).unwrap();
//     let parsed = p.generate_ast(&site_config);
//     if parsed.is_ok() {
//         assert!(true)
//     } else {
//         dbg!(&parsed);
//         assert!(false);
//     }
// }

// #[test]
// pub fn run_sample_9() {
//     let site_config = SiteConfigV42::mock_for_files_in_test_samples_dir();
//     let mut p = PageV42::mock_from_test_directory_sample(include_str!("samples/9.neo")).unwrap();
//     let parsed = p.generate_ast(&site_config);
//     if parsed.is_ok() {
//         assert!(true)
//     } else {
//         dbg!(&parsed);
//         assert!(false);
//     }
// }

// #[test]
// pub fn run_sample_8() {
//     let site_config = SiteConfigV42::mock_for_files_in_test_samples_dir();
//     let mut p = PageV42::mock_from_test_directory_sample(include_str!("samples/8.neo")).unwrap();
//     let parsed = p.generate_ast(&site_config);
//     if parsed.is_ok() {
//         assert!(true)
//     } else {
//         dbg!(&parsed);
//         assert!(false);
//     }
// }

// #[test]
// pub fn run_sample_7_and_earlier_to_do_split_out_at_some_point() {
//     let site_config = SiteConfigV42::mock_for_files_in_test_samples_dir();
//     let mut p = PageV42::mock_from_test_directory_sample(include_str!("samples/7.neo")).unwrap();
//     let parsed = p.generate_ast(&site_config);
//     if parsed.is_ok() {
//         assert!(true)
//     } else {
//         dbg!(&parsed);
//         assert!(false);
//     }
//     let mut p = PageV42::mock_from_test_directory_sample(include_str!("samples/6.neo")).unwrap();
//     let parsed = p.generate_ast(&site_config);
//     if parsed.is_ok() {
//         assert!(true)
//     } else {
//         dbg!(&parsed);
//         assert!(false);
//     }
//     let mut p = PageV42::mock_from_test_directory_sample(include_str!("samples/5.neo")).unwrap();
//     let parsed = p.generate_ast(&site_config);
//     if parsed.is_ok() {
//         assert!(true)
//     } else {
//         dbg!(&parsed);
//         assert!(false);
//     }
//     let mut p = PageV42::mock_from_test_directory_sample(include_str!("samples/4.neo")).unwrap();
//     let parsed = p.generate_ast(&site_config);
//     if parsed.is_ok() {
//         assert!(true)
//     } else {
//         dbg!(&parsed);
//         assert!(false);
//     }
//     let mut p = PageV42::mock_from_test_directory_sample(include_str!("samples/3.neo")).unwrap();
//     let parsed = p.generate_ast(&site_config);
//     if parsed.is_ok() {
//         assert!(true)
//     } else {
//         dbg!(&parsed);
//         assert!(false);
//     }
//     let mut p = PageV42::mock_from_test_directory_sample(include_str!("samples/2.neo")).unwrap();
//     let parsed = p.generate_ast(&site_config);
//     if parsed.is_ok() {
//         assert!(true)
//     } else {
//         dbg!(&parsed);
//         assert!(false);
//     }
//     let mut p = PageV42::mock_from_test_directory_sample(include_str!("samples/1.neo")).unwrap();
//     let parsed = p.generate_ast(&site_config);
//     if parsed.is_ok() {
//         assert!(true)
//     } else {
//         dbg!(&parsed);
//         assert!(false);
//     }
// }

// #[rstest]
// #[case(1, r#"``alfa bravo``"#, 0, 0)]
// #[case(2, r#"``alfa`bravo``"#, 0, 0)]
// #[case(3, r#"``alfa-bravo``"#, 0, 0)]
// #[case(4, r#"``alfa_bravo``"#, 0, 0)]
// #[case(5, r#"``alfa:bravo``"#, 0, 0)]
// #[case(6, r#"``alfa<bravo``"#, 0, 0)]
// #[case(7, r#"``alfa>bravo``"#, 0, 0)]
// #[case(8, r#"``alfa^bravo``"#, 0, 0)]
// #[case(9, r#"``alfa\|bravo``"#, 0, 0)]
// #[case(10, r#"``alfa\``bravo``"#, 0, 0)]
// #[case(11, r#"``alfa\-bravo``"#, 0, 0)]
// #[case(12, r#"``alfa\_bravo``"#, 0, 0)]
// #[case(13, r#"``alfa\--bravo``"#, 0, 0)]
// #[case(14, r#"``alfa\__bravo``"#, 0, 0)]
// #[case(15, r#"``alfa\:bravo``"#, 0, 0)]
// #[case(16, r#"``alfa\::bravo``"#, 0, 0)]
// #[case(17, r#"``alfa\\bravo``"#, 0, 0)]
// #[case(18, r#"``alfa bravo|charlie``"#, 1, 0)]
// #[case(19, r#"``alfa bravo|charlie|delta``"#, 2, 0)]
// #[case(20, r#"``alfa bravo|https://www.example.com/``"#, 1, 0)]
// #[case(21, r#"^^alfa bravo^^"#, 0, 0)]
// #[case(22, r#"^^alfa`bravo^^"#, 0, 0)]
// #[case(23, r#"^^alfa-bravo^^"#, 0, 0)]
// #[case(24, r#"^^alfa_bravo^^"#, 0, 0)]
// #[case(25, r#"^^alfa:bravo^^"#, 0, 0)]
// #[case(26, r#"^^alfa<bravo^^"#, 0, 0)]
// #[case(27, r#"^^alfa>bravo^^"#, 0, 0)]
// #[case(28, r#"^^alfa^bravo^^"#, 0, 0)]
// #[case(29, r#"^^alfa\|bravo^^"#, 0, 0)]
// #[case(30, r#"^^alfa\``bravo^^"#, 0, 0)]
// #[case(31, r#"^^alfa\-bravo^^"#, 0, 0)]
// #[case(32, r#"^^alfa\_bravo^^"#, 0, 0)]
// #[case(33, r#"^^alfa\--bravo^^"#, 0, 0)]
// #[case(34, r#"^^alfa\__bravo^^"#, 0, 0)]
// #[case(35, r#"^^alfa\:bravo^^"#, 0, 0)]
// #[case(36, r#"^^alfa\::bravo^^"#, 0, 0)]
// #[case(37, r#"^^alfa\\bravo^^"#, 0, 0)]
// #[case(38, r#"^^alfa bravo|charlie^^"#, 1, 0)]
// #[case(39, r#"^^alfa bravo|charlie|delta^^"#, 2, 0)]
// #[case(40, r#"^^alfa bravo|https://www.example.com/^^"#, 1, 0)]
// #[ignore]
// fn generated_shorthand_base_cases(
//     #[case] _number: usize,
//     #[case] source: &str,
//     #[case] flag_count: usize,
//     #[case] kv_count: usize,
// ) {
//     let span = shorthand(source).unwrap().1;
//     let kv = span
//         .attrs
//         .iter()
//         .filter_map(|attr| match &attr.kind {
//             SpanAttrKind::KeyValue { .. } => Some(()),
//             _ => None,
//         })
//         .collect::<Vec<()>>();
//     let flags = span
//         .attrs
//         .iter()
//         .filter_map(|attr| match &attr.kind {
//             SpanAttrKind::Flag { .. } => Some(()),
//             _ => None,
//         })
//         .collect::<Vec<()>>();
//     assert_eq!(kv.len(), kv_count);
//     assert_eq!(flags.len(), flag_count);
// }

use neopoligengine::{page_v42::PageV42, site_config::SiteConfigV42};
// use neopoligengine::site_config::SiteConfigV42;
// use neopoligengine::span::*;
// use neopoligengine::span_attr::*;
use neopoligengine::helpers::get_include_str_file;
use rstest::rstest;
use std::path::PathBuf;

use include_dir::{include_dir, Dir};

const VALID_SAMPLES: Dir = include_dir!("tests/parser_tests/full_page_tests");

#[rstest]
#[case("1.neo")]
#[case("2.neo")]
fn solo_integration_parser_test(#[case] file: &str) {
    let source = get_include_str_file(VALID_SAMPLES, file);
    let source_path = PathBuf::from(format!("mocks/{}", file));
    let site_config = SiteConfigV42::mock1_basic();
    match PageV42::new_from_str(&source, &source_path, &site_config) {
        Ok(page) => match page.error {
            Some(e) => {
                dbg!(e);
                assert!(false)
            }
            None => assert!(true),
        },

        Err(e) => {
            dbg!(e);
            assert!(false)
        }
    }
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

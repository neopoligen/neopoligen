#![allow(unused)]
use super::PageV42b;
use crate::helpers::get_include_str_file;
use crate::{ast_v42::parse_ast_v42, site_config::SiteConfigV42};
use include_dir::{include_dir, Dir};
use minijinja::Value;
use std::path::PathBuf;

const SAMPLES: Dir = include_dir!("src/page_v42/samples");

impl PageV42b {
    pub fn test_from_sample_file(file: &str) -> PageV42b {
        let source = get_include_str_file(SAMPLES, file);
        let site_config = SiteConfigV42::mock1_basic();
        let source_path =
            PathBuf::from(format!("{}/{}", &site_config.project_dir().display(), file));
        let mut page =
            PageV42b::new_from_str_v42(&source_path, &source, &site_config.project_dir());
        page.set_language_from_site_config(&site_config);
        page.generate_ast(&site_config);
        page.generate_id_from_ast();
        page.generate_fallback_id();
        page.generate_href_from_id();
        page.generate_href_from_ast();
        page.generate_fallback_href();
        page
    }

    pub fn new_mock_from_source(source: &str) -> PageV42b {
        let site_config = SiteConfigV42::mock_for_files_in_test_samples_dir();
        let source_path = PathBuf::from(format!(
            "{}/{}",
            &site_config.project_dir().display(),
            "mock-file.neo"
        ));
        let mut page =
            PageV42b::new_from_str_v42(&source_path, &source, &site_config.project_dir());
        page.set_language_from_site_config(&site_config);
        page.generate_ast(&site_config);
        page.generate_id_from_ast();
        page.generate_fallback_id();
        page.generate_href_from_id();
        page.generate_href_from_ast();
        page.generate_fallback_href();
        page
    }

    // // TODO: Load this thru the new_from_str method because
    // // that sets stuff up that will be needed for testing
    // pub fn valid_sample_mock(name: &str) -> PageV42 {
    //     let site_config = SiteConfigV42::mock1_basic();
    //     let source = get_include_str_file(SAMPLES, name);
    //     let raw_ast = parse_ast_v42(&source, &site_config, false).unwrap();
    //     PageV42 {
    //         id: Some("TODO_MAKE_MOCK_ID_HERE".to_string()),
    //         html_output_path: None,
    //         raw_ast,
    //         sections: Value::from_serialize(vec![""]),
    //         source,
    //         fs_path: PathBuf::from("mock/path/file.neo"),
    //         error: None,
    //     }
    // }

    //
}

// impl PageV42 {
//     pub fn test_from_sample_file(file: &str) -> PageV42 {
//         let source = get_include_str_file(SAMPLES, file);
//         let source_path = PathBuf::from(format!("mocks/{}", file));
//         let site_config = SiteConfigV42::mock1_basic();
//         PageV42::new_from_str(&source, &source_path, &site_config).unwrap()
//     }
//     // TODO: Load this thru the new_from_str method because
//     // that sets stuff up that will be needed for testing
//     pub fn valid_sample_mock(name: &str) -> PageV42 {
//         let site_config = SiteConfigV42::mock1_basic();
//         let source = get_include_str_file(SAMPLES, name);
//         let raw_ast = parse_ast_v42(&source, &site_config, false).unwrap();
//         PageV42 {
//             id: Some("TODO_MAKE_MOCK_ID_HERE".to_string()),
//             html_output_path: None,
//             raw_ast,
//             sections: Value::from_serialize(vec![""]),
//             source,
//             fs_path: PathBuf::from("mock/path/file.neo"),
//             error: None,
//         }
//     }
// }

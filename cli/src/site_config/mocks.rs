#![allow(unused_imports)]

use crate::{config_lists::ConfigList, site_config::*};
use std::collections::BTreeMap;

impl SiteConfigV42 {
    pub fn config_for_spec_tests() -> SiteConfigV42 {
        let mut section_types = BTreeMap::new();
        section_types.insert("checklist".to_string(), vec!["checklist".to_string()]);
        let config = SiteConfigV42 {
            debug_parser: false,
            language_default: "en".to_string(),
            base_url: "testsite.localhost:1999".to_string(),
            site_output_folder_name: "site".to_string(),
            project_root: Some(PathBuf::from("/site_config_mock/project_root")),
            section_types,
            attr_flag_lists: Some(ConfigLists {
                basic: BTreeMap::new(),
                block: BTreeMap::new(),
                checklist: BTreeMap::new(),
                checklist_item: BTreeMap::new(),
                json5: BTreeMap::new(),
                list: BTreeMap::new(),
                list_item: BTreeMap::new(),
                raw: BTreeMap::new(),
                unknow: BTreeMap::new(),
            }),
            theme: "test-theme".to_string(),
        };
        config
    }

    pub fn mock1_basic() -> SiteConfigV42 {
        let mut section_types = BTreeMap::new();
        let checklist_kinds = vec!["checklist".to_string(), "todos".to_string()];
        section_types.insert("checklist".to_string(), checklist_kinds);
        let config = SiteConfigV42 {
            debug_parser: false,
            language_default: "en".to_string(),
            base_url: "testsite.localhost:1999".to_string(),
            site_output_folder_name: "site".to_string(),
            project_root: Some(PathBuf::from("/test/mocks")),
            section_types,
            attr_flag_lists: Some(ConfigLists {
                basic: BTreeMap::new(),
                block: BTreeMap::new(),
                checklist: BTreeMap::new(),
                checklist_item: BTreeMap::new(),
                json5: BTreeMap::new(),
                list: BTreeMap::new(),
                list_item: BTreeMap::new(),
                raw: BTreeMap::new(),
                unknow: BTreeMap::new(),
            }),
            theme: "test-theme".to_string(),
        };
        config
    }

    pub fn mock_for_files_in_test_samples_folder() -> SiteConfigV42 {
        let mut section_types = BTreeMap::new();
        section_types.insert("checklist".to_string(), vec!["checklist".to_string()]);
        let config = SiteConfigV42 {
            debug_parser: false,
            language_default: "en".to_string(),
            base_url: "testsite.localhost:1999".to_string(),
            site_output_folder_name: "site".to_string(),
            project_root: Some(PathBuf::from("/test/mocks")),
            section_types,
            attr_flag_lists: Some(ConfigLists {
                basic: BTreeMap::new(),
                block: BTreeMap::new(),
                checklist: BTreeMap::new(),
                checklist_item: BTreeMap::new(),
                json5: BTreeMap::new(),
                list: BTreeMap::new(),
                list_item: BTreeMap::new(),
                raw: BTreeMap::new(),
                unknow: BTreeMap::new(),
            }),
            theme: "test-theme".to_string(),
        };
        config
    }

    //pub fn mock2_with_image_widths() -> SiteConfig {
    //    // let mut theme_options: BTreeMap<String, Vec<usize>> = BTreeMap::new();
    //    // theme_options.insert("image_widths".to_string(), vec![100, 400]);
    //    let sections = ConfigSections {
    //        basic: vec![
    //            "bookmark".to_string(),
    //            "div".to_string(),
    //            "image".to_string(),
    //            "p".to_string(),
    //            "preview".to_string(),
    //            "preview-thumbnail".to_string(),
    //            "previews".to_string(),
    //            "short".to_string(),
    //            "title".to_string(),
    //            "youtube".to_string(),
    //        ],
    //        block: vec![],
    //        checklist: vec!["checklist".to_string(), "todo".to_string()],
    //        checklist_item: vec!["checklist-item".to_string()],
    //        list: vec!["list".to_string(), "notes".to_string()],
    //        list_item: vec!["list-item".to_string()],
    //        json5: vec!["pagedata".to_string()],
    //        raw: vec![
    //            "code".to_string(),
    //            "css".to_string(),
    //            "html".to_string(),
    //            "javascript".to_string(),
    //            "pre".to_string(),
    //        ],
    //    };
    //    let config = SiteConfig {
    //        language_default: "en".to_string(),
    //        base_url_raw: "testsite.localhost:1999".to_string(),
    //        output_folder: "_site".to_string(),
    //        project_root: Some(PathBuf::from("/test/mocks")),
    //        section_attrs: vec![
    //            "alt".to_string(),
    //            "autofocus".to_string(),
    //            "hidden".to_string(),
    //            "rel".to_string(),
    //        ],
    //        sections,
    //        span_attrs: vec![
    //            "alt".to_string(),
    //            "autofocus".to_string(),
    //            "hidden".to_string(),
    //            "rel".to_string(),
    //        ],
    //        theme_name: "test-theme".to_string(),
    //        //theme_options: Some(serde_json::to_value(theme_options).unwrap()),
    //    };
    //    config
    //}

    // TODO Make mocks by calling SiteConfig::new____ soemthing

    // pub fn mock1() -> SiteConfig {
    //     // let sections: BTreeMap<String, Vec<String>> = BTreeMap::new();
    //     // let mut paths = BTreeMap::new();
    //     // paths.insert(
    //     //     "content_root".to_string(),
    //     //     PathBuf::from("/mock/root/content"),
    //     // );
    //     // paths.insert("output_root".to_string(), PathBuf::from("/mock/root/docs"));
    //     // paths.insert(
    //     //     "errors_root".to_string(),
    //     //     PathBuf::from("/mock/root/status/errors"),
    //     // );
    //     SiteConfig {
    //         base_url_raw: "https://www.example.com".to_string(),
    //         language_default: "en".to_string(),
    //         theme_name: "mock-theme-name".to_string(),
    //         theme_options: None,
    //         // options: serde_json::Value::from_str(r#"{}"#).unwrap(),
    //         project_root: Some(PathBuf::from("/mock/project/root")),
    //         sections: ConfigSections {
    //             basic: vec![
    //                 "bookmark".to_string(),
    //                 "div".to_string(),
    //                 "note".to_string(),
    //                 "p".to_string(),
    //                 "ref".to_string(),
    //                 "short".to_string(),
    //                 "start-theme-test".to_string(),
    //                 "title".to_string(),
    //                 "warning".to_string(),
    //             ],
    //             block: vec!["basic-block".to_string()],
    //             checklist: vec!["todo".to_string()],
    //             detail: vec!["detail".to_string()],
    //             json: vec!["json-example".to_string()],
    //             list: vec!["list".to_string()],
    //             raw: vec![
    //                 "code".to_string(),
    //                 "results".to_string(),
    //                 "expected-output".to_string(),
    //             ],
    //             table: vec!["table".to_string()],
    //             yaml: vec!["metadata".to_string()],
    //             // sections.insert("json".to_string(), vec!["metadata".to_string()]);
    //             // sections.insert("list".to_string(), vec!["list".to_string()]);
    //             // sections.insert(
    //             //     "raw".to_string(),
    //             //     vec![
    //             //         "code".to_string(),
    //             //         "css".to_string(),
    //             //         "html".to_string(),
    //             //         "javascript".to_string(),
    //             //         "pre".to_string(),
    //             //     ],
    //             // );
    //         },
    //         spans: vec![
    //             "em".to_string(),
    //             "link".to_string(),
    //             "span".to_string(),
    //             "strong".to_string(),
    //         ],
    //     }
    // }
}

// impl SiteConfig {

//pub fn mock1_basic() -> SiteConfig {
//    let sections = ConfigSections {
//        basic: vec![
//            "bookmark".to_string(),
//            "div".to_string(),
//            "image".to_string(),
//            "p".to_string(),
//            "preview".to_string(),
//            "preview-thumbnail".to_string(),
//            "previews".to_string(),
//            "short".to_string(),
//            "title".to_string(),
//            "youtube".to_string(),
//        ],
//        block: vec![],
//        checklist: vec!["checklist".to_string(), "todo".to_string()],
//        checklist_item: vec!["checklist-item".to_string()],
//        list: vec!["list".to_string(), "notes".to_string()],
//        list_item: vec!["list-item".to_string()],
//        json5: vec!["pagedata".to_string()],
//        raw: vec![
//            "code".to_string(),
//            "css".to_string(),
//            "html".to_string(),
//            "javascript".to_string(),
//            "pre".to_string(),
//        ],
//    };
//    let config = SiteConfig {
//        language_default: "en".to_string(),
//        base_url_raw: "testsite.localhost:1999".to_string(),
//        output_folder: "_site".to_string(),
//        project_root: Some(PathBuf::from("/test/mocks")),
//        section_attrs: vec![
//            "alt".to_string(),
//            "autofocus".to_string(),
//            "hidden".to_string(),
//            "rel".to_string(),
//        ],
//        sections,
//        span_attrs: vec![
//            "alt".to_string(),
//            "autofocus".to_string(),
//            "hidden".to_string(),
//            "rel".to_string(),
//        ],
//        theme_name: "test-theme".to_string(),
//        //theme_options: None,
//    };
//    config
//}

// pub fn mock2_with_image_widths() -> SiteConfig {
//     // let mut theme_options: BTreeMap<String, Vec<usize>> = BTreeMap::new();
//     // theme_options.insert("image_widths".to_string(), vec![100, 400]);
//     let sections = ConfigSections {
//         basic: vec![
//             "bookmark".to_string(),
//             "div".to_string(),
//             "image".to_string(),
//             "p".to_string(),
//             "preview".to_string(),
//             "preview-thumbnail".to_string(),
//             "previews".to_string(),
//             "short".to_string(),
//             "title".to_string(),
//             "youtube".to_string(),
//         ],
//         block: vec![],
//         checklist: vec!["checklist".to_string(), "todo".to_string()],
//         checklist_item: vec!["checklist-item".to_string()],
//         list: vec!["list".to_string(), "notes".to_string()],
//         list_item: vec!["list-item".to_string()],
//         json5: vec!["pagedata".to_string()],
//         raw: vec![
//             "code".to_string(),
//             "css".to_string(),
//             "html".to_string(),
//             "javascript".to_string(),
//             "pre".to_string(),
//         ],
//     };
//     let config = SiteConfig {
//         language_default: "en".to_string(),
//         base_url_raw: "testsite.localhost:1999".to_string(),
//         output_folder: "_site".to_string(),
//         project_root: Some(PathBuf::from("/test/mocks")),
//         section_attrs: vec![
//             "alt".to_string(),
//             "autofocus".to_string(),
//             "hidden".to_string(),
//             "rel".to_string(),
//         ],
//         sections,
//         span_attrs: vec![
//             "alt".to_string(),
//             "autofocus".to_string(),
//             "hidden".to_string(),
//             "rel".to_string(),
//         ],
//         theme_name: "test-theme".to_string(),
//         // theme_options: Some(serde_json::to_value(theme_options).unwrap()),
//     };
//     config
// }

// TODO Make mocks by calling SiteConfig::new____ soemthing

// pub fn mock1() -> SiteConfig {
//     // let sections: BTreeMap<String, Vec<String>> = BTreeMap::new();
//     // let mut paths = BTreeMap::new();
//     // paths.insert(
//     //     "content_root".to_string(),
//     //     PathBuf::from("/mock/root/content"),
//     // );
//     // paths.insert("output_root".to_string(), PathBuf::from("/mock/root/docs"));
//     // paths.insert(
//     //     "errors_root".to_string(),
//     //     PathBuf::from("/mock/root/status/errors"),
//     // );
//     SiteConfig {
//         base_url_raw: "https://www.example.com".to_string(),
//         language_default: "en".to_string(),
//         theme_name: "mock-theme-name".to_string(),
//         theme_options: None,
//         // options: serde_json::Value::from_str(r#"{}"#).unwrap(),
//         project_root: Some(PathBuf::from("/mock/project/root")),
//         sections: ConfigSections {
//             basic: vec![
//                 "bookmark".to_string(),
//                 "div".to_string(),
//                 "note".to_string(),
//                 "p".to_string(),
//                 "ref".to_string(),
//                 "short".to_string(),
//                 "start-theme-test".to_string(),
//                 "title".to_string(),
//                 "warning".to_string(),
//             ],
//             block: vec!["basic-block".to_string()],
//             checklist: vec!["todo".to_string()],
//             detail: vec!["detail".to_string()],
//             json: vec!["json-example".to_string()],
//             list: vec!["list".to_string()],
//             raw: vec![
//                 "code".to_string(),
//                 "results".to_string(),
//                 "expected-output".to_string(),
//             ],
//             table: vec!["table".to_string()],
//             yaml: vec!["metadata".to_string()],
//             // sections.insert("json".to_string(), vec!["metadata".to_string()]);
//             // sections.insert("list".to_string(), vec!["list".to_string()]);
//             // sections.insert(
//             //     "raw".to_string(),
//             //     vec![
//             //         "code".to_string(),
//             //         "css".to_string(),
//             //         "html".to_string(),
//             //         "javascript".to_string(),
//             //         "pre".to_string(),
//             //     ],
//             // );
//         },
//         spans: vec![
//             "em".to_string(),
//             "link".to_string(),
//             "span".to_string(),
//             "strong".to_string(),
//         ],
//     }
// }

//}

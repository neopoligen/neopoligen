// use std::collections::BTreeMap;
// use std::str::FromStr;

//use crate::sections::*;
use crate::site_config::*;
use std::collections::BTreeMap;
//use serde_json::Value;
// use std::path::PathBuf;

impl SiteConfig {
    pub fn mock1_basic() -> SiteConfig {
        let sections = ConfigSections {
            basic: vec![
                "bookmark".to_string(),
                "div".to_string(),
                "image".to_string(),
                "p".to_string(),
                "short".to_string(),
                "title".to_string(),
                "youtube".to_string(),
            ],
            block: vec![],
            checklist: vec!["checklist".to_string(), "todo".to_string()],
            checklist_item: vec!["checklist-item".to_string()],
            list: vec!["list".to_string(), "notes".to_string()],
            list_item: vec!["list-item".to_string()],
            json: vec![],
            raw: vec![
                "code".to_string(),
                "css".to_string(),
                "html".to_string(),
                "javascript".to_string(),
                "pre".to_string(),
            ],
            yaml: vec!["metadata".to_string()],
        };
        let config = SiteConfig {
            default_language: "en".to_string(),
            base_url_raw: "testsite.localhost:1999".to_string(),
            project_root: Some(PathBuf::from("/test/mocks")),
            section_attrs: vec![
                "alt".to_string(),
                "autofocus".to_string(),
                "hidden".to_string(),
                "rel".to_string(),
            ],
            sections,
            span_attrs: vec![
                "alt".to_string(),
                "autofocus".to_string(),
                "hidden".to_string(),
                "rel".to_string(),
            ],
            theme_name: "test-theme".to_string(),
            theme_options: None,
        };
        config
    }

    pub fn mock2_with_image_widths() -> SiteConfig {
        let mut theme_options: BTreeMap<String, Vec<usize>> = BTreeMap::new();
        theme_options.insert("image_widths".to_string(), vec![100, 400]);
        let sections = ConfigSections {
            basic: vec![
                "bookmark".to_string(),
                "div".to_string(),
                "image".to_string(),
                "p".to_string(),
                "short".to_string(),
                "title".to_string(),
                "youtube".to_string(),
            ],
            block: vec![],
            checklist: vec!["checklist".to_string(), "todo".to_string()],
            checklist_item: vec!["checklist-item".to_string()],
            list: vec!["list".to_string(), "notes".to_string()],
            list_item: vec!["list-item".to_string()],
            json: vec![],
            raw: vec![
                "code".to_string(),
                "css".to_string(),
                "html".to_string(),
                "javascript".to_string(),
                "pre".to_string(),
            ],
            yaml: vec!["metadata".to_string()],
        };
        let config = SiteConfig {
            default_language: "en".to_string(),
            base_url_raw: "testsite.localhost:1999".to_string(),
            project_root: Some(PathBuf::from("/test/mocks")),
            section_attrs: vec![
                "alt".to_string(),
                "autofocus".to_string(),
                "hidden".to_string(),
                "rel".to_string(),
            ],
            sections,
            span_attrs: vec![
                "alt".to_string(),
                "autofocus".to_string(),
                "hidden".to_string(),
                "rel".to_string(),
            ],
            theme_name: "test-theme".to_string(),
            theme_options: Some(serde_json::to_value(theme_options).unwrap()),
        };
        config
    }

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
    //         default_language: "en".to_string(),
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

use std::collections::BTreeMap;

use crate::sections::*;
use crate::site_config::SiteConfig;
//use serde_json::Value;
use std::path::PathBuf;

impl SiteConfig {
    pub fn mock1() -> SiteConfig {
        // let sections: BTreeMap<String, Vec<String>> = BTreeMap::new();
        let mut paths = BTreeMap::new();
        paths.insert(
            "content_root".to_string(),
            PathBuf::from("/mock/root/content"),
        );
        paths.insert("output_root".to_string(), PathBuf::from("/mock/root/docs"));
        paths.insert(
            "errors_root".to_string(),
            PathBuf::from("/mock/root/status/errors"),
        );

        SiteConfig {
            base_url_raw: Some("https://www.example.com".to_string()),
            default_language: Some("en".to_string()),
            max_image_width: Some(1800),
            theme: "mock-config-theme".to_string(),
            theme_options: None,
            sections: Sections {
                basic: vec![
                    "bookmark".to_string(),
                    "div".to_string(),
                    "p".to_string(),
                    "title".to_string(),
                    "note".to_string(),
                    "warning".to_string(),
                ],
                block: vec!["basic-block".to_string()],
                checklist: vec!["todo".to_string()],
                comment: vec!["comment".to_string()],
                detail: vec!["detail".to_string()],
                json: vec!["json-example".to_string()],
                list: vec!["list".to_string()],
                raw: vec!["code".to_string()],
                table: vec!["table".to_string()],
                yaml: vec!["metadata".to_string()],
                // sections.insert("json".to_string(), vec!["metadata".to_string()]);
                // sections.insert("list".to_string(), vec!["list".to_string()]);
                // sections.insert(
                //     "raw".to_string(),
                //     vec![
                //         "code".to_string(),
                //         "css".to_string(),
                //         "html".to_string(),
                //         "javascript".to_string(),
                //         "pre".to_string(),
                //     ],
                // );
            },
            spans: vec![
                "em".to_string(),
                "link".to_string(),
                "span".to_string(),
                "strong".to_string(),
            ],
            paths,
            project_root: Some(PathBuf::from("/mock/project/root")),
        }
    }
}

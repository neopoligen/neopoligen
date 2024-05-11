use std::collections::BTreeMap;

use crate::sections::*;
use crate::site_config::SiteConfig;
use crate::site_config::ThemeConfig;
use serde_json::Value;
use std::path::PathBuf;

impl SiteConfig {
    pub fn mock1() -> SiteConfig {
        let mut sections = BTreeMap::new();
        sections.insert(
            "basic".to_string(),
            vec![
                "div".to_string(),
                "note".to_string(),
                "p".to_string(),
                "title".to_string(),
            ],
        );
        sections.insert("json".to_string(), vec!["metadata".to_string()]);
        sections.insert("list".to_string(), vec!["list".to_string()]);
        sections.insert(
            "raw".to_string(),
            vec![
                "code".to_string(),
                "css".to_string(),
                "html".to_string(),
                "javascript".to_string(),
                "pre".to_string(),
            ],
        );
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
            default_language: "en".to_string(),
            theme: ThemeConfig {
                name: "mock-config-theme".to_string(),
                options: serde_json::from_str::<Value>("{}").unwrap(),
            },
            sections: Sections {
                basic: vec!["div".to_string()],
                checklist: vec!["div".to_string()],
                comment: vec!["div".to_string()],
                detail: vec!["div".to_string()],
                json: vec!["div".to_string()],
                list: vec!["div".to_string()],
                raw: vec!["div".to_string()],
                table: vec!["div".to_string()],
                yaml: vec!["div".to_string()],
            },
            spans: vec!["em".to_string(), "strong".to_string()],
            paths,
        }
    }
}

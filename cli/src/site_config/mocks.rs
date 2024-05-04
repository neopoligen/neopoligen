use std::collections::BTreeMap;

use crate::site_config::SiteConfigV2;
use crate::site_config::ThemeConfig;
use serde_json::Value;
use std::path::PathBuf;

impl SiteConfigV2 {
    pub fn mock1() -> SiteConfigV2 {
        let mut sections = BTreeMap::new();
        sections.insert(
            "basic".to_string(),
            vec!["title".to_string(), "p".to_string()],
        );
        sections.insert("json".to_string(), vec!["metadata".to_string()]);
        sections.insert("raw".to_string(), vec!["code".to_string()]);

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
        SiteConfigV2 {
            default_language: "en".to_string(),
            theme: ThemeConfig {
                name: "mock-config-theme".to_string(),
                options: serde_json::from_str::<Value>("{}").unwrap(),
            },
            sections,
            paths,
        }
    }
}

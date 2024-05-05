pub mod mocks;

use serde::Deserialize;
use serde::Serialize;
use serde_json;
use serde_json::Value;
use std::collections::BTreeMap;
use std::path::PathBuf;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub struct SiteConfig {
    #[serde(default = "default_language")]
    pub default_language: String,

    #[serde(default = "empty_paths")]
    pub paths: BTreeMap<String, PathBuf>,

    #[serde(default = "empty_sections")]
    pub sections: BTreeMap<String, Vec<String>>,

    pub theme: ThemeConfig,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub struct ThemeConfig {
    pub name: String,
    #[serde(default = "empty_options")]
    pub options: Value,
}

impl SiteConfig {
    pub fn load_sections(&mut self) {
        self.sections.insert(
            "basic".to_string(),
            vec!["title".to_string(), "div".to_string(), "p".to_string()],
        );
        self.sections
            .insert("json".to_string(), vec!["metadata".to_string()]);
        self.sections.insert(
            "raw".to_string(),
            vec![
                "code".to_string(),
                "html".to_string(),
                "javascript".to_string(),
                "css".to_string(),
            ],
        );
    }
}

fn default_language() -> String {
    "en".to_string()
}

fn empty_paths() -> BTreeMap<String, PathBuf> {
    BTreeMap::new()
}

fn empty_options() -> Value {
    serde_json::from_str::<Value>("{}").unwrap()
}

fn empty_sections() -> BTreeMap<String, Vec<String>> {
    BTreeMap::new()
}

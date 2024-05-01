pub mod mocks;
pub mod new;

use crate::config_folders::ConfigFolders;
use crate::config_section_categories::ConfigSectionCategories;
use serde::Deserialize;
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "lowercase", tag = "type")]
pub struct JsonConfig {
    pub default_language: String,
    pub domain: String,
    pub input_date_formats: Vec<String>,
    pub theme: String,
}

impl JsonConfig {
    pub fn stub1() -> JsonConfig {
        JsonConfig {
            default_language: "en".to_string(),
            domain: "localhost:1994".to_string(),
            input_date_formats: vec![],
            theme: "neopoligen-dev".to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Clone)]
#[serde(rename_all = "lowercase", tag = "type")]
pub struct Config {
    pub json_config: JsonConfig,
    pub folders: ConfigFolders,
    pub json_plugins: BTreeMap<String, String>,
    pub section_categories: ConfigSectionCategories,
    pub standard_spans: Vec<String>,
    pub text_plugins: BTreeMap<String, String>,
    ////////////////////////////
    // Deprecated: move to json.default_language
    // pub default_language: String,
    // Deprecated: move to json.domain
    pub domain: String,
    // Deprecated: move to json.input_date_formats
    pub input_date_formats: Vec<String>,
    // Deprecated: TODO - remove: key_value_spans
    pub key_value_spans: Vec<String>,
    // Deprecated: TODO - remove: main_body_section_exlucdes
    pub main_body_section_excludes: Vec<String>,
    // Deprecated: TODO - remove: section_attriubte_excludes
    pub section_attribute_excludes: Vec<String>,
    // Deprecated: move to json.theme_name
    pub theme_name: String,
    // Deprecated: move to json.time_zone_offset
    pub time_zone_offset: String,
}

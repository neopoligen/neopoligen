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
    pub input_date_formats: Vec<String>,
    pub theme: String,
}

impl JsonConfig {
    pub fn stub1() -> JsonConfig {
        JsonConfig {
            default_language: "en".to_string(),
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
    // Deprecated: TODO - remove: key_value_spans
    pub key_value_spans: Vec<String>,
    // Deprecated: move to json.time_zone_offset
    pub time_zone_offset: String,
}

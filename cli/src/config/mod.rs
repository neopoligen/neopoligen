pub mod builders;
pub mod new;

use crate::config_folders::ConfigFolders;
use crate::config_section_categories::ConfigSectionCategories;
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Serialize, Clone)]
#[serde(rename_all = "lowercase", tag = "type")]
pub struct Config {
    pub default_language: String,
    pub domain: String,
    pub folders: ConfigFolders,
    pub input_date_formats: Vec<String>,
    pub json_plugins: BTreeMap<String, String>,
    pub key_value_spans: Vec<String>,
    pub main_body_section_excludes: Vec<String>,
    pub section_attribute_excludes: Vec<String>,
    pub section_categories: ConfigSectionCategories,
    pub standard_spans: Vec<String>,
    pub text_plugins: BTreeMap<String, String>,
    pub theme_name: String,
    pub time_zone_offset: String,
}

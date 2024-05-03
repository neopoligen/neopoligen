use serde::Serialize;
use std::collections::BTreeSet;

#[derive(Debug, PartialEq, Serialize, Clone)]
#[serde(rename_all = "lowercase", tag = "type")]
pub struct ConfigSectionCategories {
    pub checklist: BTreeSet<String>,
    pub comment: BTreeSet<String>,
    pub detail: BTreeSet<String>,
    pub json: BTreeSet<String>,
    pub json_plugin: BTreeSet<String>,
    pub list: BTreeSet<String>,
    pub raw: BTreeSet<String>,
    pub standard: BTreeSet<String>,
    pub table: BTreeSet<String>,
    pub text_plugin: BTreeSet<String>,
    pub yaml: BTreeSet<String>,
}

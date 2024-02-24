use minijinja::Value;
use serde::Serialize;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct JsonPlugin {
    pub key_value_attributes: BTreeMap<String, String>,
    pub flag_attributes: BTreeSet<String>,
    pub bounds: String,
    pub template: String,
    pub object: Option<Value>,
    pub r#type: String,
}
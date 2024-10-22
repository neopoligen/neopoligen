use serde::Deserialize;
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ConfigLists {
    pub basic: BTreeMap<String, ConfigList>,
    pub block: BTreeMap<String, ConfigList>,
    pub checklist: BTreeMap<String, ConfigList>,
    pub checklist_item: BTreeMap<String, ConfigList>,
    pub json5: BTreeMap<String, ConfigList>,
    pub list: BTreeMap<String, ConfigList>,
    pub list_item: BTreeMap<String, ConfigList>,
    pub raw: BTreeMap<String, ConfigList>,
    pub unknow: BTreeMap<String, ConfigList>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ConfigList {
    pub attrs: Vec<String>,
    pub flags: Vec<String>,
}

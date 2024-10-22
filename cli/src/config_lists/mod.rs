use serde::Deserialize;
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ConfigLists {
    pub basic: BTreeMap<String, ConfigList>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ConfigList {
    pub attrs: Vec<String>,
    pub flags: Vec<String>,
}

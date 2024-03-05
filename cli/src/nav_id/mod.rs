use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub struct NavId {
    pub page_id: String,
    pub base_type: NavIdBaseType,
    pub children: Vec<NavId>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum NavIdBaseType {
    File,
    TitleFolder,
    IndexFolder,
}

use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub struct FolderMenuItem {
    pub page_id: String,
    pub is_current_link: bool,
    pub children: Vec<FolderMenuItem>,
    pub title: Option<String>,
    pub href: Option<String>,
}

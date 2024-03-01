use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub struct FolderMenuItem {
    pub children: Vec<FolderMenuItem>,
    pub href: Option<String>,
    pub is_current_link: bool,
    pub item_type: FolderMenuItemType,
    pub page_id: String,
    pub title: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
// #[serde(tag = "type", rename_all = "lowercase")]
pub enum FolderMenuItemType {
    Directory,
    File,
}

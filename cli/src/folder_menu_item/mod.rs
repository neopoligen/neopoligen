use serde::Deserialize;
use serde::Serialize;

// The first iteration used everything as a file link
// so things started as a struct. Directories
// are treated differently now so the move towards
// an enum has started. Refactoring to using enums
// entirely is on the nice-to-have list

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub struct FolderMenuItem {
    pub children: Vec<FolderMenuItem>,
    pub href: Option<String>,
    // pub is_current_link: bool,
    pub item_type: FolderMenuItemType,
    pub page_id: String,
    pub title: Option<String>,
    pub folder_path: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
// #[serde(tag = "type", rename_all = "lowercase")]
pub enum FolderMenuItemType {
    OpenDirectory,
    ClosedDirectory,
    File,
}

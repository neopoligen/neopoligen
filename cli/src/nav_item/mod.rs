use serde::Deserialize;
use serde::Serialize;

// The first iteration used everything as a file link
// so things started as a struct. Directories
// are treated differently now so the move towards
// an enum has started. Refactoring to using enums
// entirely is on the nice-to-have list

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub struct NavItem {
    pub children: Vec<NavItem>,
    pub folders: Vec<String>,
    pub href: Option<String>,
    pub is_current_page: bool,
    pub item_type: NavItemType,
    pub page_id: String,
    pub path_sort_string: String,
    pub title: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
// #[serde(tag = "type", rename_all = "lowercase")]
pub enum NavItemType {
    ClosedFolderIndex,
    ClosedFolderTitle,
    CurrentFile,
    NotCurrentFile,
}

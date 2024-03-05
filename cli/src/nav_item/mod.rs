use serde::Deserialize;
use serde::Serialize;
// use std::collections::BTreeMap;

// The first iteration used everything as a file link
// so things started as a struct. Directories
// are treated differently now so the move towards
// an enum has started. Refactoring to using enums
// entirely is on the nice-to-have list

// TODO: Add in the menu texts and prev_next stuff

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub struct NavItem {
    pub children: Vec<NavItem>,
    pub folders: Vec<String>,
    pub href: Option<String>,
    pub item_type: NavItemType,
    pub menu_title: Option<String>,
    pub menu_title_link_or_text: Option<String>,
    pub page_id: String,
    pub parent_ids: Vec<String>,
    pub path_sort_string: String,
    // pub prev_next_title: Option<String>,
    // pub prev_next_title_link_or_text: Option<String>,
    // pub short_title: Option<String>,
    // pub short_title_link_or_text: Option<String>,
    pub title: Option<String>,
    pub title_link_or_text: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum NavItemType {
    IndexFolderActive,
    IndexFolderClosed,
    TitleFolderClosed,
    FileCurrent,
    FileNotCurrent,
    IndexFolderOpened,
    TitleFolderOpened,
}

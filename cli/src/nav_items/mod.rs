pub mod new_from_files_and_folders;

use crate::nav_item::NavItem;
use crate::nav_prev_next_item::NavPrevNextItem;
use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub struct NavItems {
    pub tree: Vec<NavItem>,
    pub prev_next_order: Vec<NavPrevNextItem>,
}

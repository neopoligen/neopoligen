pub mod new;

use crate::nav_item::NavItem;
use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub struct NavTree {
    pub items: Vec<NavItem>,
}

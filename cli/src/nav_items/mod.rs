use crate::nav_item::NavItem;
use crate::nav_prev_next_item::NavPrevNextItem;
use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub struct NavTree {
    pub items: Vec<NavItem>,
    pub prev_item: Option<NavPrevNextItem>,
    pub next_item: Option<NavPrevNextItem>,
    pub prev_next_order: Vec<NavPrevNextItem>,
}

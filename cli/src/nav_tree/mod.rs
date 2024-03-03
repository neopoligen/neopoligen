pub mod new;

use crate::nav_item::NavItem;
use serde::Deserialize;
use serde::Serialize;

// TODO: Rename from NavTree to NavLinks

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub struct NavTree {
    pub items: Vec<NavItem>,
    pub prev_item: Option<NavPrevNextItem>,
    pub next_item: Option<NavPrevNextItem>,
    pub prev_next_order: Vec<NavPrevNextItem>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub struct NavPrevNextItem {
    /*
    pub href: Option<String>,
    pub menu_title: Option<String>,
    pub menu_title_link_or_text: Option<String>,
    pub page_id: String,
    pub prev_next_title: Option<String>,
    pub prev_next_title_link_or_text: Option<String>,
    pub title: Option<String>,
    */
    pub title_link_or_text: Option<String>,
}

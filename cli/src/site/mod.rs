pub mod all_site_links;
pub mod builder;
pub mod display;
pub mod filter_page_links_alpha;
pub mod full_title;
pub mod link_or_title;
pub mod link_or_title_filtered;
pub mod new;
pub mod object;
pub mod title_for_url;
pub mod tlink;
pub mod url_for_page;

use crate::page_data::PageData;
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "lowercase", tag = "type")]
pub struct Site {
    pub page_data: BTreeMap<String, PageData>,
}

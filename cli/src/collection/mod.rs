pub mod new_from_files_and_folders;

use crate::page::Page;
use minijinja::Value;
use serde::Deserialize;
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub struct Collection {
    pub items: Vec<CollectionItem>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub struct CollectionItem {
    pub page_id: String,
    pub base_type: CollectionItemType,
    pub children: Vec<CollectionItem>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum CollectionItemType {
    File,
    IndexFolder,
    TitleFolder,
}

pub fn get_nav_links_from_files_and_folders(
    pages: &BTreeMap<String, Page>,
    patterns: &[Value],
) -> Vec<CollectionItem> {
    vec![]
}

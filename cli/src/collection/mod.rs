pub mod new_from_files_and_folders;

use crate::page::Page;
use minijinja::Value;
use serde::Deserialize;
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub struct Collection {
    pub tree: Vec<CollectionItem>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub struct CollectionItem {
    pub active_type: CollectionActiveItemType,
    pub ancestors: Vec<String>,
    pub base_type: CollectionItemBaseType,
    pub children: Vec<CollectionItem>,
    pub page_id: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum CollectionItemBaseType {
    Page,
    IndexFolder,
    TitleFolder,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum CollectionActiveItemType {
    NotYetActivated,
    PageActive,
    PageInactive,
}

pub fn get_nav_links_from_files_and_folders(
    _pages: &BTreeMap<String, Page>,
    _patterns: &[Value],
) -> Vec<CollectionItem> {
    vec![]
}

impl Collection {
    pub fn set_active_item(&mut self, id: &String) {
        self.tree
            .iter_mut()
            .for_each(|item| mark_current_page(item, id));
        self.tree
            .iter_mut()
            .for_each(|item| mark_not_current_page(item, id));
    }
}

fn mark_current_page(item: &mut CollectionItem, id: &String) {
    if &item.page_id == id {
        item.active_type = CollectionActiveItemType::PageActive;
    } else {
        item.children
            .iter_mut()
            .for_each(|child| mark_current_page(child, id))
    }
}

fn mark_not_current_page(item: &mut CollectionItem, id: &String) {
    if item.base_type == CollectionItemBaseType::Page && &item.page_id != id {
        item.active_type = CollectionActiveItemType::PageInactive;
    }
    item.children
        .iter_mut()
        .for_each(|child| mark_not_current_page(child, id))
}

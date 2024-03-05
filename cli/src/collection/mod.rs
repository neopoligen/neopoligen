pub mod new_from_files_and_folders;

// use crate::page::Page;
// use minijinja::Value;
use serde::Deserialize;
use serde::Serialize;
// use std::collections::BTreeMap;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub struct Collection {
    pub tree: Vec<CollectionItem>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub struct CollectionItem {
    pub active_type: CollectionItemStatus,
    pub ancestors: Vec<String>,
    pub base_type: CollectionItemBaseType,
    pub children: Vec<CollectionItem>,
    pub id: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum CollectionItemBaseType {
    Page,
    IndexFolder,
    TitleFolder,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum CollectionItemStatus {
    NotYetActivated,
    PageActive,
    PageInactive,
}

// pub fn get_nav_links_from_files_and_folders(
//     _pages: &BTreeMap<String, Page>,
//     _patterns: &[Value],
// ) -> Vec<CollectionItem> {
//     vec![]
// }

impl Collection {
    pub fn set_active_item(&mut self, id: &String) {
        self.tree
            .iter_mut()
            .for_each(|item| mark_active_page(item, id));
        self.tree
            .iter_mut()
            .for_each(|item| mark_inactive_page(item, id));
    }
}

fn mark_active_page(item: &mut CollectionItem, id: &String) {
    if &item.id == id {
        item.active_type = CollectionItemStatus::PageActive;
    } else {
        item.children
            .iter_mut()
            .for_each(|child| mark_active_page(child, id))
    }
}

fn mark_inactive_page(item: &mut CollectionItem, id: &String) {
    if item.base_type == CollectionItemBaseType::Page && &item.id != id {
        item.active_type = CollectionItemStatus::PageInactive;
    }
    item.children
        .iter_mut()
        .for_each(|child| mark_inactive_page(child, id))
}

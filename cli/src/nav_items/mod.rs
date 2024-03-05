pub mod new_from_files_and_folders;

use crate::nav_id::NavId;
use crate::nav_id::NavIdBaseType;
use crate::nav_item::NavItem;
use crate::nav_item::NavItemType;
use minijinja::Value;
use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub struct NavItems {
    pub tree: Vec<NavItem>,
    pub prev_next_items: Vec<NavItem>,
    pub next_item: Option<NavItem>,
    pub prev_item: Option<NavItem>,
    pub open_folders: Vec<String>,
    pub current_item: Option<NavItem>,
}

impl NavItems {
    pub fn set_current_page(&mut self, page_id: &Value) {
        let page_id = page_id.to_string();
        if let Some(current_item) = self
            .tree
            .iter_mut()
            .find_map(|item| set_current_file(&page_id, item))
        {
            self.open_folders = current_item.folders.clone();
            self.current_item = Some(current_item);
        }
        self.tree
            .iter_mut()
            .for_each(|item| update_open_folders(item, &self.open_folders));
        self.next_item = get_next_item(&page_id, &self.prev_next_items);
        self.prev_item = get_prev_item(&page_id, &self.prev_next_items);
    }

    pub fn tree_items_from(&self, args: &[Value]) -> Vec<NavId> {
        let page_id = args[0].to_string();
        match self
            .tree
            .iter()
            .find_map(|item| is_tree_sub_root(item, &page_id))
        {
            Some(sub_root) => sub_root
                .children
                .iter()
                .map(|child| get_nav_id(child))
                .collect(),
            None => vec![],
        }
    }
}

fn get_nav_id(item: &NavItem) -> NavId {
    NavId {
        page_id: item.page_id.clone(),
        base_type: NavIdBaseType::File,
        children: item.children.iter().map(|c2| get_nav_id(c2)).collect(),
    }
}

fn is_tree_sub_root(item: &NavItem, page_id: &String) -> Option<NavItem> {
    if &item.page_id == page_id {
        Some(item.clone())
    } else {
        None
    }
}

fn update_open_folders(item: &mut NavItem, folders: &Vec<String>) {
    if item.item_type == NavItemType::TitleFolderClosed {
        let check_path: Vec<String> = folders
            .iter()
            .take(item.folders.len())
            .map(|f| f.to_string())
            .collect();
        if item.folders == check_path {
            item.item_type = NavItemType::TitleFolderOpened
        }
    }
    if item.item_type == NavItemType::IndexFolderClosed {
        let check_path: Vec<String> = folders
            .iter()
            .take(item.folders.len())
            .map(|f| f.to_string())
            .collect();
        if item.folders == check_path {
            item.item_type = NavItemType::IndexFolderOpened
        }
    }
    item.children
        .iter_mut()
        .for_each(|child| update_open_folders(child, folders));
}

fn get_prev_item(key: &String, items: &Vec<NavItem>) -> Option<NavItem> {
    match items.iter().position(|test_item| &test_item.page_id == key) {
        Some(index) => {
            if index > 0 {
                let prev_next_item = items.get(index - 1).unwrap().clone();
                Some(prev_next_item)
            } else {
                None
            }
        }
        None => None,
    }
}

fn get_next_item(key: &String, items: &Vec<NavItem>) -> Option<NavItem> {
    match items.iter().position(|test_item| &test_item.page_id == key) {
        Some(index) => match items.get(index + 1) {
            Some(item) => Some(item.clone()),
            None => None,
        },
        None => None,
    }
}

fn set_current_file(id: &String, item: &mut NavItem) -> Option<NavItem> {
    if item.page_id == id.to_string() {
        item.title_link_or_text = item.title.clone();
        item.menu_title_link_or_text = item.menu_title.clone();
        if matches!(item.item_type, NavItemType::IndexFolderClosed) {
            item.item_type = NavItemType::IndexFolderActive;
        } else if matches!(item.item_type, NavItemType::TitleFolderClosed) {
            item.item_type = NavItemType::TitleFolderActive;
        } else {
            item.item_type = NavItemType::FileCurrent;
        }
        Some(item.clone())
    } else {
        item.children
            .iter_mut()
            .find_map(|i| set_current_file(id, i))
    }
}

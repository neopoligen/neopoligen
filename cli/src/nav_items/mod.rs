pub mod new_from_files_and_folders;

use crate::nav_item::NavItem;
use crate::nav_item::NavItemType;
use crate::nav_prev_next_item::NavPrevNextItem;
use minijinja::Value;
use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub struct NavItems {
    pub tree: Vec<NavItem>,
    pub prev_next_items: Vec<NavPrevNextItem>,
    pub next_item: Option<NavPrevNextItem>,
    pub prev_item: Option<NavPrevNextItem>,
}

impl NavItems {
    pub fn set_current_page(&mut self, page_id: Value) {
        let page_id = page_id.to_string();
        self.tree
            .iter_mut()
            .for_each(|item| set_current_file(&page_id, item));
        self.next_item = get_next_item(&page_id, &self.prev_next_items);
        self.prev_item = get_prev_item(&page_id, &self.prev_next_items);
    }
}

fn get_prev_item(key: &String, items: &Vec<NavPrevNextItem>) -> Option<NavPrevNextItem> {
    match items.iter().position(|test_item| &test_item.page_id == key) {
        Some(index) => {
            if index > 0 {
                Some(items.get(index - 1).unwrap().clone())
            } else {
                None
            }
        }
        None => None,
    }
}

fn get_next_item(key: &String, items: &Vec<NavPrevNextItem>) -> Option<NavPrevNextItem> {
    match items.iter().position(|test_item| &test_item.page_id == key) {
        Some(index) => match items.get(index + 1) {
            Some(item) => Some(item.clone()),
            None => None,
        },
        None => None,
    }
}

fn set_current_file(id: &String, item: &mut NavItem) {
    if item.page_id == id.to_string() {
        item.is_current_page = true;
        item.title_link_or_text = item.title.clone();
        item.menu_title_link_or_text = item.menu_title.clone();
        if matches!(item.item_type, NavItemType::OpenedFolderIndex) {
            item.item_type = NavItemType::ActiveFolderIndex;
        } else {
            item.item_type = NavItemType::CurrentFile;
        }
    }
    item.children
        .iter_mut()
        .for_each(|i| set_current_file(id, i));
}

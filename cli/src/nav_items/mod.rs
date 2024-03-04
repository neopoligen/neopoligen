pub mod new_from_files_and_folders;

use crate::nav_item::NavItem;
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
        // self.next_item = Some(self.prev_next_items[2].clone());
        self.next_item = get_next_item(&page_id, &self.prev_next_items);
        self.prev_item = get_prev_item(&page_id, &self.prev_next_items);
    }
}

// fn set_prev_item<'a>(key: String, items: &'a Vec<String>) -> Option<&'a String> {
//     match items.iter().position(|item| item == &key) {
//         Some(index) => {
//             if index > 0 {
//                 items.get(index - 1)
//             } else {
//                 None
//             }
//         }
//         None => None,
//     }
// }

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

pub mod empty;
pub mod new_from_files_and_folders;
pub mod new_from_tags;

use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub struct Collection {
    pub tree: Vec<CollectionItem>,
    pub active_folders: Vec<String>,
    pub active_ancestors: Vec<String>,
    pub next_item: Option<CollectionItem>,
    pub prev_item: Option<CollectionItem>,
    pub prev_next_list: Vec<CollectionItem>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub struct CollectionItem {
    pub status: CollectionItemStatus,
    pub ancestors: Vec<String>,
    pub base_type: CollectionItemBaseType,
    pub children: Vec<CollectionItem>,
    pub folders: Vec<String>,
    pub id: String,
    pub sort_source_path: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum CollectionItemBaseType {
    Page,
    IndexFolder,
    TitleFolder,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum CollectionItemStatus {
    ToBeDetermined,
    PageActive,
    PageInactive,
    IndexFolderActive,
    IndexFolderClosed,
    IndexFolderOpened,
    TitleFolderActive,
    TitleFolderClosed,
    TitleFolderOpened,
}

impl Collection {
    pub fn get_subtree(&self, id: &String) -> Vec<CollectionItem> {
        match self.tree.iter().find_map(|item| find_subtree(item, id)) {
            Some(items) => items,
            None => vec![],
        }
    }

    pub fn set_active_ancestors(&mut self, id: &String) {
        if let Some(ancestors) = self
            .tree
            .iter()
            .find_map(|item| find_active_ancestors(item, id))
        {
            self.active_ancestors = ancestors
        }
    }

    pub fn set_active_folders(&mut self, id: &String) {
        if let Some(folders) = self
            .tree
            .iter()
            .find_map(|item| find_active_folders(item, id))
        {
            self.active_folders = folders
        }
    }

    pub fn set_active_item(&mut self, id: &String) {
        self.tree
            .iter_mut()
            .for_each(|item| mark_active_page(item, id));
        self.set_active_folders(id);
        self.set_active_ancestors(id);
        self.tree
            .iter_mut()
            .for_each(|item| mark_inactive_page(item, id));
        self.tree
            .iter_mut()
            .for_each(|item| mark_folder_opened_closed(item, id, &self.active_folders));
        self.prev_item = set_prev_item(&self.prev_next_list, &id);
        self.next_item = set_next_item(&self.prev_next_list, &id);
    }
}

fn find_active_ancestors(item: &CollectionItem, id: &String) -> Option<Vec<String>> {
    if &item.id == id {
        Some(item.ancestors.clone())
    } else {
        item.children
            .iter()
            .find_map(|child| find_active_ancestors(child, id))
    }
}

fn find_active_folders(item: &CollectionItem, id: &String) -> Option<Vec<String>> {
    if &item.id == id {
        Some(item.folders.clone())
    } else {
        item.children
            .iter()
            .find_map(|child| find_active_folders(child, id))
    }
}

fn find_subtree(item: &CollectionItem, id: &String) -> Option<Vec<CollectionItem>> {
    if &item.id == id {
        Some(item.children.clone())
    } else {
        item.children
            .iter()
            .find_map(|child| find_subtree(child, id))
    }
}

fn load_prev_next(items: &Vec<CollectionItem>, dest: &mut Vec<CollectionItem>) {
    items.iter().for_each(|item| {
        if !matches![item.base_type, CollectionItemBaseType::TitleFolder] {
            let prev_next_item = item.clone();
            dest.push(prev_next_item);
        }
        load_prev_next(&item.children, dest);
    });
}

fn mark_active_page(item: &mut CollectionItem, id: &String) {
    if &item.id == id {
        if item.base_type == CollectionItemBaseType::Page {
            item.status = CollectionItemStatus::PageActive;
        } else if item.base_type == CollectionItemBaseType::IndexFolder {
            item.status = CollectionItemStatus::IndexFolderActive;
        } else if item.base_type == CollectionItemBaseType::TitleFolder {
            item.status = CollectionItemStatus::TitleFolderActive;
        }
    } else {
        item.children
            .iter_mut()
            .for_each(|child| mark_active_page(child, id))
    }
}

fn mark_folder_opened_closed(item: &mut CollectionItem, id: &String, active_folders: &Vec<String>) {
    if item.status == CollectionItemStatus::ToBeDetermined {
        let folder_count = std::cmp::min(item.folders.len(), active_folders.len());
        if item.base_type == CollectionItemBaseType::TitleFolder {
            if active_folders.len() == 0 {
                item.status = CollectionItemStatus::TitleFolderClosed;
            } else if item.folders.len() > active_folders.len() {
                item.status = CollectionItemStatus::TitleFolderClosed;
            } else if &item
                .folders
                .iter()
                .take(folder_count)
                .collect::<Vec<&String>>()
                == &active_folders
                    .iter()
                    .take(folder_count)
                    .collect::<Vec<&String>>()
            {
                item.status = CollectionItemStatus::TitleFolderOpened;
            } else {
                item.status = CollectionItemStatus::TitleFolderClosed;
            }
        } else if item.base_type == CollectionItemBaseType::IndexFolder {
            if active_folders.len() == 0 {
                item.status = CollectionItemStatus::IndexFolderClosed;
            } else if item.folders.len() > active_folders.len() {
                item.status = CollectionItemStatus::IndexFolderClosed;
            } else if &item
                .folders
                .iter()
                .take(folder_count)
                .collect::<Vec<&String>>()
                == &active_folders
                    .iter()
                    .take(folder_count)
                    .collect::<Vec<&String>>()
            {
                item.status = CollectionItemStatus::IndexFolderOpened;
            } else {
                item.status = CollectionItemStatus::IndexFolderClosed;
            }
        }
    }
    item.children
        .iter_mut()
        .for_each(|child| mark_folder_opened_closed(child, id, active_folders))
}

fn mark_inactive_page(item: &mut CollectionItem, id: &String) {
    if item.base_type == CollectionItemBaseType::Page && &item.id != id {
        item.status = CollectionItemStatus::PageInactive;
    }
    item.children
        .iter_mut()
        .for_each(|child| mark_inactive_page(child, id))
}

fn set_prev_item(items: &Vec<CollectionItem>, id: &String) -> Option<CollectionItem> {
    match items.iter().position(|test_item| &test_item.id == id) {
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

fn set_next_item(items: &Vec<CollectionItem>, id: &String) -> Option<CollectionItem> {
    match items.iter().position(|test_item| &test_item.id == id) {
        Some(index) => match items.get(index + 1) {
            Some(item) => Some(item.clone()),
            None => None,
        },
        None => None,
    }
}

fn sort_by_source_path(items: &mut Vec<CollectionItem>) {
    items.sort_by_key(|k| k.sort_source_path.clone());
    items
        .iter_mut()
        .for_each(|item| sort_by_source_path(&mut item.children));
}

pub mod empty;
pub mod new_from_files_and_folders;

use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub struct Collection {
    pub tree: Vec<CollectionItem>,
    pub active_folders: Vec<String>,
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
    pub fn set_active_item(&mut self, id: &String) {
        self.tree
            .iter_mut()
            .for_each(|item| mark_active_page(item, id));
        self.set_active_folders(id);
        self.tree
            .iter_mut()
            .for_each(|item| mark_inactive_page(item, id));
        self.tree
            .iter_mut()
            .for_each(|item| mark_folder_opened_closed(item, id, &self.active_folders));
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

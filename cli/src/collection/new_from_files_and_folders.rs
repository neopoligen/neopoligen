use crate::collection::*;
use crate::page::Page;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

impl Collection {
    pub fn new_from_files_and_folders(
        pages: &BTreeMap<String, Page>,
        patterns: Vec<Vec<String>>,
    ) -> Collection {
        let mut tree: Vec<CollectionItem> = patterns
            .iter()
            .filter_map(|pattern| folder_menu_index_finder(pages, pattern.to_vec(), vec![]))
            .collect();
        tree.iter_mut()
            .for_each(|item| sort_by_source_path(&mut item.children));
        let mut prev_next_list = vec![];
        load_prev_next(&tree, &mut prev_next_list);
        let c = Collection {
            active_ancestors: vec![],
            active_folders: vec![],
            next_item: None,
            prev_item: None,
            prev_next_list,
            tree,
        };
        c
    }
}

fn sort_by_source_path(items: &mut Vec<CollectionItem>) {
    items.sort_by_key(|k| k.sort_source_path.clone());
    items
        .iter_mut()
        .for_each(|item| sort_by_source_path(&mut item.children));
}

fn folder_menu_index_finder(
    pages: &BTreeMap<String, Page>,
    pattern: Vec<String>,
    ancestors: Vec<String>,
) -> Option<CollectionItem> {
    let id = pattern[0].to_string();
    if let Some(page) = pages.get(&id) {
        Some(CollectionItem {
            ancestors: ancestors.clone(),
            base_type: CollectionItemBaseType::Page,
            children: vec![],
            folders: page.folders.clone(),
            id: id.clone(),
            sort_source_path: page.path_parts.join(""),
            status: CollectionItemStatus::ToBeDetermined,
        })
    } else {
        let mut full_pattern_with_title = pattern.clone();
        full_pattern_with_title.push("_title.neo".to_string());
        let mut full_pattern_with_index = pattern.clone();
        full_pattern_with_index.push("_index.neo".to_string());
        pages.iter().find_map(|page| {
            if full_pattern_with_title == page.1.path_parts {
                let mut updated_ancestors = ancestors.clone();
                updated_ancestors.push(page.0.clone());
                let mut children =
                    folder_menu_child_item_finder(pages, &pattern, updated_ancestors.clone());
                let mut next_folders: Vec<CollectionItem> =
                    folder_menu_subfolder_finder(pages, &pattern, updated_ancestors);
                children.append(&mut next_folders);
                Some(CollectionItem {
                    ancestors: ancestors.clone(),
                    base_type: CollectionItemBaseType::TitleFolder,
                    children,
                    folders: page.1.folders.clone(),
                    id: page.0.clone(),
                    sort_source_path: page.1.path_parts.join(""),
                    status: CollectionItemStatus::ToBeDetermined,
                })
            } else if full_pattern_with_index == page.1.path_parts {
                let mut updated_ancestors = ancestors.clone();
                updated_ancestors.push(page.0.clone());
                let mut children =
                    folder_menu_child_item_finder(pages, &pattern, updated_ancestors.clone());
                let mut next_folders: Vec<CollectionItem> =
                    folder_menu_subfolder_finder(pages, &pattern, updated_ancestors);
                children.append(&mut next_folders);
                Some(CollectionItem {
                    ancestors: ancestors.clone(),
                    base_type: CollectionItemBaseType::IndexFolder,
                    children,
                    folders: page.1.folders.clone(),
                    id: page.0.clone(),
                    sort_source_path: page.1.path_parts.join(""),
                    status: CollectionItemStatus::ToBeDetermined,
                })
            } else {
                None
            }
        })
    }
}

fn folder_menu_child_item_finder(
    pages: &BTreeMap<String, Page>,
    pattern: &Vec<String>,
    ancestors: Vec<String>,
) -> Vec<CollectionItem> {
    let mut full_pattern_with_title = pattern.clone();
    full_pattern_with_title.push("_title.neo".to_string());
    let mut full_pattern_with_index = pattern.clone();
    full_pattern_with_index.push("_index.neo".to_string());
    pages
        .iter()
        .filter_map(|page| {
            let page_folders = page.1.folders.clone();
            let path_parts = page.1.path_parts.clone();
            if &page_folders == pattern
                && path_parts != full_pattern_with_title
                && path_parts != full_pattern_with_index
            {
                Some(CollectionItem {
                    ancestors: ancestors.clone(),
                    base_type: CollectionItemBaseType::Page,
                    children: vec![],
                    folders: page.1.folders.clone(),
                    id: page.0.clone(),
                    sort_source_path: page.1.path_parts.join(""),
                    status: CollectionItemStatus::ToBeDetermined,
                })
            } else {
                None
            }
        })
        .collect()
}

fn folder_menu_subfolder_finder(
    pages: &BTreeMap<String, Page>,
    pattern: &Vec<String>,
    ancestors: Vec<String>,
) -> Vec<CollectionItem> {
    let mut next_level_folders: BTreeSet<Vec<String>> = BTreeSet::new();
    pages.iter().for_each(|page| {
        let page_folders = page.1.folders.clone();
        if page_folders
            .iter()
            .take(pattern.len())
            .eq(pattern.clone().iter())
        {
            if page_folders.len() == pattern.len() + 1 {
                next_level_folders.insert(page_folders);
            }
        }
    });
    next_level_folders
        .iter()
        .filter_map(|pat| folder_menu_index_finder(pages, pat.clone(), ancestors.clone()))
        .collect()
}

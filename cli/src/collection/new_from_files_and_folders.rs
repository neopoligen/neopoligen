use crate::collection::Collection;
use crate::collection::CollectionItem;
use crate::collection::CollectionItemBaseType;
use crate::collection::CollectionItemStatus;
use crate::page::Page;
use minijinja::Value;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

impl Collection {
    pub fn new_from_files_and_folders(
        pages: &BTreeMap<String, Page>,
        args: &[Value],
    ) -> Collection {
        let tree: Vec<CollectionItem> = args[0]
            .try_iter()
            .unwrap()
            .filter_map(|pattern_set| {
                let pattern: Vec<String> = pattern_set
                    .try_iter()
                    .unwrap()
                    .map(|pattern_part| pattern_part.to_string())
                    .collect();
                folder_menu_index_finder(pages, pattern)
            })
            .collect();
        let c = Collection { tree };
        c
    }
}

fn folder_menu_index_finder(
    pages: &BTreeMap<String, Page>,
    pattern: Vec<String>,
) -> Option<CollectionItem> {
    let id = pattern[0].to_string();
    if pages.contains_key(&id) {
        Some(CollectionItem {
            active_type: CollectionItemStatus::NotYetActivated,
            ancestors: vec![],
            base_type: CollectionItemBaseType::Page,
            children: vec![],
            id: id.clone(),
        })
    } else {
        let mut full_pattern_with_title = pattern.clone();
        full_pattern_with_title.push("_title.neo".to_string());
        let mut full_pattern_with_index = pattern.clone();
        full_pattern_with_index.push("_index.neo".to_string());
        pages.iter().find_map(|page| {
            if full_pattern_with_title == page.1.path_parts {
                let mut children = folder_menu_child_item_finder(pages, &pattern);
                let mut next_folders: Vec<CollectionItem> =
                    folder_menu_subfolder_finder(pages, &pattern);
                children.append(&mut next_folders);
                Some(CollectionItem {
                    active_type: CollectionItemStatus::NotYetActivated,
                    ancestors: vec![],
                    base_type: CollectionItemBaseType::TitleFolder,
                    children,
                    id: page.0.clone(),
                })
            } else if full_pattern_with_index == page.1.path_parts {
                let mut children = folder_menu_child_item_finder(pages, &pattern);
                let mut next_folders: Vec<CollectionItem> =
                    folder_menu_subfolder_finder(pages, &pattern);
                children.append(&mut next_folders);
                Some(CollectionItem {
                    active_type: CollectionItemStatus::NotYetActivated,
                    ancestors: vec![],
                    base_type: CollectionItemBaseType::IndexFolder,
                    children,
                    id: page.0.clone(),
                })

            //         let mut fmi = NavItem {
            //             children: folder_menu_child_item_finder(
            //                 site,
            //                 &pattern,
            //                 next_parent_ids.clone(),
            //             ),
            //             folders: site.page_folders(&page_args),
            //             href: site.page_href(&[Value::from(page.1.id.clone())]),
            //             item_type: NavItemBaseType::TitleFolderClosed,
            //             menu_title: site.page_menu_title(&[Value::from(page.1.id.clone())]),
            //             menu_title_link_or_text: site
            //                 .nav_link_title_link(&[Value::from(page.1.id.clone())]),
            //             page_id: page.1.id.clone(),
            //             path_sort_string: site.page_path_parts(&page_args).join(""),
            //             parent_ids: parent_ids.clone(),
            //             title: site.page_title(&[Value::from(page.1.id.clone())]),
            //             title_link_or_text: site.nav_link_title_link(&[Value::from(page.1.id.clone())]),
            //         };
            //         // TODO: Get sub folders here
            //         let mut next_folders: Vec<NavItem> =
            //             folder_menu_subfolder_finder(site, &pattern, next_parent_ids.clone());
            //         fmi.children.append(&mut next_folders);
            //         Some(fmi)
            //     } else if full_pattern_with_index
            //         == site.page_path_parts(&[Value::from(page.1.id.clone())])
            //     {
            //         let mut fmi = NavItem {
            //             children: folder_menu_child_item_finder(
            //                 site,
            //                 &pattern,
            //                 next_parent_ids.clone(),
            //             ),
            //             folders: site.page_folders(&page_args),
            //             href: site.page_href(&[Value::from(page.1.id.clone())]),
            //             item_type: NavItemBaseType::IndexFolderClosed,
            //             menu_title: site.page_menu_title(&[Value::from(page.1.id.clone())]),
            //             menu_title_link_or_text: site
            //                 .nav_link_title_link(&[Value::from(page.1.id.clone())]),
            //             page_id: page.1.id.clone(),
            //             path_sort_string: site.page_path_parts(&page_args).join(""),
            //             parent_ids: vec![],
            //             title: site.page_title(&[Value::from(page.1.id.clone())]),
            //             title_link_or_text: site.nav_link_title_link(&[Value::from(page.1.id.clone())]),
            //         };
            //         let mut next_folders: Vec<NavItem> =
            //             folder_menu_subfolder_finder(site, &pattern, next_parent_ids.clone());
            //         fmi.children.append(&mut next_folders);
            //         Some(fmi)
            } else {
                None
            }
        })
    }
}

fn folder_menu_child_item_finder(
    pages: &BTreeMap<String, Page>,
    pattern: &Vec<String>,
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
                    active_type: CollectionItemStatus::NotYetActivated,
                    ancestors: vec![],
                    base_type: CollectionItemBaseType::Page,
                    children: vec![],
                    id: page.0.clone(),
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
        .filter_map(|pat| folder_menu_index_finder(pages, pat.clone()))
        .collect()
}

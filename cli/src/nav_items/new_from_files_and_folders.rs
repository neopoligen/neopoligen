use crate::nav_item::NavItem;
use crate::nav_items::NavItems;
// use crate::page::Page;
use crate::nav_item::NavItemType;
use crate::site::Site;
use minijinja::Value;
use std::collections::BTreeSet;
use tracing::{event, instrument, Level};

impl NavItems {
    pub fn new_from_files_and_folders(site: &Site, pattern_sets: &Value) -> NavItems {
        let tree: Vec<NavItem> = pattern_sets
            .try_iter()
            .unwrap()
            .filter_map(|pattern_set| {
                let pattern: Vec<String> = pattern_set
                    .try_iter()
                    .unwrap()
                    .map(|pattern_part| pattern_part.to_string())
                    .collect();
                NavItems::folder_menu_index_finder(site, pattern)
            })
            .collect();
        NavItems {
            tree,
            prev_next_order: vec![],
        }
    }

    #[instrument]
    pub fn folder_menu_index_finder(site: &Site, pattern: Vec<String>) -> Option<NavItem> {
        event!(Level::INFO, "fn folder_menu_index_finder");
        // Get a page if the ID matches, otherwise process
        // as a folder
        let id = pattern[0].to_string();
        if site.pages.contains_key(&id) {
            let page_args = [Value::from(id.clone())];
            Some(NavItem {
                children: NavItems::folder_menu_child_item_finder(site, &pattern),
                folders: site.page_folders(&page_args),
                href: site.page_href(&page_args),
                item_type: NavItemType::NotCurrentFile,
                is_current_page: false,
                menu_title: site.page_menu_title(&page_args),
                menu_title_link_or_text: site.nav_link_title_link(&page_args),
                page_id: id.clone(),
                path_sort_string: site.page_path_parts(&page_args).join(""),
                title: site.page_title(&page_args),
                title_link_or_text: site.nav_link_title_link(&page_args),
            })
        } else {
            let mut full_pattern_with_title = pattern.clone();
            full_pattern_with_title.push("_title.neo".to_string());
            let mut full_pattern_with_index = pattern.clone();
            full_pattern_with_index.push("_index.neo".to_string());
            site.pages.iter().find_map(|page| {
                event!(Level::DEBUG, "{}", page.0);
                let page_args = [Value::from(page.1.id.clone())];
                if full_pattern_with_title
                    == site.page_path_parts(&[Value::from(page.1.id.clone())])
                {
                    let mut fmi = NavItem {
                        children: NavItems::folder_menu_child_item_finder(site, &pattern),
                        folders: site.page_folders(&page_args),
                        href: site.page_href(&[Value::from(page.1.id.clone())]),
                        is_current_page: false,
                        item_type: NavItemType::ClosedFolderTitle,
                        menu_title: site.page_menu_title(&[Value::from(page.1.id.clone())]),
                        menu_title_link_or_text: site
                            .nav_link_title_link(&[Value::from(page.1.id.clone())]),
                        page_id: page.1.id.clone(),
                        path_sort_string: site.page_path_parts(&page_args).join(""),
                        title: site.page_title(&[Value::from(page.1.id.clone())]),
                        title_link_or_text: site
                            .nav_link_title_link(&[Value::from(page.1.id.clone())]),
                    };
                    // TODO: Get sub folders here
                    let mut next_folders: Vec<NavItem> =
                        NavItems::folder_menu_subfolder_finder(site, &pattern);
                    fmi.children.append(&mut next_folders);
                    Some(fmi)
                } else if full_pattern_with_index
                    == site.page_path_parts(&[Value::from(page.1.id.clone())])
                {
                    let mut fmi = NavItem {
                        children: NavItems::folder_menu_child_item_finder(site, &pattern),
                        folders: site.page_folders(&page_args),
                        href: site.page_href(&[Value::from(page.1.id.clone())]),
                        is_current_page: false,
                        item_type: NavItemType::ClosedFolderIndex,
                        menu_title: site.page_menu_title(&[Value::from(page.1.id.clone())]),
                        menu_title_link_or_text: site
                            .nav_link_title_link(&[Value::from(page.1.id.clone())]),
                        page_id: page.1.id.clone(),
                        path_sort_string: site.page_path_parts(&page_args).join(""),
                        title: site.page_title(&[Value::from(page.1.id.clone())]),
                        title_link_or_text: site
                            .nav_link_title_link(&[Value::from(page.1.id.clone())]),
                    };
                    let mut next_folders: Vec<NavItem> =
                        NavItems::folder_menu_subfolder_finder(site, &pattern);
                    fmi.children.append(&mut next_folders);
                    Some(fmi)
                } else {
                    None
                }
            })
        }
    }

    #[instrument]
    pub fn folder_menu_child_item_finder(site: &Site, pattern: &Vec<String>) -> Vec<NavItem> {
        event!(Level::INFO, "fn folder_menu_child_item_finder");
        let mut full_pattern_with_title = pattern.clone();
        full_pattern_with_title.push("_title.neo".to_string());
        let mut full_pattern_with_index = pattern.clone();
        full_pattern_with_index.push("_index.neo".to_string());
        site.pages
            .iter()
            .filter_map(|page| {
                let page_args = [Value::from(page.1.id.clone())];
                let page_folders = site.page_folders(&[Value::from(page.1.id.clone())]);
                let path_parts = site.page_path_parts(&[Value::from(page.1.id.clone())]);
                if &page_folders == pattern
                    && path_parts != full_pattern_with_title
                    && path_parts != full_pattern_with_index
                {
                    let fmi = NavItem {
                        children: vec![],
                        folders: site.page_folders(&page_args),
                        href: site.page_href(&[Value::from(page.1.id.clone())]),
                        item_type: NavItemType::NotCurrentFile,
                        page_id: page.1.id.clone(),
                        menu_title: site.page_menu_title(&[Value::from(page.1.id.clone())]),
                        menu_title_link_or_text: site
                            .nav_link_title_link(&[Value::from(page.1.id.clone())]),
                        path_sort_string: site.page_path_parts(&page_args).join(""),
                        is_current_page: false,
                        title: site.page_title(&[Value::from(page.1.id.clone())]),
                        title_link_or_text: site
                            .nav_link_title_link(&[Value::from(page.1.id.clone())]),
                    };
                    Some(fmi)
                } else {
                    None
                }
            })
            .collect()
    }

    #[instrument]
    pub fn folder_menu_subfolder_finder(site: &Site, pattern: &Vec<String>) -> Vec<NavItem> {
        event!(Level::INFO, "fn folder_menu_subfolder_finder");
        let mut next_level_folders: BTreeSet<Vec<String>> = BTreeSet::new();
        site.pages.iter().for_each(|page| {
            let page_folders = site.page_folders(&[Value::from(page.1.id.clone())]);
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
            .filter_map(|pat| NavItems::folder_menu_index_finder(site, pat.clone()))
            .collect()
    }
}

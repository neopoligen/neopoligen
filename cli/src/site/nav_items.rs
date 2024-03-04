use crate::nav_item::NavItem;
use crate::nav_item::NavItemType;
use crate::nav_prev_next_item::NavPrevNextItem;
use crate::nav_tree::NavTree;
// use crate::site::CacheObject;
use crate::site::Site;
// use itertools::Itertools;
use minijinja::Value;
use std::collections::BTreeSet;
use tracing::{event, instrument, Level};

impl Site {
    ///////////////////////////////////////////////////////////////
    // SKIP
    pub fn find_prev_next_nav_links(&self, _id: &String, links: &mut NavTree) {
        links.next_item = Some(NavPrevNextItem {
            page_id: "content-bravo".to_string(),
            title_link_or_text: None,
        });
        // dbg!("here");
    }

    ///////////////////////////////////////////////////////////////
    // SKIP
    pub fn folder_menu(&self, args: &[Value]) -> Vec<NavItem> {
        let mut items = self.folder_menu_builder(args);
        items
            .iter_mut()
            .for_each(|item| self.folder_menu_set_open_closed_folders(args, item));
        items
    }

    ///////////////////////////////////////////////////////////////
    // MOVED
    pub fn folder_menu_builder(&self, _args: &[Value]) -> Vec<NavItem> {
        vec![]

        // let menu_key = args[1]
        //     .try_iter()
        //     .unwrap()
        //     .into_iter()
        //     .map(|f| f.try_iter().unwrap().into_iter().join("-"))
        //     .join("-");
        // match self.get_cache("menus", &menu_key) {
        //     Some(menu_cache) => {
        //         if let CacheObject::Menu(menu) = menu_cache {
        //             menu
        //         } else {
        //             vec![]
        //         }
        //     }
        //     None => {
        //         let mut r: Vec<NavItem> = args[1]
        //             .try_iter()
        //             .unwrap()
        //             .filter_map(|folder_vecs| {
        //                 let folder_pattern: Vec<String> = folder_vecs
        //                     .try_iter()
        //                     .unwrap()
        //                     .map(|f| f.to_string())
        //                     .collect();
        //                 self.folder_menu_index_finder(folder_pattern)
        //             })
        //             .collect();
        //         r.iter_mut()
        //             .for_each(|item| self.folder_menu_sort_by_path(&mut item.children));
        //         self.set_cache("menus", menu_key, CacheObject::Menu(r.clone()));
        //         r
        //     }
        // }

        // event!(Level::INFO, "fn folder_menu_builder");
        // let mut binding = self.cache.lock().unwrap();
        // let menus = binding.get("menus").unwrap();
        // match menus.get(&menu_key) {
        //     Some(menu_cache) => {
        //         if let CacheObject::Menu(menu) = menu_cache {
        //             menu.to_vec()
        //         } else {
        //             vec![]
        //         }
        //     }
        //     None => {
        //         let r = args[1]
        //             .try_iter()
        //             .unwrap()
        //             .filter_map(|folder_vecs| {
        //                 let folder_pattern: Vec<String> = folder_vecs
        //                     .try_iter()
        //                     .unwrap()
        //                     .map(|f| f.to_string())
        //                     .collect();
        //                 self.folder_menu_index_finder(folder_pattern)
        //             })
        //             .collect();
        //         r
        //     }
        // }
        // let r = args[1]
        //     .try_iter()
        //     .unwrap()
        //     .filter_map(|folder_vecs| {
        //         let folder_pattern: Vec<String> = folder_vecs
        //             .try_iter()
        //             .unwrap()
        //             .map(|f| f.to_string())
        //             .collect();
        //         self.folder_menu_index_finder(folder_pattern)
        //     })
        //     .collect();
        // r
        // }
    }

    ///////////////////////////////////////////////////////////////
    // MOVED
    #[instrument(skip(self))]
    pub fn folder_menu_child_item_finder(&self, pattern: &Vec<String>) -> Vec<NavItem> {
        event!(Level::INFO, "fn folder_menu_child_item_finder");
        let mut full_pattern_with_title = pattern.clone();
        full_pattern_with_title.push("_title.neo".to_string());
        let mut full_pattern_with_index = pattern.clone();
        full_pattern_with_index.push("_index.neo".to_string());
        self.pages
            .iter()
            .filter_map(|page| {
                let page_args = [Value::from(page.1.id.clone())];
                let page_folders = self.page_folders(&[Value::from(page.1.id.clone())]);
                let path_parts = self.page_path_parts(&[Value::from(page.1.id.clone())]);
                if &page_folders == pattern
                    && path_parts != full_pattern_with_title
                    && path_parts != full_pattern_with_index
                {
                    let fmi = NavItem {
                        children: vec![],
                        folders: self.page_folders(&page_args),
                        href: self.page_href(&[Value::from(page.1.id.clone())]),
                        item_type: NavItemType::NotCurrentFile,
                        page_id: page.1.id.clone(),
                        menu_title: self.page_menu_title(&[Value::from(page.1.id.clone())]),
                        menu_title_link_or_text: self
                            .nav_link_title_link(&[Value::from(page.1.id.clone())]),
                        path_sort_string: self.page_path_parts(&page_args).join(""),
                        is_current_page: false,
                        title: self.page_title(&[Value::from(page.1.id.clone())]),
                        title_link_or_text: self
                            .nav_link_title_link(&[Value::from(page.1.id.clone())]),
                    };
                    Some(fmi)
                } else {
                    None
                }
            })
            .collect()
    }

    ////////////////////////////////////////////////////////////////////////////
    // MOVED
    #[instrument(skip(self))]
    pub fn folder_menu_index_finder(&self, pattern: Vec<String>) -> Option<NavItem> {
        event!(Level::INFO, "fn folder_menu_index_finder");
        // Get a page if the ID matches
        let id = pattern[0].to_string();
        if self.pages.contains_key(&id) {
            let page_args = [Value::from(id.clone())];
            Some(NavItem {
                children: self.folder_menu_child_item_finder(&pattern),
                folders: self.page_folders(&page_args),
                href: self.page_href(&page_args),
                item_type: NavItemType::NotCurrentFile,
                is_current_page: false,
                menu_title: self.page_menu_title(&page_args),
                menu_title_link_or_text: self.nav_link_title_link(&page_args),
                page_id: id.clone(),
                path_sort_string: self.page_path_parts(&page_args).join(""),
                title: self.page_title(&page_args),
                title_link_or_text: self.nav_link_title_link(&page_args),
            })
        } else {
            let mut full_pattern_with_title = pattern.clone();
            full_pattern_with_title.push("_title.neo".to_string());
            let mut full_pattern_with_index = pattern.clone();
            full_pattern_with_index.push("_index.neo".to_string());
            self.pages.iter().find_map(|page| {
                event!(Level::DEBUG, "{}", page.0);
                let page_args = [Value::from(page.1.id.clone())];
                if full_pattern_with_title
                    == self.page_path_parts(&[Value::from(page.1.id.clone())])
                {
                    let mut fmi = NavItem {
                        children: self.folder_menu_child_item_finder(&pattern),
                        folders: self.page_folders(&page_args),
                        href: self.page_href(&[Value::from(page.1.id.clone())]),
                        is_current_page: false,
                        item_type: NavItemType::ClosedFolderTitle,
                        menu_title: self.page_menu_title(&[Value::from(page.1.id.clone())]),
                        menu_title_link_or_text: self
                            .nav_link_title_link(&[Value::from(page.1.id.clone())]),
                        page_id: page.1.id.clone(),
                        path_sort_string: self.page_path_parts(&page_args).join(""),
                        title: self.page_title(&[Value::from(page.1.id.clone())]),
                        title_link_or_text: self
                            .nav_link_title_link(&[Value::from(page.1.id.clone())]),
                    };
                    // TODO: Get sub folders here
                    let mut next_folders: Vec<NavItem> =
                        self.folder_menu_subfolder_finder(&pattern);
                    fmi.children.append(&mut next_folders);
                    Some(fmi)
                } else if full_pattern_with_index
                    == self.page_path_parts(&[Value::from(page.1.id.clone())])
                {
                    let mut fmi = NavItem {
                        children: self.folder_menu_child_item_finder(&pattern),
                        folders: self.page_folders(&page_args),
                        href: self.page_href(&[Value::from(page.1.id.clone())]),
                        is_current_page: false,
                        item_type: NavItemType::ClosedFolderIndex,
                        menu_title: self.page_menu_title(&[Value::from(page.1.id.clone())]),
                        menu_title_link_or_text: self
                            .nav_link_title_link(&[Value::from(page.1.id.clone())]),
                        page_id: page.1.id.clone(),
                        path_sort_string: self.page_path_parts(&page_args).join(""),
                        title: self.page_title(&[Value::from(page.1.id.clone())]),
                        title_link_or_text: self
                            .nav_link_title_link(&[Value::from(page.1.id.clone())]),
                    };
                    // TODO: Get sub folders here
                    let mut next_folders: Vec<NavItem> =
                        self.folder_menu_subfolder_finder(&pattern);
                    fmi.children.append(&mut next_folders);
                    Some(fmi)
                } else {
                    None
                }
            })
        }
    }

    //////////////////////////////////////////////////////////////////
    // MOVING
    pub fn folder_menu_set_open_closed_folders(&self, args: &[Value], item: &mut NavItem) {
        if matches!(item.item_type, NavItemType::ClosedFolderTitle) {
            let page_folders = self.page_folders(args);
            if page_folders
                .into_iter()
                .take(item.folders.len())
                .collect::<Vec<String>>()
                == item.folders
            {
                item.item_type = NavItemType::OpenedFolderTitle;
            } else {
                item.item_type = NavItemType::ClosedFolderTitle;
            }
        }
        if matches!(item.item_type, NavItemType::ClosedFolderIndex) {
            let page_folders = self.page_folders(args);
            if page_folders
                .into_iter()
                .take(item.folders.len())
                .collect::<Vec<String>>()
                == item.folders
            {
                item.item_type = NavItemType::OpenedFolderIndex;
            } else {
                item.item_type = NavItemType::ClosedFolderIndex;
            }
        }
        item.children
            .iter_mut()
            .for_each(|i| self.folder_menu_set_open_closed_folders(args, i))
        // item.folders.iter().enumerate().for_each(|(index, folder)| {
        //     dbg!("asdf");
        //     ()
        // });
        // if item
        //     .folders
        //     .iter()
        //     .all(|folder| page_folders.contains(folder))
        // {
        //     item.item_type = NavItemType::OpenFolderTitle;
        // } else {
        //     dbg!(&page_folders);
        //     dbg!(&item.folders);
        //     item.item_type = NavItemType::ClosedFolderTitle;
        // }
    }

    ///////////////////////////////////////////////////////////
    // TODO: Move over as dynamic function
    pub fn folder_menu_sort_by_path(&self, items: &mut Vec<NavItem>) {
        items.sort_by_key(|k| k.path_sort_string.clone());
        items
            .iter_mut()
            .for_each(|item| self.folder_menu_sort_by_path(&mut item.children));
    }

    ////////////////////////////////////////////////////////////////
    // MOVED
    #[instrument(skip(self))]
    pub fn folder_menu_subfolder_finder(&self, pattern: &Vec<String>) -> Vec<NavItem> {
        event!(Level::INFO, "fn folder_menu_subfolder_finder");
        let mut next_level_folders: BTreeSet<Vec<String>> = BTreeSet::new();
        self.pages.iter().for_each(|page| {
            let page_folders = self.page_folders(&[Value::from(page.1.id.clone())]);
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
            .filter_map(|pat| self.folder_menu_index_finder(pat.clone()))
            .collect()
    }

    ///////////////////////////////////////////////////////////////////////
    //// SKIP
    //pub fn nav_from_files_and_folders_dev(&self, args: &[Value]) -> NavTree {
    //    let mut nav_links = NavTree {
    //        items: self.folder_menu(args),
    //        prev_item: None,
    //        next_item: None,
    //        prev_next_order: vec![],
    //    };
    //    self.set_current_file_for_nav_links(&args[0].to_string(), &mut nav_links);
    //    self.find_prev_next_nav_links(&args[0].to_string(), &mut nav_links);
    //    nav_links
    //}

    ///////////////////////////////////////////////////////////////////////
    //// SKIP
    //pub fn nav_from_files_and_folders(&self, args: &[Value]) -> NavTree {
    //    let mut nav_links = NavTree {
    //        items: self.folder_menu(args),
    //        prev_item: None,
    //        next_item: None,
    //        prev_next_order: vec![],
    //    };
    //    self.set_current_file_for_nav_links(&args[0].to_string(), &mut nav_links);
    //    nav_links
    //}

    /////////////////////////////////////////////////////////////////////
    // TODO: Move this back into the main site mod.rs
    pub fn nav_link_title_link(&self, args: &[Value]) -> Option<String> {
        Some(format!(
            r#"<a href="{}">{}</a>"#,
            self.page_href(args).unwrap(),
            self.page_title(args).unwrap()
        ))
    }

    /////////////////////////////////////////////////////////////////////
    // MOVED
    pub fn set_current_file_for_nav_links(&self, id: &String, nav_links: &mut NavTree) {
        nav_links
            .items
            .iter_mut()
            .for_each(|item| self.set_current_file_for_nav_link_for_item(id, item))
    }

    /////////////////////////////////////////////////////////////////////
    // MOVED
    pub fn set_current_file_for_nav_link_for_item(&self, id: &String, item: &mut NavItem) {
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
            .for_each(|i| self.set_current_file_for_nav_link_for_item(id, i));
    }
}

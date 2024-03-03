use crate::nav_item::NavItem;
use crate::nav_item::NavItemType;
use crate::nav_tree::NavPrevNextItem;
use crate::nav_tree::NavTree;
use crate::site::CacheObject;
use crate::site::Site;
use itertools::Itertools;
use minijinja::Value;
use tracing::{event, instrument, Level};

impl Site {
    pub fn folder_menu(&self, args: &[Value]) -> Vec<NavItem> {
        let mut items = self.folder_menu_builder(args);
        items
            .iter_mut()
            .for_each(|item| self.folder_menu_set_open_closed_folders(args, item));
        items
    }

    #[instrument(skip(self))]
    pub fn folder_menu_builder(&self, args: &[Value]) -> Vec<NavItem> {
        event!(Level::INFO, "{}", args[0].to_string());
        let menu_key = args[1]
            .try_iter()
            .unwrap()
            .into_iter()
            .map(|f| f.try_iter().unwrap().into_iter().join("-"))
            .join("-");
        match self.get_cache("menus", &menu_key) {
            Some(menu_cache) => {
                if let CacheObject::Menu(menu) = menu_cache {
                    menu
                } else {
                    vec![]
                }
            }
            None => {
                let mut r: Vec<NavItem> = args[1]
                    .try_iter()
                    .unwrap()
                    .filter_map(|folder_vecs| {
                        let folder_pattern: Vec<String> = folder_vecs
                            .try_iter()
                            .unwrap()
                            .map(|f| f.to_string())
                            .collect();
                        self.folder_menu_index_finder(folder_pattern)
                    })
                    .collect();
                r.iter_mut()
                    .for_each(|item| self.folder_menu_sort_by_path(&mut item.children));
                self.set_cache("menus", menu_key, CacheObject::Menu(r.clone()));
                r
            }
        }
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

    pub fn folder_menu_sort_by_path(&self, items: &mut Vec<NavItem>) {
        items.sort_by_key(|k| k.path_sort_string.clone());
        items
            .iter_mut()
            .for_each(|item| self.folder_menu_sort_by_path(&mut item.children));
    }

    pub fn find_prev_next_nav_links(&self, _id: &String, links: &mut NavTree) {
        links.next_item = Some(NavPrevNextItem {
            page_id: "content-bravo".to_string(),
            title_link_or_text: None,
        });
        dbg!("here");
    }

    pub fn nav_from_files_and_folders_dev(&self, args: &[Value]) -> NavTree {
        let mut nav_links = NavTree {
            items: self.folder_menu(args),
            prev_item: None,
            next_item: None,
            prev_next_order: vec![],
        };
        self.set_current_file_for_nav_links(&args[0].to_string(), &mut nav_links);
        self.find_prev_next_nav_links(&args[0].to_string(), &mut nav_links);
        nav_links
    }

    pub fn nav_from_files_and_folders(&self, args: &[Value]) -> NavTree {
        let mut nav_links = NavTree {
            items: self.folder_menu(args),
            prev_item: None,
            next_item: None,
            prev_next_order: vec![],
        };
        self.set_current_file_for_nav_links(&args[0].to_string(), &mut nav_links);
        nav_links
    }

    // TODO: Set this up so it pulls actual link text
    pub fn nav_link_title_link(&self, args: &[Value]) -> Option<String> {
        Some(format!(
            r#"<a href="{}">{}</a>"#,
            self.page_href(args).unwrap(),
            self.page_title(args).unwrap()
        ))
    }
}

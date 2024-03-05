use crate::collection::Collection;
use crate::collection::CollectionItem;
use crate::collection::CollectionItemType;
use crate::page::Page;
use minijinja::Value;
use std::collections::BTreeMap;

impl Collection {
    pub fn new_from_files_and_folders(
        pages: &BTreeMap<String, Page>,
        args: &[Value],
    ) -> Collection {
        let items: Vec<CollectionItem> = args[0]
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
        let c = Collection { items };
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
            page_id: id.clone(),
            base_type: CollectionItemType::File,
            children: vec![],
        })
    } else {
        None

        // let mut full_pattern_with_title = pattern.clone();
        // full_pattern_with_title.push("_title.neo".to_string());
        // let mut full_pattern_with_index = pattern.clone();
        // full_pattern_with_index.push("_index.neo".to_string());
        // site.pages.iter().find_map(|page| {
        //     event!(Level::DEBUG, "{}", page.0);
        //     let page_args = [Value::from(page.1.id.clone())];
        //     let mut next_parent_ids = parent_ids.clone();
        //     next_parent_ids.push(page.1.id.clone());
        //     if full_pattern_with_title == site.page_path_parts(&[Value::from(page.1.id.clone())]) {
        //         let mut fmi = NavItem {
        //             children: folder_menu_child_item_finder(
        //                 site,
        //                 &pattern,
        //                 next_parent_ids.clone(),
        //             ),
        //             folders: site.page_folders(&page_args),
        //             href: site.page_href(&[Value::from(page.1.id.clone())]),
        //             item_type: NavItemType::TitleFolderClosed,
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
        //             item_type: NavItemType::IndexFolderClosed,
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
        //     } else {
        //         None
        //     }
        // })
    }
}

// DEPRECATED:
// Delete this when all the features are implemented
// in collections

// fn do_sort_by_source_path(items: &mut Vec<NavItem>) {
//     items.sort_by_key(|k| k.path_sort_string.clone());
//     items
//         .iter_mut()
//         .for_each(|item| do_sort_by_source_path(&mut item.children));
// }

// fn load_prev_next(tree: &Vec<NavItem>) -> Vec<NavItem> {
//     let mut prev_next_vec: Vec<NavItem> = vec![];
//     prev_next_flattener(tree, &mut prev_next_vec);
//     prev_next_vec
// }

// fn prev_next_flattener(items: &Vec<NavItem>, dest: &mut Vec<NavItem>) {
//     items.iter().for_each(|item| {
//         if !matches![item.item_type, NavItemType::TitleFolderOpened]
//             && !matches![item.item_type, NavItemType::TitleFolderClosed]
//         {
//             let mut prev_next_item = item.clone();
//             // Children are removed from prev_next items to avoid
//             // exploding trees with unused values
//             prev_next_item.children = vec![];
//             dest.push(prev_next_item);
//         }
//         prev_next_flattener(&item.children, dest);
//     });
// }

//     fn folder_menu_set_open_closed_folders(&self, args: &[Value], item: &mut NavItem) {
//         if matches!(item.item_type, NavItemType::TitleFolderClosed) {
//             let page_folders = self.page_folders(args);
//             if page_folders
//                 .into_iter()
//                 .take(item.folders.len())
//                 .collect::<Vec<String>>()
//                 == item.folders
//             {
//                 item.item_type = NavItemType::TitleFolderOpened;
//             } else {
//                 item.item_type = NavItemType::TitleFolderClosed;
//             }
//         }
//         if matches!(item.item_type, NavItemType::IndexFolderClosed) {
//             let page_folders = self.page_folders(args);
//             if page_folders
//                 .into_iter()
//                 .take(item.folders.len())
//                 .collect::<Vec<String>>()
//                 == item.folders
//             {
//                 item.item_type = NavItemType::IndexFolderOpened;
//             } else {
//                 item.item_type = NavItemType::IndexFolderClosed;
//             }
//         }
//         item.children
//             .iter_mut()
//             .for_each(|i| self.folder_menu_set_open_closed_folders(args, i))
//         // item.folders.iter().enumerate().for_each(|(index, folder)| {
//         //     dbg!("asdf");
//         //     ()
//         // });
//         // if item
//         //     .folders
//         //     .iter()
//         //     .all(|folder| page_folders.contains(folder))
//         // {
//         //     item.item_type = NavItemType::OpenFolderTitle;
//         // } else {
//         //     dbg!(&page_folders);
//         //     dbg!(&item.folders);
//         //     item.item_type = NavItemType::TitleFolderClosed;
//         // }
//     }

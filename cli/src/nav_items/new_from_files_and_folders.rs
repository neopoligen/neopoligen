// DEPRECATED:
// Delete this when all the features are implemented
// in collections

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

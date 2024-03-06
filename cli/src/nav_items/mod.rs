// DEPRECATED:
//
// deleting from this file when things are moved
// to collections. once everything's there this
// can be delted

// #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
// #[serde(tag = "type", rename_all = "lowercase")]
// pub struct NavItems {
//     pub tree: Vec<NavItem>,
//     pub prev_next_items: Vec<NavItem>,
//     pub next_item: Option<NavItem>,
//     pub prev_item: Option<NavItem>,
//     pub open_folders: Vec<String>,
//     pub current_item: Option<NavItem>,
// }

// impl NavItems {
//     pub fn tree_items_from(&self, args: &[Value]) -> Vec<NavId> {
//         let page_id = args[0].to_string();
//         match self
//             .tree
//             .iter()
//             .find_map(|item| is_tree_sub_root(item, &page_id))
//         {
//             Some(sub_root) => sub_root
//                 .children
//                 .iter()
//                 .map(|child| get_nav_id(child))
//                 .collect(),
//             None => vec![],
//         }
//     }
// }

// fn get_prev_item(key: &String, items: &Vec<NavItem>) -> Option<NavItem> {
//     match items.iter().position(|test_item| &test_item.page_id == key) {
//         Some(index) => {
//             if index > 0 {
//                 let prev_next_item = items.get(index - 1).unwrap().clone();
//                 Some(prev_next_item)
//             } else {
//                 None
//             }
//         }
//         None => None,
//     }
// }

// fn get_next_item(key: &String, items: &Vec<NavItem>) -> Option<NavItem> {
//     match items.iter().position(|test_item| &test_item.page_id == key) {
//         Some(index) => match items.get(index + 1) {
//             Some(item) => Some(item.clone()),
//             None => None,
//         },
//         None => None,
//     }
// }

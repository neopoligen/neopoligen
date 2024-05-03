// DEPRECATED:
//
// deleting from this file when things are moved
// to collections. once everything's there this
// can be delted

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

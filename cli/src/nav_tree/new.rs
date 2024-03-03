use crate::nav_tree::NavTree;
use minijinja::Value;

impl NavTree {
    pub fn new_from_files_and_folders(_args: &[Value]) -> NavTree {
        NavTree {
            items: vec![],
            prev_item: None,
            next_item: None,
            prev_next_order: vec![],
        }
    }
}

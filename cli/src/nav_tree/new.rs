use crate::nav_tree::NavTree;
use minijinja::Value;

impl NavTree {
    pub fn new_from_files_and_folders(args: &[Value]) -> NavTree {
        NavTree { items: vec![] }
    }
}

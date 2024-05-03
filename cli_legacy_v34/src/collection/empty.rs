use crate::collection::Collection;

impl Collection {
    pub fn empty() -> Collection {
        Collection {
            active_ancestors: vec![],
            active_folders: vec![],
            next_item: None,
            prev_item: None,
            prev_next_list: vec![],
            tree: vec![],
        }
    }
}

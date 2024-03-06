use crate::collection::Collection;

impl Collection {
    pub fn empty() -> Collection {
        Collection {
            active_ancestors: vec![],
            active_folders: vec![],
            tree: vec![],
        }
    }
}

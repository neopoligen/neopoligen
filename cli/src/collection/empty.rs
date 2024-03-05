use crate::collection::Collection;

impl Collection {
    pub fn empty() -> Collection {
        Collection {
            tree: vec![],
            active_folders: vec![],
        }
    }
}

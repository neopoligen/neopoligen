use crate::file_set::FileSet;
use std::collections::BTreeMap;

impl FileSet {
    pub fn new() -> FileSet {
        FileSet {
            content: BTreeMap::new(),
            templates: BTreeMap::new(),
        }
    }
}

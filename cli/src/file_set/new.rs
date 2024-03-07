use crate::file_set::FileSet;
use std::collections::BTreeMap;
use std::path::PathBuf;

impl FileSet {
    pub fn new() -> FileSet {
        FileSet {
            pages: BTreeMap::new(),
            templates: BTreeMap::new(),
            images: vec![],
        }
    }
}

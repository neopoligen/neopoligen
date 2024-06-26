use crate::file_set::FileSet;
use std::collections::BTreeMap;

impl FileSet {
    pub fn new() -> FileSet {
        FileSet {
            pages: BTreeMap::new(),
            templates: BTreeMap::new(),
            images: vec![],
            mp3s: vec![],
        }
    }
}

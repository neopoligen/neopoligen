pub mod mocks;

use crate::page::Page;
use std::collections::BTreeMap;
use std::sync::Mutex;

pub struct Site {
    pub pages: BTreeMap<String, Page>,
    pub cache: Mutex<BTreeMap<String, BTreeMap<String, String>>>,
}

impl Site {
    pub fn page_title(&self, id: &str) -> &str {
        // let mut cache = self.cache.lock().unwrap();
        // match cache.get("page_title") {
        //     Some(c) => {}
        //     None => {}
        // }
        "Site 1 Home Page"
    }
}

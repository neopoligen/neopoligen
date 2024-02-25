pub mod builders;

use crate::page::Page;
use std::collections::BTreeMap;

pub struct Site {
    pub pages: BTreeMap<String, Page>,
}


impl Site {
    pub fn page_title(&self, id: &str) -> &str {
        "Site 1 Home Page"
    }
}

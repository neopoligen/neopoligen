pub mod object;

use crate::page_v39::PageV39;
use std::collections::BTreeMap;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct SiteV39 {
    pages: BTreeMap<String, PageV39>,
}

impl SiteV39 {
    pub fn new(source_pages: &BTreeMap<PathBuf, PageV39>) -> SiteV39 {
        let mut pages = BTreeMap::new();
        for (_, page) in source_pages.iter() {
            if let Some(id) = page.id() {
                pages.insert(id, page.clone());
            }
        }
        SiteV39 { pages }
    }
}

use crate::page::Page;
use crate::page_data::PageData;
use crate::site::Site;
use std::collections::BTreeMap;

impl Site {
    pub fn new(pages: &BTreeMap<String, Page>) -> Site {
        let mut page_data: BTreeMap<String, PageData> = BTreeMap::new();
        for (page_id, page) in pages.iter() {
            page_data.insert(page_id.clone(), PageData::new(page));
        }
        Site { page_data }
    }
}

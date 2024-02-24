use crate::page::Page;
use crate::page_data::PageData;

impl PageData {
    pub fn new(page: &Page) -> PageData {
        PageData {
            filters: page.filters(),
            tags: page.tags(),
            full_title: page.full_title(),
            title_for_url: page.title_for_url(),
            url_path: page.url_path(),
        }
    }
}

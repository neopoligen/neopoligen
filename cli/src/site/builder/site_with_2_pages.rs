use crate::page::Page;
use crate::site::Site;
use std::collections::BTreeMap;

impl Site {
    pub fn site_with_2_pages() -> Site {
        let mut pages: BTreeMap<String, Page> = BTreeMap::new();
        let p1 = Page::id12345c_tags();
        pages.insert(p1.id().unwrap(), p1);
        let p2 = Page::test_with_tags_2();
        pages.insert(p2.id().unwrap(), p2);
        let site = Site::new(&pages);
        site
    }
}

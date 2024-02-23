use crate::page::Page;
use crate::site::Site;
use std::collections::BTreeMap;

impl Site {
    pub fn site_with_6_pages() -> Site {
        let mut pages: BTreeMap<String, Page> = BTreeMap::new();
        let p1 = Page::id12345c_tags();
        let p2 = Page::test_with_tags_2();
        let p3 = Page::test_with_output_path();
        let p4 = Page::test_with_output_to_root_index_html();
        let p5 = Page::id12345d_title_and_metadata();
        let p6 = Page::test_with_title_metadata_and_one_p();
        let p7 = Page::test_with_filters_section();
        pages.insert(p1.id().unwrap(), p1);
        pages.insert(p2.id().unwrap(), p2);
        pages.insert(p3.id().unwrap(), p3);
        pages.insert(p4.id().unwrap(), p4);
        pages.insert(p5.id().unwrap(), p5);
        pages.insert(p6.id().unwrap(), p6);
        pages.insert(p7.id().unwrap(), p7);
        let site = Site::new(&pages);
        site
    }
}

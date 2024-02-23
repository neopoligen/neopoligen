use crate::config::Config;
use crate::page::Page;
use crate::site_v2::SiteV2;
use std::collections::BTreeMap;

impl SiteV2 {
    pub fn site_with_eight_pages() -> SiteV2 {
        let mut site = SiteV2 {
            config: Config::mock_basic_config(),
            pages: BTreeMap::new(),
            page_templates: BTreeMap::new(),
        };
        let p1 = Page::id12345c_tags();
        let p2 = Page::test_with_tags_2();
        let p3 = Page::test_with_output_path();
        let p4 = Page::test_with_output_to_root_index_html();
        let p5 = Page::id12345d_title_and_metadata();
        let p6 = Page::test_with_title_metadata_and_one_p();
        let p7 = Page::test_with_filters_section();
        let p8 = Page::title_in_bookmark();
        site.pages.insert(p1.id().unwrap(), p1);
        site.pages.insert(p2.id().unwrap(), p2);
        site.pages.insert(p3.id().unwrap(), p3);
        site.pages.insert(p4.id().unwrap(), p4);
        site.pages.insert(p5.id().unwrap(), p5);
        site.pages.insert(p6.id().unwrap(), p6);
        site.pages.insert(p7.id().unwrap(), p7);
        site.pages.insert(p8.id().unwrap(), p8);
        site
    }
}

use crate::config::Config;
use crate::page::Page;
use crate::site_v2::SiteV2;
use std::collections::BTreeMap;
use std::sync::Mutex;

impl SiteV2 {
    pub fn folder_menu_test_site() -> SiteV2 {
        let mut site = SiteV2 {
            config: Config::mock_basic_config(),
            pages: BTreeMap::new(),
            page_templates: BTreeMap::new(),
            holder: Mutex::new(BTreeMap::new()),
        };
        let page_set = vec![
            Page::level1a_index(),
            Page::level1a_file1(),
            Page::level1b_index(),
            Page::level2a_index(),
        ];
        page_set.iter().for_each(|p| {
            site.pages.insert(p.id().unwrap(), p.clone());
        });
        site
    }
}

use crate::page_v39::PageV39;
use crate::site_config::SiteConfig;
use crate::site_v39::SiteV39;
use std::collections::BTreeMap;

impl SiteV39 {
    pub fn mock1() -> SiteV39 {
        let config = SiteConfig::mock1();
        let mut pages = BTreeMap::new();
        pages.insert(
            "20240101alfa1234".to_string(),
            PageV39::mock_1_20240101_basic_page(),
        );
        pages.insert(
            "20240102bravo123".to_string(),
            PageV39::mock_2_20240102_with_type_and_status(),
        );
        SiteV39 { config, pages }
    }
}



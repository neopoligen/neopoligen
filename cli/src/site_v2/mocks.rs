use crate::page_v2::PageV2;
use crate::site_config::SiteConfig;
use crate::site_v2::SiteV2;
use std::collections::BTreeMap;

impl SiteV2 {
    pub fn mock1() -> SiteV2 {
        let mut pages = BTreeMap::new();
        pages.insert("abcd1234".to_string(), PageV2::mock_1_with_ast());
        pages.insert("bravo123".to_string(), PageV2::mock_2_home_page());
        pages.insert("charlie1".to_string(), PageV2::mock_3_bookmark_section());
        pages.insert("delta123".to_string(), PageV2::mock_4_title_from_text());
        pages.insert("echo1234".to_string(), PageV2::mock_5_no_title());
        pages.insert("foxtrot1".to_string(), PageV2::mock_6_url_title_parsing());
        pages.insert("golf1234".to_string(), PageV2::mock_7_golf1234());
        pages.insert("hotel123".to_string(), PageV2::mock_8_hotel123());
        SiteV2 {
            config: SiteConfig::mock1(),
            pages,
            images: BTreeMap::new(),
            mp3s: BTreeMap::new(),
        }
    }
}

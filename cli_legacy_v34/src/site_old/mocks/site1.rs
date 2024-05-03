use crate::config::Config;
use crate::page::Page;
use crate::site::Site;

impl Site {
    pub fn site1() -> Site {
        let config = Config::set1();
        let mut site = Site::new(config);
        site.pages.insert("id_index".to_string(), Page::s1_index());
        site.pages
            .insert("page-alfa".to_string(), Page::s1_page_alfa());
        site
    }
}

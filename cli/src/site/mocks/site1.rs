use crate::config::Config;
use crate::page::Page;
use crate::site::Site;

impl Site {
    pub fn site1() -> Site {
        let config = Config::site1_config();
        let mut site = Site::new(config);
        site.pages.insert("id_index".to_string(), Page::s1_index());
        site
    }
}

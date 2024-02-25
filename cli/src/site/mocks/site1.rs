use crate::page::Page;
use crate::site::Site;

impl Site {
    pub fn site1() -> Site {
        let mut site = Site::new();
        site.pages
            .insert("site1_index".to_string(), Page::site1_index());
        site
    }
}

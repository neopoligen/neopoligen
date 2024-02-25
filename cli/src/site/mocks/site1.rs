use crate::page::Page;
use crate::site::Site;

impl Site {
    pub fn site1() -> Site {
        let mut site = Site::new();
        site.pages
            .insert("site1_index".to_string(), Page::site1_index());
        site.pages.insert(
            "site1_title_with_inline_span".to_string(),
            Page::site1_title_with_inline_span(),
        );
        site.pages.insert(
            "site1_title_with_nested_spans".to_string(),
            Page::site1_title_with_nested_spans(),
        );
        site
    }
}

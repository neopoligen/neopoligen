use crate::page::Page;
use crate::site::Site;

impl Site {
    pub fn site1() -> Site {
        let mut site = Site::new();
        site.pages.insert("s1_index".to_string(), Page::s1_index());
        site.pages.insert(
            "s1_title_with_inline_span".to_string(),
            Page::s1_title_with_inline_span(),
        );
        site.pages.insert(
            "s1_title_with_nested_spans".to_string(),
            Page::s1_title_with_nested_spans(),
        );
        site.pages.insert(
            "s1_title_in_metadata".to_string(),
            Page::s1_title_in_metadata(),
        );
        site.pages
            .insert("s1_only_metadata".to_string(), Page::s1_only_metadata());
        site
    }
}

use crate::config::Config;
use crate::page::Page;
use crate::site::Site;

impl Site {
    pub fn site2() -> Site {
        let config = Config::set2();
        let mut site = Site::new(config);
        site.pages.insert("id_index".to_string(), Page::s2_index());
        site.pages
            .insert("id_only_metadata".to_string(), Page::s2_only_metadata());
        site.pages.insert(
            "id_title_from_content".to_string(),
            Page::s2_title_from_content(),
        );
        site.pages
            .insert("id_title_from_text".to_string(), Page::s2_title_from_text());
        site.pages.insert(
            "id_title_with_inline_span".to_string(),
            Page::s2_title_with_inline_span(),
        );
        site.pages.insert(
            "id_title_in_metadata".to_string(),
            Page::s2_title_in_metadata(),
        );
        site.pages.insert(
            "id_title_with_nested_spans".to_string(),
            Page::s2_title_with_nested_spans(),
        );
        site.pages.insert(
            "id-no-type-or-status-in-metadata".to_string(),
            Page::s2_no_type_or_status_in_metadata(),
        );
        site
    }
}

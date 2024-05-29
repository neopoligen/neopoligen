use crate::site_v2::SiteV2;
use minijinja::value::{Object, Value};
use minijinja::Error;
use std::fmt::Display;
use std::sync::Arc;

impl Object for SiteV2 {
    fn call_method(
        self: &Arc<SiteV2>,
        _state: &minijinja::State,
        name: &str,
        args: &[Value],
    ) -> Result<Value, Error> {
        match name {
            "base_url" => self.base_url(),
            "config" => self.config(),
            "collection_by_date" => self.collection_by_date(args),
            // TODO: "collection_by_title" => self.collection_by_title(args),
            // TODO: "collection_by_title_with_date" => self.collection_by_title_with_date(args),
            "get_image" => self.get_image(args),
            "get_mp3" => self.get_mp3(args),
            "page_format_date" => self.page_format_date(args),
            "page_href" => self.page_href(args),
            "page_og_image" => self.page_og_image(args),
            "page_permalink" => self.page_permalink(args),
            "page_sections" => self.page_sections(args),
            "page_title_as_plain_text" => self.page_title_as_plain_text(args),
            // DEPRECATED: Change to collection_*
            //"get_pages_by_date" => self.get_pages_by_date(args),
            "theme" => self.theme(),
            _ => Ok(Value::from("")),
        }
    }
}

impl Display for SiteV2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "site_object")
    }
}

// DEPRECATED: MOVE this functionality
// into site_v2
//
use crate::page::Page;
use minijinja::Value;

impl Page {
    pub fn url_for_page(&self, _args: &[Value]) -> Option<String> {
        Some("FIX: url_for_page".to_string())
        // self.site.clone().unwrap().url_for_page(args)
    }
}

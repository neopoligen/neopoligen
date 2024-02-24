use crate::page::Page;
use minijinja::Value;

impl Page {
    pub fn full_title_for_page(&self, args: &[Value]) -> Option<String> {
        self.site.clone().unwrap().full_title(args)
    }
}

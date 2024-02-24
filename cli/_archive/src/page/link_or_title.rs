use crate::page::Page;
use minijinja::Value;

impl Page {
    pub fn link_or_title(&self, args: &[Value]) -> Value {
        self.site.clone().unwrap().link_or_title(args)
    }
}

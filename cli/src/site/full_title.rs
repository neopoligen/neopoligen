use crate::site::Site;
use minijinja::Value;

impl Site {
    pub fn full_title(&self, args: &[Value]) -> Option<String> {
        match self
            .page_data
            .get(&args[0].to_string())
            .map(|data| data.full_title.clone())
        {
            Some(response) => response,
            None => None,
        }
    }
}

use crate::site::Site;
use minijinja::Value;

impl Site {
    pub fn title_for_url(&self, args: &[Value]) -> Option<String> {
        match self.page_data.get(&args[0].to_string()).map(|data| data.title_for_url.clone()) {
            Some(response) => response,
            None => None
        }
    }
}


use crate::site::Site;
use minijinja::Value;

impl Site {
    pub fn url_for_page(&self, args: &[Value]) -> Option<String> {
        match self
            .page_data
            .get(&args[0].to_string())
            .map(|data| data.url_path.clone())
        {
            Some(response) => response,
            None => None,
        }
    }
}

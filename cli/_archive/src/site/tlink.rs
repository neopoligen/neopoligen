use crate::site::Site;
use minijinja::Value;

impl Site {
    pub fn tlink(&self, args: &[Value]) -> Option<String> {
        let pd = self.page_data.get(&args[0].to_string()).unwrap();
        Some(format!(
            r#"<a href="{}">{}</a>"#,
            pd.url_path.clone().unwrap(),
            pd.full_title.clone().unwrap()
        ))
    }
}

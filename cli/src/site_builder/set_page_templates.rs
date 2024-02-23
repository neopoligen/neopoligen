use crate::site_builder::SiteBuilder;

impl SiteBuilder<'_> {
    pub fn set_page_templates(&mut self) {
        for (page_id, page) in self.site.pages.iter() {
            let check_path = format!(
                "page-types/{}/{}.jinja",
                page.r#type().unwrap(),
                page.status().unwrap()
            );
            if let Ok(_) = self.env.get_template(check_path.as_str()) {
                self.site
                    .page_templates
                    .insert(page_id.to_string(), check_path);
            } else {
                let check_path = format!("page-types/{}/published.jinja", page.r#type().unwrap());
                if let Ok(_) = self.env.get_template(check_path.as_str()) {
                    self.site
                        .page_templates
                        .insert(page_id.to_string(), check_path);
                } else {
                    let fallback_path = "page-types/post/published.jinja".to_string();
                    self.site
                        .page_templates
                        .insert(page_id.to_string(), fallback_path);
                }
            }
        }
    }
}

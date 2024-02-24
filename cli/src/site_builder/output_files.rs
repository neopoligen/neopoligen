use crate::site_builder::SiteBuilder;
use crate::site_v2::SiteV2;
use minijinja::context;
use minijinja::Value;
use std::fs;
use std::path::PathBuf;

impl SiteBuilder<'_> {
    pub fn output_files(&self) {
        let template_file = "includes/splitter.jinja";
        let mut site = SiteV2::new(self.config.clone());
        site.load_pages();

        /////////////////////////////////////
        // Here be dragons
        //
        //

        for (page_id, page) in site.pages.iter() {
            let check_path = format!(
                "page-types/{}/{}.jinja",
                page.r#type().unwrap(),
                page.status().unwrap()
            );
            if let Ok(_) = self.env.get_template(check_path.as_str()) {
                site.page_templates.insert(page_id.to_string(), check_path);
            } else {
                let check_path = format!("page-types/{}/published.jinja", page.r#type().unwrap());
                if let Ok(_) = self.env.get_template(check_path.as_str()) {
                    site.page_templates.insert(page_id.to_string(), check_path);
                } else {
                    let fallback_path = "page-types/post/published.jinja".to_string();
                    site.page_templates
                        .insert(page_id.to_string(), fallback_path);
                }
            }
        }

        ///////////////////////////////////////

        match self.env.get_template(template_file) {
            Ok(tmpl) => {
                match tmpl.render(context!(
                    // site => Value::from_object(self.site.clone()),
                     site => Value::from_object(site),
                )) {
                    Ok(all_pages) => {
                        self.empty_output_dir();
                        self.make_output_dirs();
                        let pages: Vec<_> = all_pages.split("--- PAGE_SEPERATOR ---").collect();
                        pages.iter().for_each(|p| {
                            let parts: Vec<_> = p.split("--- PAGE_DATA_SPLIT ---").collect();
                            if parts.len() == 2 {
                                let output_path = PathBuf::from(parts[0].trim());
                                match fs::write(output_path, parts[1]) {
                                    Ok(_) => {}
                                    Err(e) => println!("{}", e),
                                };
                            }
                        });
                    }
                    Err(e) => {
                        // // Error happened while building the splitter file
                        // // Doing a run through the files individually to
                        // // help troubleshoot
                        println!(
                            "{}\n-- TODO: Set flag in output_files() to rerun with logging on",
                            e
                        );
                    }
                }
            }
            Err(e) => println!("{}", e),
        }
    }
}

use crate::site_builder::SiteBuilder;
use minijinja::context;
use minijinja::Value;
use std::fs;
use std::path::PathBuf;

impl SiteBuilder<'_> {
    pub fn output_files(&self) {
        let template_file = "includes/splitter.jinja";
        match self.env.get_template(template_file) {
            Ok(tmpl) => {
                match tmpl.render(context!(
                    site => Value::from_object(self.site.clone()),
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

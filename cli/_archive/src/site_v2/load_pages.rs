use crate::helpers::get_file_paths_for_extension::get_file_paths_for_extension;
use crate::page::Page;
use crate::site_v2::SiteV2;
use std::fs;

// use crate::helpers::make_parent_folder::make_parent_folder;
// use crate::site::Site;
// use crate::site_builder::SiteBuilder;
// use minijinja::context;
// use minijinja::Value;

impl SiteV2 {
    pub fn load_pages(&mut self) {
        println!("Loading pages");
        let source_files = get_file_paths_for_extension(
            &self.config.folders.site_production_content_root.clone(),
            "neo",
        );
        source_files.iter().for_each(|source_path| {
            if let Ok(page_string) = fs::read_to_string(source_path) {
                // let folder_path = input_path
                //     .parent()
                //     .unwrap()
                //     .strip_prefix(&self.config.folders.site_production_content_root.clone())
                //     .unwrap();
                // let page = Page::new(folder_path.to_path_buf(), &page_string, self.config.clone());

                let page = Page::new(source_path.to_path_buf(), &page_string, self.config.clone());
                if let Some(id) = page.id() {
                    self.pages.insert(id, page);
                }
            }
        });

        // // event!(Level::INFO, "build_site started");
        // // self.env.clear_templates();
        // self.load_default_templates();
        // self.load_custom_templates();
        // let input_files = get_file_paths_for_extension(&self.config.folders.content.clone(), "neo");
        // // let site = Site::new();
        // input_files.iter().for_each(|input_path| {
        //     if let Ok(page_string) = fs::read_to_string(input_path) {
        //         let page = Page::new(&page_string, self.config.clone());
        //         if let Some(output_path) = &page.output_path() {
        //             if make_parent_folder(output_path).is_ok() {
        //                 let requested_page_templated = format!(
        //                     "pages/{}/{}.jinja",
        //                     page.r#type().unwrap(),
        //                     page.status().unwrap()
        //                 );
        //                 let tmpl = match self.env.get_template(requested_page_templated.as_str()) {
        //                     Ok(t) => t,
        //                     Err(_e) => self.env.get_template("pages/post/published.jinja").unwrap(),
        //                 };
        //                 event!(Level::INFO, "start_render");
        //                 match tmpl.render(context!(page => Value::from_object(page))) {
        //                     Ok(output) => {
        //                         match fs::write(output_path, output) {
        //                             Ok(_) => {
        //                                 // println!("Output: {:?}", output_path);
        //                             }
        //                             Err(e) => println!("{}", e),
        //                         }
        //                     }
        //                     Err(e) => println!("{:?}", e),
        //                 };
        //                 event!(Level::INFO, "end_render");
        //             }
        //         } else {
        //             println!("No output path for {:?}", &input_path);
        //         }
        //     } else {
        //         println!("Could no open file: {:?}", &input_path);
        //     }
        // });
    }
}

#[cfg(test)]
mod test {
    // no tests at the moment
}

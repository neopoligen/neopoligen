// use crate::config::Config;
// use crate::helpers::get_file_paths_for_extension::get_file_paths_for_extension;

// use crate::helpers::get_file_paths_for_extension::get_file_paths_for_extension;
// use crate::helpers::get_files_with_extension_in_a_single_directory::get_files_with_extension_in_a_single_directory;
// use crate::page::Page;

use crate::site_builder::SiteBuilder;

// use crate::site_v2::SiteV2;
// use minijinja::context;
// use minijinja::Value;
// use std::fs;
// use std::path::PathBuf;

impl SiteBuilder<'_> {
    pub fn build_site(&mut self) {
        // TODO: Don't empty the directory
        // unless the new splitter file
        // build properly
        self.load_templates();
        self.set_global_template_variables();
        // if self.run_theme_page_tests() {
        // self.set_page_templates();
        self.output_files();
        self.copy_assets();
        self.copy_extras();
        self.deploy_images();
        self.make_cname_file();
        // } else {
        //     println!("Encountered template errors. Process stopped");
        // }
    }
}

// impl SiteBuilder<'_> {
//     pub fn run_theme_page_tests(&mut self) -> bool {
//         // This function is pretty hacky in order to call
//         // the templates from a mocked site for testing.
//         // It works as needed though so it's just gonna
//         // be that way. Also, I'm pretty sure this injects
//         // into the main environment even though I'm not
//         // currently seeing the test files output.
//         // So, basically, this needs an overhaul
//         println!("Running template tests");
//         let mut error_dir = self.config.folders.theme_tests_root.clone();
//         error_dir.push("_errors");
//         let _ = fs::remove_dir_all(error_dir.clone());
//         let mut total_tests: u32 = 0;
//         let mut failed_tests: u32 = 0;
//         get_file_paths_for_extension(&self.config.folders.theme_tests_root, "neo")
//             .iter()
//             .for_each(|test_file| {
//                 if let Ok(test_content_raw) = fs::read_to_string(&test_file) {
//                     // TODO: Load TOML from split1[0] for things
//                     // like ignoring tests
//                     let target_parts: Vec<&str> = test_content_raw.split("TARGET:").collect();
//                     let input_parts: Vec<&str> = target_parts[0].split("INPUT:").collect();
//                     target_parts.iter().skip(1).for_each(|target| {
//                         total_tests += 1;
//                         let search_string = target
//                             .trim()
//                             .replace(" ", "")
//                             .replace("\n", "")
//                             .replace("\t", "")
//                             .to_lowercase()
//                             .to_string();
//                         // dbg!(search_string);
//                         let theme_test_config = &self.config.clone();
//                         let mut site = SiteV2::new(theme_test_config.clone());
//                         let mut tmp_page_path =
//                             self.config.folders.site_production_content_root.clone();
//                         tmp_page_path.push("_page_builder");
//                         tmp_page_path.push("page.neo");
//                         let page =
//                             Page::new(tmp_page_path, input_parts[1], theme_test_config.clone());
//                         if let Some(id) = page.clone().id() {
//                             site.pages.insert(id, page.clone());
//                         };
//                         let mut test_templates_dir = self.config.folders.theme_tests_root.clone();
//                         test_templates_dir.push("_support");
//                         test_templates_dir.push("templates");
//                         get_files_with_extension_in_a_single_directory(
//                             &test_templates_dir,
//                             "jinja",
//                         )
//                         .iter()
//                         .for_each(|s| {
//                             let file_name = s.file_name().unwrap().to_string_lossy();
//                             let template_content = fs::read_to_string(s.clone()).unwrap();
//                             self.env
//                                 .add_template_owned(
//                                     format!("page-types/theme-test/{}", file_name),
//                                     template_content,
//                                 )
//                                 .unwrap();
//                         });
//                         let mut samples_dir =
//                             self.config.folders.site_production_content_root.clone();
//                         samples_dir.push("_theme_test_support");
//                         samples_dir.push("pages");
//                         get_file_paths_for_extension(&samples_dir, "neo")
//                             .iter()
//                             .for_each(|s| {
//                                 // let sample_stub_path = s.strip_prefix(samples_dir.clone()).unwrap();
//                                 let mut sample_stub_path = samples_dir.clone();
//                                 sample_stub_path.push("stitcher.neo");
//                                 let page_content = fs::read_to_string(s.clone()).unwrap();
//                                 let sample_page = Page::new(
//                                     PathBuf::from(sample_stub_path),
//                                     &page_content.to_string(),
//                                     theme_test_config.clone(),
//                                 );
//                                 if let Some(id) = sample_page.id() {
//                                     site.pages.insert(id, sample_page);
//                                 };
//                             });
//                         for (page_id, page) in site.pages.iter() {
//                             let check_path = format!(
//                                 "page-types/{}/{}.jinja",
//                                 page.r#type().unwrap(),
//                                 page.status().unwrap()
//                             );
//                             // dbg!(&check_path);
//                             if let Ok(_) = self.env.get_template(check_path.as_str()) {
//                                 site.page_templates.insert(page_id.to_string(), check_path);
//                             } else {
//                                 let check_path = format!(
//                                     "page-types/{}/published.jinja",
//                                     page.r#type().unwrap()
//                                 );
//                                 if let Ok(_) = self.env.get_template(check_path.as_str()) {
//                                     site.page_templates.insert(page_id.to_string(), check_path);
//                                 } else {
//                                     let fallback_path =
//                                         "page-types/post/published.jinja".to_string();
//                                     site.page_templates
//                                         .insert(page_id.to_string(), fallback_path);
//                                 }
//                             }
//                         }
//                         let splitter_template = "includes/splitter.jinja";
//                         match self.env.get_template(splitter_template) {
//                             Ok(tmpl) => {
//                                 match tmpl.render(context!(
//                                     site => Value::from_object(site.clone()),
//                                 )) {
//                                     Ok(all_pages) => {
//                                         let pages: Vec<_> =
//                                             all_pages.split("--- PAGE_SEPERATOR ---").collect();
//                                         pages.iter().for_each(|pg| {
//                                             let parts: Vec<_> =
//                                                 pg.split("--- PAGE_DATA_SPLIT ---").collect();
//                                             let tmp_file_path = PathBuf::from(parts[0].trim());
//                                             if let Some(dir_path) = tmp_file_path.parent() {
//                                                 if let Some(pid) = page.clone().id() {
//                                                     if pid
//                                                         == dir_path
//                                                             .file_name()
//                                                             .unwrap()
//                                                             .to_string_lossy()
//                                                             .to_string()
//                                                     {
//                                                         let under_test_string = parts[1]
//                                                             .replace(" ", "")
//                                                             .replace("\n", "")
//                                                             .replace("\t", "")
//                                                             .to_lowercase()
//                                                             .to_string();
//                                                         if under_test_string.as_str()
//                                                             != search_string.as_str()
//                                                         {
//                                                             failed_tests += 1;
//                                                             let _ = fs::create_dir_all(
//                                                                 error_dir.clone(),
//                                                             );
//                                                             let mut error_file = error_dir.clone();
//                                                             error_file.push(format!(
//                                                                 "{}.html",
//                                                                 test_file
//                                                                     .file_stem()
//                                                                     .unwrap()
//                                                                     .to_string_lossy()
//                                                             ));
//                                                             let _ = fs::write(
//                                                                 &error_file,
//                                                                 parts[1].trim(),
//                                                             );
//                                                             let template_display_path = test_file
//                                                                 .strip_prefix(
//                                                                     &self
//                                                                         .config
//                                                                         .folders
//                                                                         .site_configuration_root,
//                                                                 )
//                                                                 .unwrap();
//                                                             let error_display_path = error_file
//                                                                 .strip_prefix(
//                                                                     &self
//                                                                         .config
//                                                                         .folders
//                                                                         .site_configuration_root,
//                                                                 )
//                                                                 .unwrap();
//                                                             println!(
//                                                         "#### ERROR IN TEMPLATE: \n- Test File: {}",
//                                                         template_display_path.to_string_lossy()
//                                                     );
//                                                             println!(
//                                                                 "- Output File: {}",
//                                                                 error_display_path.display()
//                                                             );
//                                                         }
//                                                     }
//                                                 }
//                                             }
//                                         });
//                                     }
//                                     Err(e) => {
//                                         println!("{}", e)
//                                     }
//                                 }
//                             }
//                             Err(e) => {
//                                 println!("{}", e)
//                             }
//                         }
//                     });
//                 }
//             });
//         println!(
//             "Ran {} template test -  {} failed",
//             total_tests, failed_tests
//         );
//         if failed_tests == 0 {
//             true
//         } else {
//             false
//         }
//     }
// }

// // this won't work until things are set up
// // to only do a single changed file
// let last_edit_path = PathBuf::from(format!(
//     "{}/last-edit/index.html",
//     self.config.folders.site_output_root.display()
// ));
// let _ = make_parent_folder(&last_edit_path);

// for (page_id, page) in self.pages.iter_mut() {
//     page.site = Some(site.clone());
//     if let Some(output_path) = &page.output_path() {
//         if make_parent_folder(output_path).is_ok() {
//             let type_status_teamplate = format!(
//                 "page-types/{}/{}.jinja",
//                 page.r#type().unwrap(),
//                 page.status().unwrap()
//             );
//             let tmpl = match self.env.get_template(&type_status_teamplate) {
//                 Ok(t) => t,
//                 Err(_) => {
//                     let type_only_template =
//                         format!("page-types/{}/published.jinja", page.r#type().unwrap());
//                     match self.env.get_template(&type_only_template) {
//                         Ok(totmpl) => totmpl,
//                         Err(_) => self
//                             .env
//                             .get_template("page-types/post/published.jinja")
//                             .unwrap(),
//                     }
//                 }
//             };
//
//             match tmpl.render(context!(
//                 payload => Value::from_object(page.clone()),
//             )) {
//                 Ok(output) => {
//                     // Run a test for the page if there is one
//                     if let Some(tests) = self.page_tests.get(page_id) {
//                         let output_test_string = output
//                             .clone()
//                             .replace(" ", "")
//                             .replace("\t", "")
//                             .replace("\n", "");
//                         tests.iter().for_each(|test_payload| {
//                             let compressed_string = test_payload
//                                 .1
//                                 .replace(" ", "")
//                                 .replace("\t", "")
//                                 .replace("\n", "");
//                             if !output_test_string.contains(&compressed_string) {
//                                 println!(
//                                     "ERROR in: {}\n{}",
//                                     &test_payload.0, &test_payload.1
//                                 );
//                             }
//                         });
//                     };
//
//                     match fs::write(output_path, &output) {
//                         Ok(_) => {
//                             // this won't work until you do
//                             // things individually
//                             // let _ = fs::write(&last_edit_path, output);
//                         }
//                         Err(e) => println!("{}", e),
//                     }
//                 }
//                 Err(e) => println!("{:?}", e),
//             };
//         }
//     } else {
//         println!("No output path for {:?}", page_id);
//     }
// }
// self.copy_assets();
// self.copy_extras();
// self.copy_images();
// println!("Site build complete");

// let input_files = get_file_paths_for_extension(&self.config.folders.content.clone(), "neo");
// let site = Site::new();
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

#[cfg(test)]
mod test {
    // use super::*;
    // NOTE: This does an actual build right now
    // because I don't have a way at the moment
    // to build the site without starting the app
    // and that takes a few seconds which is not
    // to bad, but I'd rather not wait.
    // See: feature 2bqfpeqe for adding a command
    // line tool
    // #[test]
    // // #[ignore]
    // fn build_site_test() {
    //     let site_builder = SiteBuilder::mock_basic_builder();
    //     site_builder.build_site();
    // }
}

// impl Builder {
//     pub fn build_site(&self) {
//         ////////////////////////////////////////
//         // DEV HARD CODED VALUES
//         let theme_dir = "/Users/alan/Neopoligen/neopoligen-site/_themes/welcome/";
//         let local_site_root_dir = PathBuf::from("/Users/alan/Neopoligen/neopoligen-site");
//         ////////////////////////////////////////

//         println!("BUILDING SITE...");
//         let mut contend_dir = local_site_root_dir.clone();
//         contend_dir.push("_content_test");

//         let mut env = Environment::new();

//         let input_files = get_input_paths(&contend_dir, "neo");
//         input_files.iter().for_each(|input_path| {
//             if let Ok(page_string) = fs::read_to_string(input_path) {
//                 let config = Config::new(local_site_root_dir.clone());
//                 let page = Page::new(&page_string, config);
//                 if let Some(output_path) = &page.output_file_path() {
//                     // println!("Got output path for {:?}", &input_path);
//                     if make_parent_dir(output_path).is_ok() {
//                         // println!("Creating parent dir for: {:?}", &output_path);
//                         // try to make things go faster with the reload with clear_templates()
//                         env.clear_templates();
//                         env.set_loader(path_loader(theme_dir));
//                         let tmpl = env.get_template("pages/post/published.jinja").unwrap();
//                         match tmpl.render(context!(page => Value::from_object(page))) {
//                             Ok(output) => {
//                                 match fs::write(output_path, output) {
//                                     Ok(_) => {
//                                         // println!("Output: {:?}", output_path),
//                                     }
//                                     Err(e) => println!("{}", e),
//                                 }
//                             }
//                             Err(e) => println!("{:?}", e),
//                         };
//                     }
//                 } else {
//                     println!("No output path for {:?}", &input_path);
//                 }
//             } else {
//                 println!("Could no open file: {:?}", &input_path);
//             }
//         });
//     }
// }

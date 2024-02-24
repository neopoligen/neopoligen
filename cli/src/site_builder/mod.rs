pub mod build_site;
pub mod copy_assets;
pub mod copy_extras;
pub mod empty_output_dir;
pub mod load_pages;
pub mod load_templates;
pub mod make_output_dirs;
pub mod mocks;
pub mod new;
pub mod output_files;
pub mod set_page_templates;

use crate::config::Config;
use crate::page::Page;
use crate::site_v2::SiteV2;
use fs_extra::dir::copy;
use minijinja::Environment;
use std::collections::BTreeMap;
use std::fs;
use url::Url;

#[derive(Debug, Clone)]
pub struct SiteBuilder<'a> {
    pub config: Config,
    pub env: Environment<'a>,
    pub pages: BTreeMap<String, Page>,
    pub page_tests: BTreeMap<String, Vec<(String, String)>>,
    pub site: SiteV2,
}

impl SiteBuilder<'_> {
    pub fn deploy_images(&self) {
        let mut options = fs_extra::dir::CopyOptions::new();
        options.overwrite = true;
        options.content_only = true;
        let images_dir = &self.config.folders.site_images_root;
        let mut output_images_dir = self.config.folders.site_output_root.clone();
        output_images_dir.push("images");
        match copy(images_dir, output_images_dir, &options) {
            Ok(_) => (),
            Err(e) => println!("{}", e),
        }
    }

    pub fn make_cname_file(&self) {
        match Url::parse(self.config.domain.as_str()) {
            Ok(url) => {
                let mut cname_path = self.config.folders.site_output_root.clone();
                cname_path.push("CNAME");
                match url.host_str() {
                    Some(domain) => {
                        let _ = fs::write(cname_path, domain);
                    }
                    None => println!("Could not get domain name from config file"),
                }
            }
            Err(e) => println!("{}", e),
        }
    }

    pub fn set_global_template_variables(&mut self) {
        // TODO: Add default language here
        let mut global_vars = String::from("{%- set section_attribute_excludes = [");
        global_vars.push_str(r#"""#);
        global_vars.push_str(&self.config.section_attribute_excludes.join(r#"", ""#));
        global_vars.push_str(r#"""#);
        global_vars.push_str("] -%}");
        self.env
            .add_template_owned("global_vars", global_vars)
            .unwrap();
    }
}

#[cfg(test)]
mod test {
    // Be careful if you add tests since this
    // touches the file system
}

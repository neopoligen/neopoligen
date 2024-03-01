pub mod new;

use crate::config::Config;
use crate::file_set::FileSet;
use crate::site::Site;
use fs_extra::dir::copy;
use minijinja::context;
use minijinja::Environment;
use minijinja::Value;
use std::collections::BTreeMap;
use std::fs;
use std::fs::create_dir_all;
use std::path::PathBuf;

pub struct Builder {
    file_set: FileSet,
    config: Config,
}

impl Builder {
    pub fn copy_files(&self) {
        let mut options = fs_extra::dir::CopyOptions::new();
        options.overwrite = true;
        options.content_only = true;
        let extras_dir = self.config.folders.files_root.display().to_string();
        let site_output_root_dir = self.config.folders.output_root.display().to_string();
        match copy(extras_dir, site_output_root_dir, &options) {
            Ok(_) => (),
            Err(e) => println!("{}", e),
        }
    }

    pub fn copy_theme_assets(&self) {
        let mut options = fs_extra::dir::CopyOptions::new();
        options.overwrite = true;
        options.content_only = true;
        let in_dir = self
            .config
            .folders
            .theme_assets_input_root
            .display()
            .to_string();
        let site_output_root_dir = self
            .config
            .folders
            .theme_assets_output_root
            .display()
            .to_string();
        match copy(in_dir, site_output_root_dir, &options) {
            Ok(_) => (),
            Err(e) => println!("{}", e),
        }
    }

    pub fn files_to_output(&self) -> BTreeMap<PathBuf, String> {
        let mut env = Environment::new();
        let site = Site::new(&self.file_set, &self.config);
        let mut outputs = BTreeMap::new();

        self.file_set
            .templates
            .iter()
            .for_each(|t| env.add_template_owned(t.0, t.1).unwrap());

        env.add_template_owned(
            "splitter.jinja".to_string(),
            r#"{%- import "includes/macros.jinja" as macros -%}
{#- include "global_vars" -#}
{%- for page_id in site.page_ids() -%}
{{ site.log(page_id) }}
{{ site.page_output_path(page_id) }}
--- PAGE_DATA_SPLIT ---
{% include site.page_template(page_id) %}
--- PAGE_SEPARATOR ---
{% endfor -%}"#
                .to_string(),
        )
        .unwrap();

        match env.get_template("splitter.jinja") {
            Ok(splitter) => {
                match splitter.render(context!(
                     site => Value::from_object(site),
                )) {
                    Ok(combined_pages) => {
                        combined_pages
                            .split("--- PAGE_SEPARATOR ---")
                            .for_each(|page| {
                                let page_parts: Vec<_> =
                                    page.split("--- PAGE_DATA_SPLIT ---").collect();
                                if page_parts.len() == 2 {
                                    outputs.insert(
                                        PathBuf::from(page_parts[0].trim()),
                                        page_parts[1].trim().to_string(),
                                    );
                                }
                            });
                    }
                    Err(e) => {
                        println!("SPLITTER ERROR 1 - {}", e);
                        let mut redo_env = Environment::new();
                        self.file_set
                            .templates
                            .iter()
                            .for_each(|t| redo_env.add_template_owned(t.0, t.1).unwrap());
                        let site_redo_wrapper = Site::new(&self.file_set, &self.config);
                        site_redo_wrapper.pages.iter().for_each(|redo_page| {
                            let redo_site = Site::new(&self.file_set, &self.config);
                            let redo_template = redo_env
                                .get_template(
                                    &redo_site
                                        .page_template(&[Value::from(redo_page.0.to_string())])
                                        .unwrap(),
                                )
                                .unwrap();
                            match redo_template.render(context!(
                                site => Value::from_object(redo_site),
                                page_id => Value::from("2ck5rhdc")
                            )) {
                                // Ok(out) => println!("{}", out),
                                Ok(_) => (),
                                Err(e) => println!(
                                    "PAGE: {}\nERROR: {}\n",
                                    redo_page.1.source_path.display(),
                                    e
                                ),
                            };
                            // dbg!(redo_page.0);
                        });
                    }
                }
            }
            Err(e) => {
                println!("SPLITTER ERROR 2 - {}", e);
            }
        };
        outputs
    }

    pub fn write_files(&self) {
        println!("Writing files");
        self.files_to_output().iter().for_each(|f| {
            let output_path = PathBuf::from(f.0);
            // println!("{}", &f.0.display());
            let parent_dir = output_path.parent().unwrap();
            let _ = create_dir_all(parent_dir);
            let _ = fs::write(output_path, f.1);
        });
    }
}

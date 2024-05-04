use crate::page::Page;
// use crate::section::Section;
// use crate::section_attr::SectionAttr;
use crate::site_config::SiteConfigV2;
use minijinja::context;
//use minijinja::syntax;
// use crate::error::Error;
use minijinja::syntax::SyntaxConfig;
use minijinja::Environment;
use minijinja::Value;
use serde::Serialize;
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use tracing::{event, instrument, Level};
use walkdir::WalkDir;

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub struct Site {
    pub config: SiteConfigV2,
    pub page_errors: BTreeMap<PathBuf, Page>,
    pub missing_ids: BTreeMap<PathBuf, String>,
    pub pages: BTreeMap<String, Page>,
    pub source_files: BTreeMap<PathBuf, String>,
    pub templates: BTreeMap<String, String>,
}

impl Site {
    #[instrument]
    pub fn new(config: SiteConfigV2) -> Site {
        event!(Level::DEBUG, "Creating Site Object");
        Site {
            config,
            source_files: BTreeMap::new(),
            missing_ids: BTreeMap::new(),
            pages: BTreeMap::new(),
            page_errors: BTreeMap::new(),
            templates: BTreeMap::new(),
        }
    }
}

impl Site {
    //

    pub fn output_errors(&self) {
        // self.parsing_errors.iter().for_each(|p| {
        //     if let Err(e) = write_file_with_mkdir(&p.0, &p.1) {
        //         event!(Level::ERROR, "Could not write error file: {}", e);
        //     }
        // });
        // self.missing_ids.iter().for_each(|p| {
        //     let _ = write_file_with_mkdir(&p.0, &p.1);
        // });
    }

    pub fn generate_content_pages(&self) -> BTreeMap<PathBuf, String> {
        let mut outputs = BTreeMap::new();
        let mut env = Environment::new();
        let site_obj = Value::from_serialize(&self.clone());
        env.set_syntax(
            SyntaxConfig::builder()
                .block_delimiters("[!", "!]")
                .variable_delimiters("[@", "@]")
                .comment_delimiters("[#", "#]")
                .build()
                .unwrap(),
        );
        env.set_trim_blocks(true);
        env.set_lstrip_blocks(true);
        self.templates
            .iter()
            .for_each(|t| env.add_template_owned(t.0, t.1).unwrap());
        self.pages.iter().for_each(|p| {
            let template_name = "default.neoj";
            if let Ok(tmpl) = env.get_template(template_name) {
                match tmpl.render(context!(
                     site => site_obj,
                    page_id => p.0
                )) {
                    Ok(output) => {
                        outputs.insert(p.1.output_path.clone().unwrap(), output.clone());
                    }
                    Err(e) => {
                        event!(Level::ERROR, "{}", e)
                    }
                }
            } else {
                event!(Level::ERROR, "Could not get template: {}", template_name);
            }
            ()
        });
        outputs
    }

    pub fn parse_pages(&mut self) {
        self.source_files.iter().for_each(|f| {
            let p = Page::new(f.1.clone(), f.0.clone(), &self.config);
            if let Some(_) = p.error.clone() {
                self.page_errors.insert(p.output_path.clone().unwrap(), p);
            } else {
                self.pages.insert(p.id.clone().unwrap(), p);
            }
        });
    }

    //match ast(f.1, &self.config.sections) {
    //   Ok(ast) => {
    //  },
    //     Err(e) => {
    //}

    // if let Some(id) = get_page_id(&ast) {
    //     let mut output_path =
    //         self.config.paths.get("output_root").unwrap().to_path_buf();
    //     output_path.push(self.config.default_language.clone());
    //     output_path.push(id.clone());
    //     output_path.push("index.html");
    //     let page = Page {
    //         ast,
    //         id: id.clone(),
    //         source_path: f.0.clone(),
    //         output_path,
    //     };
    //     let _ = self.pages.insert(id, page).is_none();
    // } else {
    //     let _ = self
    //         .missing_ids
    //         .insert(
    //             error_file_path,
    //             format!("Missing ID: \n\n{}", f.1.to_string()),
    //         )
    //         .is_none();
    // }
    // }
    // Err(e) => {
    // self.parsing_errors.insert(error_file_path, e.to_string());
    // }

    // };

    // pub fn parse_pages(&mut self) {
    //     self.source_files.iter().for_each(|f| {
    //         let error_file_path = replace_path(
    //             &f.0,
    //             &self.config.paths.get("content_root").unwrap(),
    //             &self.config.paths.get("errors_root").unwrap(),
    //         )
    //         .unwrap()
    //         .with_extension("txt");
    //         match ast(f.1, &self.config.sections) {
    //             Ok(ast) => {
    //                 if let Some(id) = get_page_id(&ast) {
    //                     let mut output_path =
    //                         self.config.paths.get("output_root").unwrap().to_path_buf();
    //                     output_path.push(self.config.default_language.clone());
    //                     output_path.push(id.clone());
    //                     output_path.push("index.html");
    //                     let page = Page {
    //                         ast,
    //                         id: id.clone(),
    //                         source_path: f.0.clone(),
    //                         output_path,
    //                     };
    //                     let _ = self.pages.insert(id, page).is_none();
    //                 } else {
    //                     let _ = self
    //                         .missing_ids
    //                         .insert(
    //                             error_file_path,
    //                             format!("Missing ID: \n\n{}", f.1.to_string()),
    //                         )
    //                         .is_none();
    //                 }
    //             }
    //             Err(e) => {
    //                 self.parsing_errors.insert(error_file_path, e.to_string());
    //             }
    //         };
    //     });
    // }

    #[instrument]
    pub fn load_source_files(&mut self) {
        let dir = &self.config.paths.get("content_root").unwrap();
        if dir.exists() {
            WalkDir::new(dir)
                .into_iter()
                .filter(|entry| match entry.as_ref().unwrap().path().extension() {
                    Some(ext) => ext.to_str().unwrap() == "neo",
                    None => false,
                })
                .for_each(|entry| {
                    let path = entry.as_ref().unwrap().path().to_path_buf();
                    match fs::read_to_string(&path) {
                        Ok(content) => {
                            self.source_files.insert(path, content);
                        }
                        Err(e) => {
                            event!(Level::ERROR, "{}", e)
                        }
                    }
                });
        } else {
            event!(
                Level::ERROR,
                "Direcotory does not exist: {}",
                &dir.display()
            );
        }
    }

    pub fn load_templates(&mut self) {
        let mut templates_root = self.config.paths.get("themes_root").unwrap().to_path_buf();
        templates_root.push(self.config.theme.name.clone());
        templates_root.push("templates");
        if templates_root.exists() {
            WalkDir::new(templates_root.clone())
                .into_iter()
                .filter(|entry| match entry.as_ref().unwrap().path().extension() {
                    Some(ext) => ext.to_str().unwrap() == "neoj",
                    None => false,
                })
                .for_each(|entry| {
                    let path = entry.as_ref().unwrap().path().to_path_buf();
                    match fs::read_to_string(&path) {
                        Ok(content) => {
                            let template_name = path.strip_prefix(templates_root.clone()).unwrap();
                            self.templates
                                .insert(template_name.to_string_lossy().to_string(), content);
                        }
                        Err(e) => {
                            event!(Level::ERROR, "{}", e)
                        }
                    }
                });
        } else {
            event!(
                Level::ERROR,
                "Direcotory does not exist: {}",
                &templates_root.display()
            );
        }
    }

    //
}

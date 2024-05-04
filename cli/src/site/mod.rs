use crate::page::Page;
use crate::section::Section;
use crate::site_config::SiteConfigV2;
use crate::{ast::ast, section_attr::SectionAttr};
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
    pub content_files: BTreeMap<PathBuf, String>,
    pub missing_ids: BTreeMap<PathBuf, String>,
    pub pages: BTreeMap<String, Page>,
    pub parsing_errors: BTreeMap<PathBuf, String>,
}

impl Site {
    #[instrument]
    pub fn new(config: SiteConfigV2) -> Site {
        event!(Level::DEBUG, "Creating Site Object");
        Site {
            config,
            content_files: BTreeMap::new(),
            missing_ids: BTreeMap::new(),
            pages: BTreeMap::new(),
            parsing_errors: BTreeMap::new(),
        }
    }
}

impl Site {
    //

    pub fn output_errors(&self) {
        self.parsing_errors.iter().for_each(|p| {
            if let Err(e) = write_file_with_mkdir(&p.0, &p.1) {
                event!(Level::ERROR, "Could not write error file: {}", e);
            }
        });
        self.missing_ids.iter().for_each(|p| {
            let _ = write_file_with_mkdir(&p.0, &p.1);
        });
    }

    pub fn parse_pages(&mut self) {
        self.content_files.iter().for_each(|f| {
            let error_file_path = replace_path(
                &f.0,
                &self.config.paths.get("content_root").unwrap(),
                &self.config.paths.get("errors_root").unwrap(),
            )
            .unwrap()
            .with_extension("txt");
            match ast(f.1, &self.config.sections) {
                Ok(ast) => {
                    if let Some(id) = get_page_id(&ast) {
                        let mut output_path =
                            self.config.paths.get("output_root").unwrap().to_path_buf();
                        output_path.push("todo-fixme.txt");
                        let page = Page {
                            ast,
                            id: id.clone(),
                            source_path: f.0.clone(),
                            output_path,
                        };
                        let _ = self.pages.insert(id, page).is_none();
                    } else {
                        let _ = self
                            .missing_ids
                            .insert(
                                error_file_path,
                                format!("Missing ID: \n\n{}", f.1.to_string()),
                            )
                            .is_none();
                    }
                }
                Err(e) => {
                    self.parsing_errors.insert(error_file_path, e.to_string());
                }
            };
        });
    }

    #[instrument]
    pub fn load_pages(&mut self) {
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
                            self.content_files.insert(path, content);
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

    //
}

fn get_page_id(ast: &Vec<Section>) -> Option<String> {
    ast.iter().find_map(|sec_enum| {
        if let Section::Json { r#type, attrs, .. } = sec_enum {
            if r#type == "metadata" {
                attrs.iter().find_map(|attr| {
                    if let SectionAttr::KeyValue { key, value } = attr {
                        if key == "id" {
                            Some(value.trim().to_string())
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
            } else {
                None
            }
        } else {
            None
        }
    })
}

fn replace_path(path: &PathBuf, find: &PathBuf, replace: &PathBuf) -> Result<PathBuf, String> {
    match path.strip_prefix(find) {
        Ok(path_part) => Ok(replace.clone().join(path_part)),
        Err(e) => Err(format!("Problem: {}", e)),
    }
}

fn write_file_with_mkdir(path: &PathBuf, content: &str) -> Result<(), String> {
    match path.parent() {
        Some(parent_dir) => match fs::create_dir_all(parent_dir) {
            Ok(_) => match fs::write(path, content) {
                Ok(_) => Ok(()),
                Err(e) => Err(e.to_string()),
            },
            Err(e) => Err(e.to_string()),
        },
        None => Err("Could not make directory".to_string()),
    }
}

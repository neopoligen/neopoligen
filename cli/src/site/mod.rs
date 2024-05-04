use crate::ast::ast;
use crate::page::Page;
use crate::section::Section;
use crate::site_config::SiteConfig;
use serde::Serialize;
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use tracing::{event, instrument, Level};
use walkdir::WalkDir;

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub struct Site {
    pub config: SiteConfig,
    pub content_files: BTreeMap<PathBuf, String>,
    pub missing_ids: BTreeMap<PathBuf, String>,
    pub pages: BTreeMap<String, Page>,
    pub parsing_errors: BTreeMap<PathBuf, String>,
}

impl Site {
    pub fn new(config: SiteConfig) -> Site {
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
    pub fn parse_pages(&mut self) {
        self.content_files.iter().for_each(|f| {
            let error_file_path = replace_path(
                &f.0,
                &self.config.folders.content_root,
                &self.config.folders.error_root,
            )
            .unwrap()
            .with_extension("txt");

            match ast(f.1, &self.config.sections) {
                Ok(ast) => {
                    if let Some(id) = get_page_id(&ast) {
                        let page = Page {
                            ast,
                            id: id.clone(),
                            source_path: f.0.clone(),
                        };
                        let _ = self.pages.insert(id, page).is_none();
                    } else {
                        let _ = self
                            .missing_ids
                            .insert(
                                error_file_path,
                                format!("Missing ID: \n{}", f.1.to_string()),
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
        let dir = &self.config.folders.content_root;
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
}

fn get_page_id(ast: &Vec<Section>) -> Option<String> {
    None
}

fn replace_path(path: &PathBuf, find: &PathBuf, replace: &PathBuf) -> Result<PathBuf, String> {
    match path.strip_prefix(find) {
        Ok(path_part) => Ok(replace.clone().join(path_part)),
        Err(e) => Err(format!("Problem: {}", e)),
    }
}

use crate::ast::ast;
use crate::page::Page;
use crate::site_config::SiteConfig;
use serde::Serialize;
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
// use walkdir::DirEntry;
use walkdir::WalkDir;

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub struct Site {
    pub config: SiteConfig,
    pub content_files: BTreeMap<PathBuf, String>,
    pub pages: BTreeMap<String, Page>,
    pub parsing_errors: BTreeMap<PathBuf, String>,
}

impl Site {
    pub fn new(config: SiteConfig) -> Site {
        Site {
            config,
            content_files: BTreeMap::new(),
            pages: BTreeMap::new(),
            parsing_errors: BTreeMap::new(),
        }
    }
}

impl Site {
    pub fn parse_pages(&mut self) {
        self.content_files.iter().for_each(|f| {
            match ast(f.1, &self.config.sections) {
                Ok(data) => {
                    // dbg!(data);
                    ()
                }
                Err(e) => {
                    if let Ok(error_file_path) = replace_path(
                        &f.0,
                        &self.config.folders.content_root,
                        &self.config.folders.error_root,
                    ) {
                        self.parsing_errors.insert(error_file_path, e.to_string());
                    }
                }
            };
        })
    }

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
                            ()
                        }
                        Err(e) => {
                            println!("{}", e);
                            ()
                        }
                    }
                });
        } else {
            println!("Error, dir does not exist");
        }
    }
}

fn replace_path(path: &PathBuf, find: &PathBuf, replace: &PathBuf) -> Result<PathBuf, String> {
    match path.strip_prefix(find) {
        Ok(path_part) => Ok(replace.clone().join(path_part)),
        Err(e) => Err("Problem".to_string()), // todo make this a better error
    }
}

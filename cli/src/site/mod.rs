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
    config: SiteConfig,
    content_files: BTreeMap<PathBuf, String>,
    pages: BTreeMap<String, Page>,
}

impl Site {
    pub fn new(config: SiteConfig) -> Site {
        Site {
            config,
            content_files: BTreeMap::new(),
            pages: BTreeMap::new(),
        }
    }
}

impl Site {
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

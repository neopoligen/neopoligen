pub mod mocks;

use crate::sections::*;
use serde::Deserialize;
use serde::Serialize;
use serde_json;
use serde_json::Value;
use std::arch::aarch64;
use std::collections::BTreeMap;
use std::fs::{self, DirEntry};
use std::io;
use std::path::PathBuf;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub struct SiteConfig {
    #[serde(rename = "base_url")]
    pub base_url_raw: Option<String>,

    #[serde(default = "default_language")]
    pub default_language: Option<String>,

    #[serde(default = "empty_paths")]
    pub paths: BTreeMap<String, PathBuf>,

    pub project_root: Option<PathBuf>,

    #[serde(default = "empty_sections")]
    pub sections: Sections,

    #[serde(default = "empty_spans")]
    pub spans: Vec<String>,

    pub theme: String,
    pub theme_options: Option<Value>,
}

// #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
// #[serde(tag = "type", rename_all = "lowercase")]
// pub struct ThemeConfig {
//     pub name: String,
//     #[serde(default = "empty_options")]
//     pub options: Value,
// }

impl SiteConfig {
    pub fn base_url(&self) -> Option<String> {
        if let Some(base_url) = &self.base_url_raw {
            Some(base_url.strip_suffix("/").unwrap().to_string())
        } else {
            None
        }
    }
    pub fn cache_db_path(&self) -> PathBuf {
        self.cache_dir().join("cache.sqlite")
    }
    pub fn cache_dir(&self) -> PathBuf {
        self.project_root.clone().unwrap().join("cache")
    }
    pub fn content_dir(&self) -> PathBuf {
        self.project_root.clone().unwrap().join("content")
    }
    pub fn images_dir(&self) -> PathBuf {
        self.project_root.clone().unwrap().join("images")
    }
    pub fn og_images_dir(&self) -> PathBuf {
        self.output_dir().clone().join("og-images")
    }
    pub fn og_images_cache_dir(&self) -> PathBuf {
        self.cache_dir().clone().join("og-images")
    }
    pub fn output_dir(&self) -> PathBuf {
        self.project_root.clone().unwrap().join("docs")
    }
    pub fn page_cache_dir(&self) -> PathBuf {
        self.cache_dir().join("pages")
    }
    pub fn themes_dir(&self) -> PathBuf {
        self.project_root.clone().unwrap().join("themes")
    }
    pub fn theme_dir(&self) -> PathBuf {
        self.project_root
            .clone()
            .unwrap()
            .join(PathBuf::from(format!("themes/{}", self.theme)))
    }
    pub fn templates_dir(&self) -> PathBuf {
        self.theme_dir().join("templates")
    }

    pub fn load_sections(&mut self) {
        // // define the sections
        // self.sections.insert("basic".to_string(), vec![]);
        // self.sections.insert("checklist".to_string(), vec![]);
        // self.sections.insert("comment".to_string(), vec![]);
        // self.sections.insert("detail".to_string(), vec![]);
        // self.sections.insert("json".to_string(), vec![]);
        // self.sections.insert("list".to_string(), vec![]);
        // self.sections.insert("raw".to_string(), vec![]);
        // self.sections.insert("table".to_string(), vec![]);
        // self.sections.insert("yaml".to_string(), vec![]);

        let section_root = self
            .paths
            .get("theme_root")
            .unwrap()
            .clone()
            .join(PathBuf::from("templates/sections"));
        let section_dirs = get_dirs_in_dir(&section_root).unwrap();
        section_dirs.iter().for_each(|dir| {
            let cat_file_path = dir.join("category.txt");
            if cat_file_path.exists() {
                if let Ok(cat_raw) = fs::read_to_string(cat_file_path) {
                    if let Some(basename) = dir.file_name() {
                        if let Some(name) = cat_raw.lines().nth(0) {
                            if name == "basic" {
                                self.sections
                                    .basic
                                    .push(basename.to_string_lossy().to_string())
                            } else if name == "checklist" {
                                self.sections
                                    .checklist
                                    .push(basename.to_string_lossy().to_string())
                            } else if name == "comment" {
                                self.sections
                                    .comment
                                    .push(basename.to_string_lossy().to_string())
                            } else if name == "detail" {
                                self.sections
                                    .detail
                                    .push(basename.to_string_lossy().to_string())
                            } else if name == "json" {
                                self.sections
                                    .json
                                    .push(basename.to_string_lossy().to_string())
                            } else if name == "list" {
                                self.sections
                                    .list
                                    .push(basename.to_string_lossy().to_string())
                            } else if name == "raw" {
                                self.sections
                                    .raw
                                    .push(basename.to_string_lossy().to_string())
                            } else if name == "table" {
                                self.sections
                                    .table
                                    .push(basename.to_string_lossy().to_string())
                            } else if name == "yaml" {
                                self.sections
                                    .yaml
                                    .push(basename.to_string_lossy().to_string())
                            }
                        }
                    };
                }
            }
        });
    }
}

fn default_language() -> Option<String> {
    Some("en".to_string())
}

fn empty_paths() -> BTreeMap<String, PathBuf> {
    BTreeMap::new()
}

// fn empty_options() -> Value {
//     serde_json::from_str::<Value>("{}").unwrap()
// }

fn empty_sections() -> Sections {
    Sections {
        basic: vec![],
        block: vec![],
        checklist: vec![],
        comment: vec![],
        detail: vec![],
        json: vec![],
        list: vec![],
        raw: vec![],
        table: vec![],
        yaml: vec![],
    }
}

fn empty_spans() -> Vec<String> {
    // TODO: Load these dynamically by looking at the theme
    vec![
        "em".to_string(),
        "strong".to_string(),
        "span".to_string(),
        "link".to_string(),
    ]
}

fn get_dirs_in_dir(dir: &PathBuf) -> io::Result<Vec<PathBuf>> {
    Result::from_iter(
        fs::read_dir(dir)?
            .map(|entry| {
                let entry = entry?;
                Ok(entry)
            })
            .filter_map(|entry: Result<DirEntry, io::Error>| {
                let path = entry.unwrap().path();
                if path.is_dir() {
                    match path.file_name() {
                        Some(file_name) => {
                            if file_name.to_string_lossy().starts_with(".") {
                                None
                            } else {
                                Some(Ok(path))
                            }
                        }
                        None => None,
                    }
                } else {
                    None
                }
            }),
    )
}

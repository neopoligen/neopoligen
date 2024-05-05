pub mod mocks;

use serde::Deserialize;
use serde::Serialize;
use serde_json;
use serde_json::Value;
use std::collections::BTreeMap;
use std::fs::{self, DirEntry};
use std::io;
use std::path::PathBuf;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub struct SiteConfig {
    #[serde(default = "default_language")]
    pub default_language: String,

    #[serde(default = "empty_paths")]
    pub paths: BTreeMap<String, PathBuf>,

    #[serde(default = "empty_sections")]
    pub sections: BTreeMap<String, Vec<String>>,

    pub theme: ThemeConfig,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub struct ThemeConfig {
    pub name: String,
    #[serde(default = "empty_options")]
    pub options: Value,
}

impl SiteConfig {
    pub fn load_sections(&mut self) {
        // define the sections
        self.sections.insert("basic".to_string(), vec![]);
        self.sections.insert("comment".to_string(), vec![]);
        self.sections.insert("json".to_string(), vec![]);
        self.sections.insert("list".to_string(), vec![]);
        self.sections.insert("checklist".to_string(), vec![]);
        self.sections.insert("raw".to_string(), vec![]);
        self.sections.insert("table".to_string(), vec![]);
        self.sections.insert("detail".to_string(), vec![]);

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
                    if let Some(name) = cat_raw.lines().nth(0) {
                        if let Some(vec_to_add_to) = self.sections.get_mut(name) {
                            if let Some(basename) = dir.file_name() {
                                vec_to_add_to.push(basename.to_string_lossy().to_string());
                            }
                        }
                    };
                }
            }
        });
    }
}

fn default_language() -> String {
    "en".to_string()
}

fn empty_paths() -> BTreeMap<String, PathBuf> {
    BTreeMap::new()
}

fn empty_options() -> Value {
    serde_json::from_str::<Value>("{}").unwrap()
}

fn empty_sections() -> BTreeMap<String, Vec<String>> {
    BTreeMap::new()
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

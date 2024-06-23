pub mod mocks;

use crate::engine_config::EngineConfig;
use crate::neo_error::NeoError;
use crate::neo_error::NeoErrorKind;
use anyhow::Result;
use itertools::Itertools;
use regex::Regex;
use serde::Deserialize;
use serde::Serialize;
use std::fs::{self, DirEntry};
use std::io;
use std::path::PathBuf;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SiteConfig {
    #[serde(rename = "base_url")]
    pub base_url_raw: String,
    pub default_language: String,
    pub theme_name: String,
    pub theme_options: Option<serde_json::Value>,
    #[serde(default = "empty_sections")]
    pub sections: ConfigSections,
    // Reminder: This isn't expected to come from the
    // JSON file. The process sets it internally
    pub project_root: Option<PathBuf>,
    ///
    /// section_attrs
    ///
    /// The key/value and flag attrs to include
    /// in the defatul ``attr_string`` for sections
    ///
    /// TODO: Load this from admin/attrs/section.txt
    #[serde(default = "empty_vec")]
    pub section_attrs: Vec<String>,
    /// span_attrs
    /// The key value and flag attrs to include
    /// in the default ``attr_string`` for spans
    ///
    /// TODO: Load this from admin/attrs/span.txt
    #[serde(default = "empty_vec")]
    pub span_attrs: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ConfigSections {
    pub basic: Vec<String>,
    pub block: Vec<String>,
    pub checklist: Vec<String>,
    pub checklist_item: Vec<String>,
    pub comment: Vec<String>,
    //     pub detail: Vec<String>,
    pub json: Vec<String>,
    pub list: Vec<String>,
    pub list_item: Vec<String>,
    pub raw: Vec<String>,
    // pub table: Vec<String>,
    pub yaml: Vec<String>,
}

impl SiteConfig {
    pub fn new_from_engine_config(engine_config: &EngineConfig) -> Result<SiteConfig, NeoError> {
        let project_root = engine_config
            .sites_dir
            .join(engine_config.active_site.as_str());
        let config_path = project_root.join("admin").join("config.json");
        match fs::read_to_string(&config_path) {
            Ok(text) => match serde_json::from_str::<SiteConfig>(&text) {
                Ok(mut config) => {
                    config.project_root = Some(project_root.clone());
                    if !config.theme_dir().is_dir() {
                        Err(NeoError {
                            kind: NeoErrorKind::MissingThemeDirectory {
                                path: config.theme_dir().clone(),
                            },
                        })
                    } else {
                        config.load_sections();
                        config.load_section_attrs();
                        config.load_span_attrs();
                        Ok(config)
                    }
                }
                Err(_e) => Err(NeoError {
                    kind: NeoErrorKind::GenericErrorWithoutSourcePath {
                        msg: format!("Could not load config: {}", config_path.display()),
                    },
                }),
            },
            Err(_) => Err(NeoError {
                kind: NeoErrorKind::GenericErrorWithoutSourcePath {
                    msg: format!("Could not load config: {}", config_path.display()),
                },
            }),
        }
    }
}

// #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
// pub struct ImageConfig {
//     pub template: String,
//     pub responsive_widths: Vec<usize>,
// }

impl SiteConfig {
    pub fn admin_dir(&self) -> PathBuf {
        self.project_dir().join("admin")
    }

    pub fn base_url(&self) -> String {
        self.base_url_raw.trim_end_matches("/").to_string()
    }

    pub fn cache_db_path(&self) -> PathBuf {
        self.cache_dir().join("cache.sqlite")
    }

    pub fn cache_dir(&self) -> PathBuf {
        self.admin_dir().join("cache")
    }

    pub fn content_source_dir(&self) -> PathBuf {
        self.project_dir().join("content")
    }

    pub fn default_language(&self) -> Result<String> {
        Ok(self.default_language.clone())
    }

    // pub fn feeds_dest_dir(&self) -> PathBuf {
    //     self.output_dir().join("feeds")
    // }

    // pub fn feeds_source_dir(&self) -> PathBuf {
    //     self.templates_dir().join("feeds")
    // }

    pub fn image_cache_dir(&self) -> PathBuf {
        self.cache_dir().join("neo-images")
    }

    pub fn image_dest_dir(&self) -> PathBuf {
        self.output_dest_dir().join("neo-images")
    }

    pub fn image_source_dir(&self) -> PathBuf {
        self.project_dir().join("images")
    }

    // pub fn image_widths(&self) -> Vec<u32> {
    //     let mut tmp = BTreeSet::new();
    //     for w in self.base_image_widths.iter() {
    //         tmp.insert(*w);
    //     }
    //     for image in self.theme.images.iter() {
    //         tmp.insert(image.max_width);
    //     }
    //     itertools::sorted(tmp).collect()
    // }

    // pub fn mp3_dest_dir(&self) -> PathBuf {
    //     self.output_dir().join("mp3s")
    // }
    // pub fn mp3_source_dir(&self) -> PathBuf {
    //     self.project_dir().join("mp3s")
    // }

    // pub fn og_images_custom_source_dir(&self) -> PathBuf {
    //     self.project_dir().join("og-images")
    // }

    // pub fn og_images_dest_dir(&self) -> PathBuf {
    //     self.output_dir().clone().join("og-images")
    // }

    // pub fn og_images_cache_dir(&self) -> PathBuf {
    //     self.cache_dir().clone().join("og-images")
    // }

    pub fn output_dest_dir(&self) -> PathBuf {
        self.project_dir().join("docs")
    }

    // pub fn page_cache_dir(&self) -> PathBuf {
    //     self.cache_dir().join("pages")
    // }

    pub fn status_dest_dir(&self) -> PathBuf {
        self.admin_dir().join("status")
    }

    // pub fn tmp_dir(&self) -> PathBuf {
    //     self.cache_dir().join("tmp")
    // }

    pub fn themes_dir(&self) -> PathBuf {
        self.project_dir().join("themes")
    }

    pub fn theme_dir(&self) -> PathBuf {
        self.themes_dir().join(self.theme_name.clone())
    }

    // pub fn templates_dir(&self) -> PathBuf {
    //     self.theme_dir().join("templates")
    // }

    // pub fn theme_tests_dest_dir(&self) -> PathBuf {
    //     self.status_dir().join("theme-tests")
    // }

    // pub fn theme_tests_source_dir(&self) -> PathBuf {
    //     self.theme_dir().join("tests")
    // }

    pub fn project_dir(&self) -> PathBuf {
        self.project_root.clone().unwrap()
    }

    pub fn load_section_attrs(&mut self) {
        let section_attrs_path = self.admin_dir().join("attrs").join("sections.txt");
        match fs::read_to_string(section_attrs_path) {
            Ok(data) => {
                let re = Regex::new(r"^\s*$").unwrap();
                self.section_attrs = data
                    .lines()
                    .filter_map(|line| {
                        if !re.is_match(line) {
                            Some(line.to_string())
                        } else {
                            None
                        }
                    })
                    .unique()
                    .collect::<Vec<String>>();
            }
            Err(_e) => {
                dbg!("TODO: Add error message on missing seciton attrs");
                ()
            }
        }
    }

    pub fn load_span_attrs(&mut self) {
        let span_attrs_path = self.admin_dir().join("attrs").join("spans.txt");
        match fs::read_to_string(span_attrs_path) {
            Ok(data) => {
                let re = Regex::new(r"^\s*$").unwrap();
                self.span_attrs = data
                    .lines()
                    .filter_map(|line| {
                        if !re.is_match(line) {
                            Some(line.to_string())
                        } else {
                            None
                        }
                    })
                    .unique()
                    .collect::<Vec<String>>();
            }
            Err(_e) => {
                dbg!("TODO: Add error message on missing seciton attrs");
                ()
            }
        }
    }

    pub fn load_sections(&mut self) {
        let section_root = self.theme_dir().join("sections");
        //let section_root = self.templates_dir().join("sections");
        let section_categories = [
            "basic",
            "block",
            "checklist",
            "checklist-item",
            "comment",
            "json",
            "list",
            "list-item",
            "raw",
            "yaml",
        ];
        section_categories.iter().for_each(|category| {
            let category_dir = section_root.join(category);
            let section_dirs = get_dirs_in_dir(&category_dir).unwrap();
            section_dirs.iter().for_each(|section_dir| {
                let section_name = section_dir
                    .file_stem()
                    .unwrap()
                    .to_string_lossy()
                    .to_string();
                if section_name.as_str().ne("generic") && !section_name.starts_with(".") {
                    if category.eq_ignore_ascii_case("basic") {
                        self.sections.basic.push(section_name);
                    } else if category.eq_ignore_ascii_case("block") {
                        self.sections.block.push(section_name);
                    } else if category.eq_ignore_ascii_case("checklist") {
                        self.sections.checklist.push(section_name);
                    // } else if category.eq_ignore_ascii_case("checklist-item") {
                    //     self.sections.checklistitem.push(section_name);
                    } else if category.eq_ignore_ascii_case("json") {
                        self.sections.json.push(section_name);
                    } else if category.eq_ignore_ascii_case("list") {
                        self.sections.list.push(section_name);
                    // } else if category.eq_ignore_ascii_case("list-item") {
                    //     self.sections.listitem.push(section_name);
                    } else if category.eq_ignore_ascii_case("raw") {
                        self.sections.raw.push(section_name);
                    } else if category.eq_ignore_ascii_case("yaml") {
                        self.sections.yaml.push(section_name);
                    }
                }
            });
            //  let section_category = dir.file_stem().unwrap().to_string_lossy().to_string();

            //     let cat_file_path = dir.join("category.txt");
            //     if cat_file_path.exists() {
            //         if let Ok(cat_raw) = fs::read_to_string(cat_file_path) {
            //             if let Some(basename) = dir.file_name() {
            //                 if let Some(name) = cat_raw.lines().nth(0) {
            //                     if name == "basic" {
            //                         self.sections
            //                             .basic
            //                             .push(basename.to_string_lossy().to_string())
            //                     } else if name == "checklist" {
            //                         self.sections
            //                             .checklist
            //                             .push(basename.to_string_lossy().to_string())
            //                     } else if name == "comment" {
            //                         self.sections
            //                             .comment
            //                             .push(basename.to_string_lossy().to_string())
            //                     } else if name == "detail" {
            //                         self.sections
            //                             .detail
            //                             .push(basename.to_string_lossy().to_string())
            //                     } else if name == "json" {
            //                         self.sections
            //                             .json
            //                             .push(basename.to_string_lossy().to_string())
            //                     } else if name == "list" {
            //                         self.sections
            //                             .list
            //                             .push(basename.to_string_lossy().to_string())
            //                     } else if name == "raw" {
            //                         self.sections
            //                             .raw
            //                             .push(basename.to_string_lossy().to_string())
            //                     } else if name == "table" {
            //                         self.sections
            //                             .table
            //                             .push(basename.to_string_lossy().to_string())
            //                     } else if name == "yaml" {
            //                         self.sections
            //                             .yaml
            //                             .push(basename.to_string_lossy().to_string())
            //                     }
            //                 }
            //             };
            //         }
            //     }
        });
    }
}

fn empty_sections() -> ConfigSections {
    ConfigSections {
        basic: vec![],
        block: vec![],
        checklist: vec![],
        checklist_item: vec![],
        comment: vec![],
        json: vec![],
        list: vec![],
        list_item: vec![],
        raw: vec![],
        yaml: vec![],
    }
}

fn empty_vec() -> Vec<String> {
    vec![]
}

// fn empty_spans() -> Vec<String> {
//     vec![]
// }

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

// fn hard_code_image_widths() -> Vec<u32> {
//     vec![100, 300, 500, 750, 1000, 1500]
// }

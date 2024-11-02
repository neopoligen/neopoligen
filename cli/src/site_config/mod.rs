pub mod mocks;

use crate::config_lists::ConfigLists;
use crate::engine_config::EngineConfig;
use crate::helpers::get_dirs_in_dir;
use crate::neo_error::NeoError;
use crate::neo_error::NeoErrorKind;
use anyhow::Result;
use serde::Deserialize;
use serde::Serialize;
use std::fs::{self};
use std::path::PathBuf;
use tracing::{event, instrument, Level};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SiteConfigV42 {
    pub attr_flag_lists: Option<ConfigLists>,
    pub base_url: String,
    pub default_language: String,
    pub theme: String,
    // REMINDER: project_root is set internally
    pub project_root: Option<PathBuf>,
    #[serde(default = "output_dir_name")]
    pub output_directory_name: String,
    #[serde(default = "empty_sections_v41")]
    pub sections: ConfigSections,
    #[serde(default = "debug_parser_default")]
    pub debug_parser: bool,
}

fn output_dir_name() -> String {
    "site".to_string()
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SectionSettings {
    pub attrs: Vec<String>,
    pub flags: Vec<String>,
}

impl SiteConfigV42 {
    pub fn new_from_engine_config(engine_config: &EngineConfig) -> Result<SiteConfigV42, NeoError> {
        let project_root = engine_config
            .sites_dir
            .join(engine_config.active_site.as_str());
        let config_path = project_root.join("admin").join("settings.json5");
        match fs::read_to_string(&config_path) {
            Ok(text) => match serde_json5::from_str::<SiteConfigV42>(&text) {
                Ok(mut config) => {
                    config.project_root = Some(project_root.clone());
                    if !config.theme_dir().is_dir() {
                        Err(NeoError {
                            kind: NeoErrorKind::MissingThemeDirectory {
                                path: config.theme_dir().clone(),
                            },
                        })
                    } else {
                        // TODO: Handle errors here
                        config.load_sections().unwrap();
                        //config.load_section_attrs();
                        //config.load_span_attrs();
                        Ok(config)
                    }
                }
                Err(e) => Err(NeoError {
                    kind: NeoErrorKind::GenericErrorWithoutSourcePath {
                        msg: format!("Could not load config: {} - {}", config_path.display(), e),
                    },
                }),
            },
            Err(e) => Err(NeoError {
                kind: NeoErrorKind::GenericErrorWithoutSourcePath {
                    msg: format!("Could not load config: {} - {}", config_path.display(), e),
                },
            }),
        }
    }
}

impl SiteConfigV42 {
    #[instrument(skip(self))]
    pub fn load_sections(&mut self) -> Result<(), NeoError> {
        event!(Level::INFO, "Loading Theme Sections");
        let section_root = self.theme_dir().join("sections");
        let section_categories: Vec<&str> = vec![
            "basic",
            "block",
            "checklist",
            "checklist_item",
            "json5",
            "list",
            "list_item",
            "raw",
        ];
        section_categories.iter().for_each(|category| {
            let category_dir = section_root.join(category);
            if let Ok(section_dirs) = get_dirs_in_dir(&category_dir) {
                section_dirs.iter().for_each(|section_dir| {
                    let json5_path = section_dir.join("_settings.json5");
                    let _ = match fs::read_to_string(&json5_path) {
                        Ok(text) => match serde_json5::from_str::<SectionSettings>(&text) {
                            Ok(_j) => {
                                let section_name = section_dir
                                    .file_stem()
                                    .unwrap()
                                    .to_string_lossy()
                                    .to_string();
                                if !section_name.starts_with(".") {
                                    // check to see if there's a settings.json5 file
                                    if category.eq_ignore_ascii_case("basic") {
                                        self.sections.basic.push(section_name.clone());
                                    } else if category.eq_ignore_ascii_case("block") {
                                        self.sections.block.push(section_name.clone());
                                    } else if category.eq_ignore_ascii_case("checklist") {
                                        self.sections.checklist.push(section_name.clone());
                                    } else if category.eq_ignore_ascii_case("checklist-item") {
                                        self.sections.checklist_item.push(section_name.clone());
                                    } else if category.eq_ignore_ascii_case("json5") {
                                        self.sections.json5.push(section_name.clone());
                                    } else if category.eq_ignore_ascii_case("list") {
                                        self.sections.list.push(section_name.clone());
                                    } else if category.eq_ignore_ascii_case("list-item") {
                                        self.sections.list_item.push(section_name.clone());
                                    } else if category.eq_ignore_ascii_case("raw") {
                                        self.sections.raw.push(section_name.clone());
                                    }
                                }
                                Ok(())
                            }
                            Err(e) => {
                                event!(
                                    Level::ERROR,
                                    "Could not load _settings.json5 file: {} - {}",
                                    e,
                                    json5_path.display()
                                );
                                Err(NeoError {
                                    kind: NeoErrorKind::GenericErrorWithoutSourcePath {
                                        msg: format!(
                                            "Could not load _settings.json5 file: {} - {}",
                                            e,
                                            json5_path.display(),
                                        ),
                                    },
                                })
                            }
                        },
                        Err(e) => {
                            event!(
                                Level::ERROR,
                                "Could not load _settings.json5 file: {} - {}",
                                e,
                                json5_path.display()
                            );
                            Err(NeoError {
                                kind: NeoErrorKind::GenericErrorWithoutSourcePath {
                                    msg: format!(
                                        "Could not load _settings.json5 file: {} - {}",
                                        e,
                                        json5_path.display(),
                                    ),
                                },
                            })
                        }
                    };
                })
            }
        });
        Ok(())
    }
}

impl SiteConfigV42 {
    pub fn admin_dir(&self) -> PathBuf {
        self.project_dir().join("admin")
    }

    pub fn built_in_dir(&self) -> PathBuf {
        self.project_dir().join(".built-in")
    }

    pub fn content_source_dir(&self) -> PathBuf {
        self.project_dir().join("content")
    }

    pub fn default_language(&self) -> Result<String> {
        Ok(self.default_language.clone())
    }

    pub fn errors_dir(&self) -> PathBuf {
        self.built_in_dir().join("errors")
    }

    pub fn errors_index_page(&self) -> PathBuf {
        self.errors_dir().join("index.html")
    }

    pub fn output_dest_dir(&self) -> PathBuf {
        self.project_dir().join(self.output_directory_name.clone())
    }

    pub fn project_dir(&self) -> PathBuf {
        self.project_root.clone().unwrap()
    }

    pub fn source_file_error_dir(&self) -> PathBuf {
        self.errors_dir().join("source_files")
    }

    pub fn status_dest_dir(&self) -> PathBuf {
        self.admin_dir().join("status")
    }

    pub fn theme_dir(&self) -> PathBuf {
        self.themes_dir().join(self.theme.clone())
    }

    pub fn themes_dir(&self) -> PathBuf {
        self.project_dir().join("themes")
    }
}

fn empty_sections_v41() -> ConfigSections {
    ConfigSections {
        basic: vec![],
        block: vec![],
        checklist: vec![],
        checklist_item: vec![],
        json5: vec![],
        list: vec![],
        list_item: vec![],
        raw: vec![],
    }
}

// fn empty_vec_v41() -> Vec<String> {
//     vec![]
// }

// //////////////////////////////////////////////////////////////////////////

//// TODO: Deprecate this when SiteConfigV42 is in place
//#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
//pub struct SiteConfig {
//    #[serde(rename = "base_url")]
//    pub base_url_raw: String,
//    pub default_language: String,
//    #[serde(default = "output_dir_name")]
//    pub output_directory: String,
//    pub theme_name: String,
//    // pub theme_options: Option<serde_json5::Value>,
//    #[serde(default = "empty_sections")]
//    pub sections: ConfigSections,
//    // Reminder: This isn't expected to come from the
//    // JSON file. The process sets it internally
//    pub project_root: Option<PathBuf>,
//    ///
//    /// section_attrs
//    ///
//    /// The key/value and flag attrs to include
//    /// in the defatul ``attr_string`` for sections
//    ///
//    /// TODO: Load this from admin/attrs/section.txt
//    #[serde(default = "empty_vec")]
//    pub section_attrs: Vec<String>,
//    /// span_attrs
//    /// The key value and flag attrs to include
//    /// in the default ``attr_string`` for spans
//    ///
//    /// TODO: Load this from admin/attrs/span.txt
//    #[serde(default = "empty_vec")]
//    pub span_attrs: Vec<String>,
//}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ConfigSections {
    pub basic: Vec<String>,
    pub block: Vec<String>,
    pub checklist: Vec<String>,
    pub checklist_item: Vec<String>,
    pub json5: Vec<String>,
    pub list: Vec<String>,
    pub list_item: Vec<String>,
    pub raw: Vec<String>,
}

// impl SiteConfig {
//     pub fn new_from_engine_config(engine_config: &EngineConfig) -> Result<SiteConfig, NeoError> {
//         let project_root = engine_config
//             .sites_dir
//             .join(engine_config.active_site.as_str());
//         let config_path = project_root.join("admin").join("settings.json5");
//         match fs::read_to_string(&config_path) {
//             Ok(text) => match serde_json5::from_str::<SiteConfig>(&text) {
//                 Ok(mut config) => {
//                     config.project_root = Some(project_root.clone());
//                     if !config.theme_dir().is_dir() {
//                         Err(NeoError {
//                             kind: NeoErrorKind::MissingThemeDirectory {
//                                 path: config.theme_dir().clone(),
//                             },
//                         })
//                     } else {
//                         config.load_sections();
//                         config.load_section_attrs();
//                         config.load_span_attrs();
//                         Ok(config)
//                     }
//                 }
//                 Err(_e) => Err(NeoError {
//                     kind: NeoErrorKind::GenericErrorWithoutSourcePath {
//                         msg: format!("Could not load config: {}", config_path.display()),
//                     },
//                 }),
//             },
//             Err(_) => Err(NeoError {
//                 kind: NeoErrorKind::GenericErrorWithoutSourcePath {
//                     msg: format!("Could not load config: {}", config_path.display()),
//                 },
//             }),
//         }
//     }
// }

// #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
// pub struct ImageConfig {
//     pub template: String,
//     pub responsive_widths: Vec<usize>,
// }

//impl SiteConfig {

// pub fn admin_dir(&self) -> PathBuf {
//     self.project_dir().join("admin")
// }
// pub fn base_url(&self) -> String {
//     self.base_url_raw.trim_end_matches("/").to_string()
// }
// pub fn cache_db_path(&self) -> PathBuf {
//     self.cache_dir().join("cache.sqlite")
// }
// pub fn cache_dir(&self) -> PathBuf {
//     self.admin_dir().join("cache")
// }
// pub fn content_source_dir(&self) -> PathBuf {
//     self.project_dir().join("content")
// }
// pub fn default_language(&self) -> Result<String> {
//     Ok(self.default_language.clone())
// }

// pub fn feeds_dest_dir(&self) -> PathBuf {
//     self.output_dir().join("feeds")
// }
// pub fn feeds_source_dir(&self) -> PathBuf {
//     self.templates_dir().join("feeds")
// }

// pub fn image_cache_dir(&self) -> PathBuf {
//     self.cache_dir().join("neo-images")
// }
// pub fn image_dest_dir(&self) -> PathBuf {
//     self.output_dest_dir().join("neo-images")
// }
// pub fn image_source_dir(&self) -> PathBuf {
//     self.project_dir().join("images")
// }

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

// pub fn output_dest_dir(&self) -> PathBuf {
//     self.project_dir().join("docs")
// }

// pub fn page_cache_dir(&self) -> PathBuf {
//     self.cache_dir().join("pages")
// }

// pub fn status_dest_dir(&self) -> PathBuf {
//     self.admin_dir().join("status")
// }

// pub fn tmp_dir(&self) -> PathBuf {
//     self.cache_dir().join("tmp")
// }

// pub fn themes_dir(&self) -> PathBuf {
//     self.project_dir().join("themes")
// }
// pub fn theme_dir(&self) -> PathBuf {
//     self.themes_dir().join(self.theme_name.clone())
// }

// pub fn theme_image_widths(&self) -> Vec<u32> {
//     match &self.theme_options {
//         Some(theme_options) => {
//             if let Some(widths) = theme_options.get("image_widths") {
//                 // serde_json5::from_value(widths.clone()).unwrap()
//                 vec![]
//             } else {
//                 vec![]
//             }
//         }
//         None => vec![],
//     }
// }
// pub fn templates_dir(&self) -> PathBuf {
//     self.theme_dir().join("templates")
// }
// pub fn theme_tests_dest_dir(&self) -> PathBuf {
//     self.status_dir().join("theme-tests")
// }
// pub fn theme_tests_source_dir(&self) -> PathBuf {
//     self.theme_dir().join("tests")
// }

// pub fn project_dir(&self) -> PathBuf {
//     self.project_root.clone().unwrap()
// }

// pub fn load_section_attrs(&mut self) {
//     let section_attrs_path = self.admin_dir().join("attrs").join("sections.txt");
//     match fs::read_to_string(section_attrs_path) {
//         Ok(data) => {
//             let re = Regex::new(r"^\s*$").unwrap();
//             self.section_attrs = data
//                 .lines()
//                 .filter_map(|line| {
//                     if !re.is_match(line) {
//                         Some(line.to_string())
//                     } else {
//                         None
//                     }
//                 })
//                 .unique()
//                 .collect::<Vec<String>>();
//         }
//         Err(_e) => {
//             dbg!("TODO: Add error message on missing seciton attrs");
//             ()
//         }
//     }
// }

// pub fn load_span_attrs(&mut self) {
//     let span_attrs_path = self.admin_dir().join("attrs").join("spans.txt");
//     match fs::read_to_string(span_attrs_path) {
//         Ok(data) => {
//             let re = Regex::new(r"^\s*$").unwrap();
//             self.span_attrs = data
//                 .lines()
//                 .filter_map(|line| {
//                     if !re.is_match(line) {
//                         Some(line.to_string())
//                     } else {
//                         None
//                     }
//                 })
//                 .unique()
//                 .collect::<Vec<String>>();
//         }
//         Err(_e) => {
//             dbg!("TODO: Add error message on missing seciton attrs");
//             ()
//         }
//     }
// }

//pub fn load_sections(&mut self) {
//    let section_root = self.theme_dir().join("sections");
//    //let section_root = self.templates_dir().join("sections");
//    let section_categories = [
//        "basic",
//        "block",
//        "checklist",
//        "checklist-item",
//        "json",
//        "list",
//        "list-item",
//        "raw",
//        "yaml",
//    ];
//    section_categories.iter().for_each(|category| {
//        let category_dir = section_root.join(category);
//        let section_dirs = get_dirs_in_dir(&category_dir).unwrap();
//        section_dirs.iter().for_each(|section_dir| {
//            let section_name = section_dir
//                .file_stem()
//                .unwrap()
//                .to_string_lossy()
//                .to_string();
//            if section_name.as_str().ne("generic") && !section_name.starts_with(".") {
//                if category.eq_ignore_ascii_case("basic") {
//                    self.sections.basic.push(section_name);
//                } else if category.eq_ignore_ascii_case("block") {
//                    self.sections.block.push(section_name);
//                } else if category.eq_ignore_ascii_case("checklist") {
//                    self.sections.checklist.push(section_name);
//                // } else if category.eq_ignore_ascii_case("checklist-item") {
//                //     self.sections.checklistitem.push(section_name);
//                } else if category.eq_ignore_ascii_case("json") {
//                    self.sections.json5.push(section_name);
//                } else if category.eq_ignore_ascii_case("list") {
//                    self.sections.list.push(section_name);
//                // } else if category.eq_ignore_ascii_case("list-item") {
//                //     self.sections.listitem.push(section_name);
//                } else if category.eq_ignore_ascii_case("raw") {
//                    self.sections.raw.push(section_name);
//                }
//            }
//        });
//        //  let section_category = dir.file_stem().unwrap().to_string_lossy().to_string();
//        //     let cat_file_path = dir.join("category.txt");
//        //     if cat_file_path.exists() {
//        //         if let Ok(cat_raw) = fs::read_to_string(cat_file_path) {
//        //             if let Some(basename) = dir.file_name() {
//        //                 if let Some(name) = cat_raw.lines().nth(0) {
//        //                     if name == "basic" {
//        //                         self.sections
//        //                             .basic
//        //                             .push(basename.to_string_lossy().to_string())
//        //                     } else if name == "checklist" {
//        //                         self.sections
//        //                             .checklist
//        //                             .push(basename.to_string_lossy().to_string())
//        //                     } else if name == "comment" {
//        //                         self.sections
//        //                             .comment
//        //                             .push(basename.to_string_lossy().to_string())
//        //                     } else if name == "detail" {
//        //                         self.sections
//        //                             .detail
//        //                             .push(basename.to_string_lossy().to_string())
//        //                     } else if name == "json" {
//        //                         self.sections
//        //                             .json
//        //                             .push(basename.to_string_lossy().to_string())
//        //                     } else if name == "list" {
//        //                         self.sections
//        //                             .list
//        //                             .push(basename.to_string_lossy().to_string())
//        //                     } else if name == "raw" {
//        //                         self.sections
//        //                             .raw
//        //                             .push(basename.to_string_lossy().to_string())
//        //                     } else if name == "table" {
//        //                         self.sections
//        //                             .table
//        //                             .push(basename.to_string_lossy().to_string())
//        //                     } else if name == "yaml" {
//        //                         self.sections
//        //                             .yaml
//        //                             .push(basename.to_string_lossy().to_string())
//        //                     }
//        //                 }
//        //             };
//        //         }
//        //     }
//    });
//}

// }

fn debug_parser_default() -> bool {
    false
}

// // TODO: deprecate when v41 is done
// fn empty_sections() -> ConfigSections {
//     ConfigSections {
//         basic: vec![],
//         block: vec![],
//         checklist: vec![],
//         checklist_item: vec![],
//         json5: vec![],
//         list: vec![],
//         list_item: vec![],
//         raw: vec![],
//     }
// }

// fn empty_vec() -> Vec<String> {
//     vec![]
// }

// fn output_dir_name() -> String {
//     "_site".to_string()
// }

// fn get_dirs_in_dir(dir: &PathBuf) -> io::Result<Vec<PathBuf>> {
//     Result::from_iter(
//         fs::read_dir(dir)?
//             .map(|entry| {
//                 let entry = entry?;
//                 Ok(entry)
//             })
//             .filter_map(|entry: Result<DirEntry, io::Error>| {
//                 let path = entry.unwrap().path();
//                 if path.is_dir() {
//                     match path.file_name() {
//                         Some(file_name) => {
//                             if file_name.to_string_lossy().starts_with(".") {
//                                 None
//                             } else {
//                                 Some(Ok(path))
//                             }
//                         }
//                         None => None,
//                     }
//                 } else {
//                     None
//                 }
//             }),
//     )
// }

#[cfg(test)]
mod test {
    // use super::*;
    // use pretty_assertions::assert_eq;

    // #[test]
    // fn theme_image_widths_returns_empty_array_if_none_exist() {
    //     let config = SiteConfig::mock1_basic();
    //     let left: Vec<u32> = vec![];
    //     let right = config.theme_image_widths();
    //     assert_eq!(left, right);
    // }

    // #[test]
    // fn theme_image_widths_return_if_there_are_any() {
    //     let config = SiteConfig::mock2_with_image_widths();
    //     let left: Vec<u32> = vec![100, 400];
    //     let right = config.theme_image_widths();
    //     assert_eq!(left, right);
    // }

    //
}

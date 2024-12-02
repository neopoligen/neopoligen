pub mod mocks;

use crate::config_lists::ConfigLists;
use crate::engine_config::EngineConfig;
use crate::helpers::get_folders_in_folder;
use crate::neo_error::NeoError;
use crate::neo_error::NeoErrorKind;
use anyhow::Result;
use nom::bytes::complete::tag;
use nom::bytes::complete::tag_no_case;
use nom::bytes::complete::take_until;
use nom::character::complete::alphanumeric1;
use nom::character::complete::space0;
use nom::character::complete::space1;
use nom::combinator::not;
use nom::combinator::opt;
use nom::multi::many0;
use nom::sequence::preceded;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::ParserExt;
use serde::Deserialize;
use serde::Serialize;
use std::collections::BTreeMap;
use std::fs::{self};
use std::path::PathBuf;
use tracing::{event, instrument, Level};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SiteConfigV42 {
    // TODO: deprecate attr_flag_lists  and switch this
    // to section_attrs_to_hoist and make a BTreeMap.
    // Or, add it to the section itself.
    pub attr_flag_lists: Option<ConfigLists>,
    #[serde(default = "debug_parser_default")]
    pub debug_parser: bool,
    #[serde(default = "default_image_maxes")]
    pub image_maxes: Vec<usize>,
    pub language_default: String,
    pub theme: String,
    pub time_zone: Option<String>,
    pub project_root: Option<PathBuf>,
    pub site_root: String,
    #[serde(default = "site_output_folder_name")]
    pub site_output_folder_name: String,
    #[serde(default = "empty_section_types")]
    pub section_types: BTreeMap<String, Vec<String>>,
}

fn debug_parser_default() -> bool {
    false
}

fn default_image_maxes() -> Vec<usize> {
    vec![
        50, 100, 150, 200, 300, 400, 600, 800, 1000, 1200, 1400, 1800, 2000, 2400, 2800, 3200,
    ]
}

fn empty_section_types() -> BTreeMap<String, Vec<String>> {
    let mut types = BTreeMap::new();
    types.insert("basic".to_string(), vec![]);
    types.insert("checklist".to_string(), vec![]);
    types.insert("css".to_string(), vec![]);
    types.insert("csv".to_string(), vec![]);
    types.insert("html".to_string(), vec![]);
    types.insert("json5".to_string(), vec![]);
    types.insert("list".to_string(), vec![]);
    types.insert("none".to_string(), vec![]);
    types.insert("olist".to_string(), vec![]);
    types.insert("raw".to_string(), vec![]);
    types.insert("script".to_string(), vec![]);
    types.insert("toml".to_string(), vec![]);
    types
}

fn site_output_folder_name() -> String {
    "docs".to_string()
}

pub fn parse_type<'a>(source: &str) -> IResult<&str, &str, ErrorTree<&str>> {
    let (source, _) = many0(preceded(
        tuple((take_until("[#"), tag("[#"), opt(tag("-")), space0)),
        not(tag_no_case("TYPE")),
    ))
    .context("")
    .parse(source)?;
    let (source, kind) = preceded(
        tuple((
            take_until("[#"),
            tag("[#"),
            opt(tag("-")),
            space0,
            tag_no_case("TYPE:"),
            space1,
        )),
        alphanumeric1,
    )
    .context("")
    .parse(source)?;
    Ok((source, kind))
}

impl SiteConfigV42 {
    #[instrument(skip(self))]
    pub fn load_sections(&mut self) {
        match get_folders_in_folder(&self.theme_sections_input_folder()) {
            Ok(folders) => {
                folders.iter().for_each(|folder| {
                    let default_path = folder.join("default.neoj");
                    if default_path.is_file() {
                        if let Ok(source) = fs::read_to_string(&default_path) {
                            match parse_type(&source) {
                                Ok((_, t)) => {
                                    if let Some(name) = &default_path.parent().unwrap().file_name()
                                    {
                                        if let Some(types) = self.section_types.get_mut(t) {
                                            types.push(name.to_string_lossy().to_string());
                                            ()
                                        } else {
                                            // TODO: Log an error if the type isn't valid
                                        }
                                    }
                                    ()
                                }
                                Err(_) => {
                                    // TODO: Log an error if there's no type
                                    ()
                                }
                            }
                        }
                    }
                });
            }
            Err(_e) => {
                // TODO: Hoist Error
            }
        }
    }

    #[instrument(skip(engine_config))]
    pub fn new_from_engine_config(engine_config: &EngineConfig) -> Result<SiteConfigV42, NeoError> {
        event!(Level::INFO, "Loading Site Config");
        let project_root = engine_config
            .sites_folder
            .join(engine_config.active_site.as_str());
        let config_path = project_root.join("config.json5");
        match fs::read_to_string(&config_path) {
            Ok(text) => match serde_json5::from_str::<SiteConfigV42>(&text) {
                Ok(mut config) => {
                    config.project_root = Some(project_root.clone());
                    if !config.theme_input_root_folder().is_dir() {
                        Err(NeoError {
                            kind: NeoErrorKind::MissingThemeDirectory {
                                path: config.theme_input_root_folder().clone(),
                            },
                        })
                    } else {
                        // TODO: Handle errors here
                        config.load_sections();
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
    pub fn cache_root_folder(&self) -> PathBuf {
        self.project_folder().join("cache")
    }

    pub fn content_input_root_folder(&self) -> PathBuf {
        self.project_folder().join("content")
    }

    pub fn images_input_root_folder(&self) -> PathBuf {
        self.project_folder().join("images")
    }

    pub fn language_default(&self) -> Result<String> {
        Ok(self.language_default.clone())
    }

    pub fn errors_output_root_folder(&self) -> PathBuf {
        self.cache_root_folder().join("errors")
    }

    pub fn errors_output_index_page(&self) -> PathBuf {
        self.errors_output_root_folder().join("index.html")
    }

    pub fn macros_input_root_folder(&self) -> PathBuf {
        self.theme_input_root_folder().join("macros")
    }

    pub fn neo_files_output_root_folder(&self) -> PathBuf {
        self.site_output_root_folder()
            .join(&self.neo_files_output_folder_name())
    }

    pub fn neo_files_output_folder_name(&self) -> String {
        "neo-files".to_string()
    }

    pub fn site_output_root_folder(&self) -> PathBuf {
        self.project_folder()
            .join(self.site_output_folder_name.clone())
    }

    pub fn project_folder(&self) -> PathBuf {
        self.project_root.clone().unwrap()
    }

    pub fn source_file_error_folder(&self) -> PathBuf {
        self.errors_output_root_folder().join("source_files")
    }

    pub fn status_dest_folder(&self) -> PathBuf {
        self.cache_root_folder().join("status")
    }

    pub fn theme_input_root_folder(&self) -> PathBuf {
        self.themes_input_root_folder().join(self.theme.clone())
    }

    pub fn themes_input_root_folder(&self) -> PathBuf {
        self.project_folder().join("themes")
    }

    pub fn theme_files_input_folder(&self) -> PathBuf {
        self.theme_input_root_folder().join("files")
    }

    pub fn theme_files_output_folder(&self) -> PathBuf {
        self.neo_files_output_root_folder()
            .join(&self.theme_files_output_folder_name())
    }

    pub fn theme_files_output_folder_name(&self) -> String {
        "theme-files".to_string()
    }

    pub fn theme_files_folder_href(&self) -> PathBuf {
        PathBuf::from("/")
            .join(&self.neo_files_output_folder_name())
            .join(&self.theme_files_output_folder_name())
    }

    pub fn theme_sections_input_folder(&self) -> PathBuf {
        self.theme_input_root_folder().join("sections")
    }
}

// fn empty_sections_v41() -> ConfigSections {
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

// fn empty_vec_v41() -> Vec<String> {
//     vec![]
// }

// //////////////////////////////////////////////////////////////////////////

//// TODO: Deprecate this when SiteConfigV42 is in place
//#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
//pub struct SiteConfig {
//    #[serde(rename = "base_url")]
//    pub base_url_raw: String,
//    pub language_default: String,
//    #[serde(default = "output_folder_name")]
//    pub output_folder: String,
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

// impl SiteConfig {
//     pub fn new_from_engine_config(engine_config: &EngineConfig) -> Result<SiteConfig, NeoError> {
//         let project_root = engine_config
//             .sites_folder
//             .join(engine_config.active_site.as_str());
//         let config_path = project_root.join("admin").join("settings.json5");
//         match fs::read_to_string(&config_path) {
//             Ok(text) => match serde_json5::from_str::<SiteConfig>(&text) {
//                 Ok(mut config) => {
//                     config.project_root = Some(project_root.clone());
//                     if !config.theme_input_root_folder().is_folder() {
//                         Err(NeoError {
//                             kind: NeoErrorKind::MissingThemeDirectory {
//                                 path: config.theme_input_root_folder().clone(),
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

// pub fn admin_folder(&self) -> PathBuf {
//     self.project_folder().join("admin")
// }
// pub fn base_url(&self) -> String {
//     self.base_url_raw.trim_end_matches("/").to_string()
// }
// pub fn cache_db_path(&self) -> PathBuf {
//     self.cache_folder().join("cache.sqlite")
// }
// pub fn cache_folder(&self) -> PathBuf {
//     self.admin_folder().join("cache")
// }
// pub fn content_input_root_folder(&self) -> PathBuf {
//     self.project_folder().join("content")
// }
// pub fn language_default(&self) -> Result<String> {
//     Ok(self.language_default.clone())
// }

// pub fn feeds_dest_folder(&self) -> PathBuf {
//     self.output_folder().join("feeds")
// }
// pub fn feeds_source_folder(&self) -> PathBuf {
//     self.templates_folder().join("feeds")
// }

// pub fn image_cache_folder(&self) -> PathBuf {
//     self.cache_folder().join("neo-images")
// }
// pub fn image_dest_folder(&self) -> PathBuf {
//     self.site_output_root_folder().join("neo-images")
// }
// pub fn image_source_folder(&self) -> PathBuf {
//     self.project_folder().join("images")
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
// pub fn mp3_dest_folder(&self) -> PathBuf {
//     self.output_folder().join("mp3s")
// }
// pub fn mp3_source_folder(&self) -> PathBuf {
//     self.project_folder().join("mp3s")
// }
// pub fn og_images_custom_source_folder(&self) -> PathBuf {
//     self.project_folder().join("og-images")
// }
// pub fn og_images_dest_folder(&self) -> PathBuf {
//     self.output_folder().clone().join("og-images")
// }
// pub fn og_images_cache_folder(&self) -> PathBuf {
//     self.cache_folder().clone().join("og-images")
// }

// pub fn site_output_root_folder(&self) -> PathBuf {
//     self.project_folder().join("docs")
// }

// pub fn page_cache_folder(&self) -> PathBuf {
//     self.cache_folder().join("pages")
// }

// pub fn status_dest_folder(&self) -> PathBuf {
//     self.admin_folder().join("status")
// }

// pub fn tmp_folder(&self) -> PathBuf {
//     self.cache_folder().join("tmp")
// }

// pub fn themes_folder(&self) -> PathBuf {
//     self.project_folder().join("themes")
// }
// pub fn theme_input_root_folder(&self) -> PathBuf {
//     self.themes_folder().join(self.theme_name.clone())
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
// pub fn templates_folder(&self) -> PathBuf {
//     self.theme_input_root_folder().join("templates")
// }
// pub fn theme_tests_dest_folder(&self) -> PathBuf {
//     self.status_folder().join("theme-tests")
// }
// pub fn theme_tests_source_folder(&self) -> PathBuf {
//     self.theme_input_root_folder().join("tests")
// }

// pub fn project_folder(&self) -> PathBuf {
//     self.project_root.clone().unwrap()
// }

// pub fn load_section_attrs(&mut self) {
//     let section_attrs_path = self.admin_folder().join("attrs").join("sections.txt");
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
//     let span_attrs_path = self.admin_folder().join("attrs").join("spans.txt");
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
//    let section_root = self.theme_input_root_folder().join("sections");
//    //let section_root = self.templates_folder().join("sections");
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
//        let category_folder = section_root.join(category);
//        let section_folders = get_folders_in_folder(&category_folder).unwrap();
//        section_folders.iter().for_each(|section_folder| {
//            let section_name = section_folder
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

// fn output_folder_name() -> String {
//     "_site".to_string()
// }

// fn get_folders_in_folder(dir: &PathBuf) -> io::Result<Vec<PathBuf>> {
//     Result::from_iter(
//         fs::read_folder(dir)?
//             .map(|entry| {
//                 let entry = entry?;
//                 Ok(entry)
//             })
//             .filter_map(|entry: Result<DirEntry, io::Error>| {
//                 let path = entry.unwrap().path();
//                 if path.is_folder() {
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
//

// impl SiteConfigV42 {
//     #[instrument(skip(self))]
//     pub fn load_sections(&mut self) -> Result<(), NeoError> {
//         event!(Level::INFO, "Loading Theme Sections");
//         let section_root = self.theme_input_root_folder().join("sections");
//         let section_categories: Vec<&str> = vec![
//             "basic",
//             "block",
//             "checklist",
//             "checklist_item",
//             "json5",
//             "list",
//             "list_item",
//             "raw",
//         ];
//         section_categories.iter().for_each(|category| {
//             let category_folder = section_root.join(category);
//             if let Ok(section_folders) = get_folders_in_folder(&category_folder) {
//                 section_folders.iter().for_each(|section_folder| {
//                     let json5_path = section_folder.join("_settings.json5");
//                     let _ = match fs::read_to_string(&json5_path) {
//                         Ok(text) => match serde_json5::from_str::<SectionSettings>(&text) {
//                             Ok(_j) => {
//                                 let section_name = section_folder
//                                     .file_stem()
//                                     .unwrap()
//                                     .to_string_lossy()
//                                     .to_string();
//                                 if !section_name.starts_with(".") {
//                                     // check to see if there's a settings.json5 file
//                                     if category.eq_ignore_ascii_case("basic") {
//                                         self.sections.basic.push(section_name.clone());
//                                     } else if category.eq_ignore_ascii_case("block") {
//                                         self.sections.block.push(section_name.clone());
//                                     } else if category.eq_ignore_ascii_case("checklist") {
//                                         self.sections.checklist.push(section_name.clone());
//                                     } else if category.eq_ignore_ascii_case("checklist-item") {
//                                         self.sections.checklist_item.push(section_name.clone());
//                                     } else if category.eq_ignore_ascii_case("json5") {
//                                         self.sections.json5.push(section_name.clone());
//                                     } else if category.eq_ignore_ascii_case("list") {
//                                         self.sections.list.push(section_name.clone());
//                                     } else if category.eq_ignore_ascii_case("list-item") {
//                                         self.sections.list_item.push(section_name.clone());
//                                     } else if category.eq_ignore_ascii_case("raw") {
//                                         self.sections.raw.push(section_name.clone());
//                                     }
//                                 }
//                                 Ok(())
//                             }
//                             Err(e) => {
//                                 event!(
//                                     Level::ERROR,
//                                     "Could not load _settings.json5 file: {} - {}",
//                                     e,
//                                     json5_path.display()
//                                 );
//                                 Err(NeoError {
//                                     kind: NeoErrorKind::GenericErrorWithoutSourcePath {
//                                         msg: format!(
//                                             "Could not load _settings.json5 file: {} - {}",
//                                             e,
//                                             json5_path.display(),
//                                         ),
//                                     },
//                                 })
//                             }
//                         },
//                         Err(e) => {
//                             event!(
//                                 Level::ERROR,
//                                 "Could not load _settings.json5 file: {} - {}",
//                                 e,
//                                 json5_path.display()
//                             );
//                             Err(NeoError {
//                                 kind: NeoErrorKind::GenericErrorWithoutSourcePath {
//                                     msg: format!(
//                                         "Could not load _settings.json5 file: {} - {}",
//                                         e,
//                                         json5_path.display(),
//                                     ),
//                                 },
//                             })
//                         }
//                     };
//                 })
//             }
//         });
//         Ok(())
//     }
// }

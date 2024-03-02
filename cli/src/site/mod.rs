pub mod new;
pub mod object;

use crate::child::Child;
use crate::config::Config;
use crate::folder_menu_item::FolderMenuItem;
use crate::folder_menu_item::FolderMenuItemType;
use crate::page::Page;
use crate::section::Section;
use crate::section_category::SectionCategory;
use crate::span::Span;
use itertools::Itertools;
use minijinja::Value;
use serde::Serialize;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fmt::Display;
use std::path::PathBuf;
use std::sync::Mutex;
use tracing::{event, instrument, Level};

#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub struct Site {
    pub cache: Mutex<BTreeMap<String, BTreeMap<String, CacheObject>>>,
    pub config: Config,
    pub pages: BTreeMap<String, Page>,
    pub invalid_pages: BTreeMap<PathBuf, String>,
    pub templates: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum CacheObject {
    Menu(Vec<FolderMenuItem>),
    Value(Value),
    OptionString(Option<String>),
}

impl Site {
    #[instrument(skip(self))]
    pub fn get_cache(&self, name: &str, key: &str) -> Option<CacheObject> {
        let binding = self.cache.lock().unwrap();
        match binding.get(name) {
            Some(cache) => match cache.get(key) {
                Some(obj) => Some(obj.clone()),
                None => None,
            },
            None => None,
        }
    }

    // TODO: Convert this from an Option into a Result for the return value
    #[instrument(skip(self))]
    pub fn set_cache(&self, name: &str, key: String, obj: CacheObject) -> Option<CacheObject> {
        let mut binding = self.cache.lock().unwrap();
        match binding.get_mut(name) {
            Some(cache) => match cache.insert(key, obj) {
                Some(_) => None,
                None => None,
            },
            None => None,
        }
    }

    #[instrument(skip(self))]
    pub fn log_from_template(&self, args: &[Value]) -> String {
        event!(Level::INFO, "{}", args[0].to_string());
        "".to_string()
    }

    pub fn folder_menu(&self, args: &[Value]) -> Vec<FolderMenuItem> {
        let mut items = self.folder_menu_builder(args);
        items
            .iter_mut()
            .for_each(|item| self.folder_menu_set_open_closed_folders(args, item));
        items
    }

    pub fn folder_menu_set_open_closed_folders(&self, args: &[Value], item: &mut FolderMenuItem) {
        if matches!(item.item_type, FolderMenuItemType::OpenDirectory) {
            let page_folders = self.page_folders(args);
            if page_folders
                .into_iter()
                .take(item.folders.len())
                .collect::<Vec<String>>()
                == item.folders
            {
                item.item_type = FolderMenuItemType::OpenDirectory;
            } else {
                item.item_type = FolderMenuItemType::ClosedDirectory;
            }
        }

        // item.folders.iter().enumerate().for_each(|(index, folder)| {
        //     dbg!("asdf");
        //     ()
        // });

        // if item
        //     .folders
        //     .iter()
        //     .all(|folder| page_folders.contains(folder))
        // {
        //     item.item_type = FolderMenuItemType::OpenDirectory;
        // } else {
        //     dbg!(&page_folders);
        //     dbg!(&item.folders);
        //     item.item_type = FolderMenuItemType::ClosedDirectory;
        // }
    }

    #[instrument(skip(self))]
    pub fn folder_menu_builder(&self, args: &[Value]) -> Vec<FolderMenuItem> {
        let menu_key = args[1]
            .try_iter()
            .unwrap()
            .into_iter()
            .map(|f| f.try_iter().unwrap().into_iter().join("-"))
            .join("-");
        match self.get_cache("menus", &menu_key) {
            Some(menu_cache) => {
                if let CacheObject::Menu(menu) = menu_cache {
                    menu
                } else {
                    vec![]
                }
            }
            None => {
                let r: Vec<FolderMenuItem> = args[1]
                    .try_iter()
                    .unwrap()
                    .filter_map(|folder_vecs| {
                        let folder_pattern: Vec<String> = folder_vecs
                            .try_iter()
                            .unwrap()
                            .map(|f| f.to_string())
                            .collect();
                        self.folder_menu_index_finder(folder_pattern)
                    })
                    .collect();
                self.set_cache("menus", menu_key, CacheObject::Menu(r.clone()));
                r
            }
        }

        // event!(Level::INFO, "fn folder_menu_builder");
        // let mut binding = self.cache.lock().unwrap();
        // let menus = binding.get("menus").unwrap();
        // match menus.get(&menu_key) {
        //     Some(menu_cache) => {
        //         if let CacheObject::Menu(menu) = menu_cache {
        //             menu.to_vec()
        //         } else {
        //             vec![]
        //         }
        //     }
        //     None => {
        //         let r = args[1]
        //             .try_iter()
        //             .unwrap()
        //             .filter_map(|folder_vecs| {
        //                 let folder_pattern: Vec<String> = folder_vecs
        //                     .try_iter()
        //                     .unwrap()
        //                     .map(|f| f.to_string())
        //                     .collect();
        //                 self.folder_menu_index_finder(folder_pattern)
        //             })
        //             .collect();
        //         r
        //     }
        // }

        // let r = args[1]
        //     .try_iter()
        //     .unwrap()
        //     .filter_map(|folder_vecs| {
        //         let folder_pattern: Vec<String> = folder_vecs
        //             .try_iter()
        //             .unwrap()
        //             .map(|f| f.to_string())
        //             .collect();
        //         self.folder_menu_index_finder(folder_pattern)
        //     })
        //     .collect();
        // r
        // }
    }

    #[instrument(skip(self))]
    pub fn folder_menu_index_finder(&self, pattern: Vec<String>) -> Option<FolderMenuItem> {
        event!(Level::INFO, "fn folder_menu_index_finder");
        let mut full_pattern_with_file = pattern.clone();
        full_pattern_with_file.push("_title.neo".to_string());
        self.pages.iter().find_map(|page| {
            event!(Level::DEBUG, "{}", page.0);
            let page_args = [Value::from(page.1.id.clone())];
            if full_pattern_with_file == self.page_path_parts(&[Value::from(page.1.id.clone())]) {
                let mut fmi = FolderMenuItem {
                    page_id: page.1.id.clone(),
                    // is_current_link: false,
                    title: self.page_title(&[Value::from(page.1.id.clone())]),
                    href: self.page_href(&[Value::from(page.1.id.clone())]),
                    children: self.folder_menu_child_item_finder(&pattern),
                    item_type: FolderMenuItemType::OpenDirectory,
                    folders: self.page_folders(&page_args),
                };
                // TODO: Get sub folders here
                let mut next_folders: Vec<FolderMenuItem> =
                    self.folder_menu_subfolder_finder(&pattern);
                fmi.children.append(&mut next_folders);
                Some(fmi)
            } else {
                None
            }
        })
    }

    #[instrument(skip(self))]
    pub fn folder_menu_subfolder_finder(&self, pattern: &Vec<String>) -> Vec<FolderMenuItem> {
        event!(Level::INFO, "fn folder_menu_subfolder_finder");
        let mut next_level_folders: BTreeSet<Vec<String>> = BTreeSet::new();
        self.pages.iter().for_each(|page| {
            let page_folders = self.page_folders(&[Value::from(page.1.id.clone())]);
            if page_folders
                .iter()
                .take(pattern.len())
                .eq(pattern.clone().iter())
            {
                if page_folders.len() == pattern.len() + 1 {
                    next_level_folders.insert(page_folders);
                }
            }
        });
        next_level_folders
            .iter()
            .filter_map(|pat| self.folder_menu_index_finder(pat.clone()))
            .collect()
    }

    #[instrument(skip(self))]
    pub fn folder_menu_child_item_finder(&self, pattern: &Vec<String>) -> Vec<FolderMenuItem> {
        event!(Level::INFO, "fn folder_menu_child_item_finder");
        let mut full_pattern_with_file = pattern.clone();
        full_pattern_with_file.push("_title.neo".to_string());
        self.pages
            .iter()
            .filter_map(|page| {
                let page_args = [Value::from(page.1.id.clone())];
                let page_folders = self.page_folders(&[Value::from(page.1.id.clone())]);
                let path_parts = self.page_path_parts(&[Value::from(page.1.id.clone())]);
                if &page_folders == pattern && path_parts != full_pattern_with_file {
                    let fmi = FolderMenuItem {
                        page_id: page.1.id.clone(),
                        // is_current_link: false,
                        title: self.page_title(&[Value::from(page.1.id.clone())]),
                        href: self.page_href(&[Value::from(page.1.id.clone())]),
                        children: vec![],
                        item_type: FolderMenuItemType::File,
                        folders: self.page_folders(&page_args),
                    };
                    Some(fmi)
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn link_or_title(&self, args: &[Value]) -> Option<String> {
        let current_page_id = args[0].to_string();
        let target_page_id = args[1].to_string();
        if current_page_id == target_page_id {
            match self.pages.get(&target_page_id) {
                Some(_) => Some(format!(
                    r#"{}"#,
                    self.page_title(&[Value::from(target_page_id.clone())])
                        .unwrap(),
                )),
                None => None,
            }
        } else {
            match self.pages.get(&target_page_id) {
                Some(_) => Some(format!(
                    r#"<a href="{}">{}</a>"#,
                    self.page_href(&[Value::from(target_page_id.clone())])
                        .unwrap(),
                    self.page_title(&[Value::from(target_page_id.clone())])
                        .unwrap(),
                )),
                None => None,
            }
        }
    }

    pub fn page_folders(&self, args: &[Value]) -> Vec<String> {
        let id = args[0].to_string();
        match self.pages.get(&id) {
            Some(page) => {
                // dbg!(&page.source_path);
                // dbg!(&self.config.folders.content_root.clone());
                page.source_path
                    .strip_prefix(&self.config.folders.content_root.clone())
                    .unwrap()
                    .parent()
                    .unwrap()
                    .components()
                    .map(|c| c.as_os_str().to_string_lossy().to_string().to_lowercase())
                    .collect()
            }
            None => vec![],
        }
    }

    pub fn page_href(&self, args: &[Value]) -> Option<String> {
        let id = args[0].to_string();
        match self.pages.get(&id) {
            Some(_) => Some(format!(
                "/{}/{}/?{}",
                self.config.default_language,
                id,
                self.page_href_title(&id).unwrap()
            )),
            None => None,
        }
    }

    pub fn page_href_title(&self, id: &str) -> Option<String> {
        match self.page_title(&[Value::from(id)]) {
            Some(title) => Some(
                urlencoding::encode(&title.to_lowercase().replace(" ", "-").to_string())
                    .into_owned(),
            ),
            None => None,
        }
    }

    pub fn page_ids(&self) -> Vec<String> {
        self.pages.iter().map(|page| page.0.to_string()).collect()
    }

    // #[instrument(skip(self))]
    pub fn page_main_body(&self, args: &[Value]) -> Value {
        // event!(Level::INFO, "running page_main_body");
        if let Some(page) = self.pages.get(&args[0].to_string()) {
            // event!(Level::INFO, "{}", page.source_path.display());
            Value::from_serializable(
                &page
                    .ast
                    .clone()
                    .into_iter()
                    .filter_map(|child| {
                        if let Child::Section(sec) = &child {
                            if self.config.main_body_section_excludes.contains(&sec.r#type) {
                                None
                            } else {
                                Some(child)
                            }
                        } else if let Child::List(sec) = &child {
                            if self.config.main_body_section_excludes.contains(&sec.r#type) {
                                None
                            } else {
                                Some(child)
                            }
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<Child>>(),
            )
        } else {
            Value::from_serializable::<Vec<Child>>(&vec![])
        }
    }

    pub fn page_output_path(&self, args: &[Value]) -> Option<String> {
        let id = args[0].to_string();
        match self.pages.get(&id) {
            Some(page) => match page.ast.iter().find_map(|child| {
                if let Child::Section(section) = child {
                    if &section.r#type == "metadata" {
                        section.key_value_attributes.iter().find_map(|attr| {
                            if attr.0 == "path" {
                                Some(Some(attr.1.to_string()))
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
            }) {
                Some(override_path) => {
                    let mut output_path = self.config.folders.output_root.clone();
                    output_path.push(override_path.unwrap().strip_prefix("/").unwrap());
                    output_path.push("index.html");
                    Some(output_path.display().to_string())
                }
                None => Some(format!(
                    "{}/{}/{}/index.html",
                    self.config.folders.output_root.display(),
                    self.config.default_language,
                    &id,
                )),
            },
            None => None,
        }
        // match self.pages.get(&id) {
        //     Some(_) => Some(format!(
        //         "{}/{}/{}/index.html",
        //         self.config.folders.output_root.display(),
        //         self.config.default_language,
        //         &id,
        //     )),
        //     None => None,
        // }
    }

    pub fn page_path_parts(&self, args: &[Value]) -> Vec<String> {
        let id = args[0].to_string();
        match self.pages.get(&id) {
            Some(page) => {
                // dbg!(&page.source_path);
                // dbg!(&self.config.folders.content_root.clone());
                page.source_path
                    .strip_prefix(&self.config.folders.content_root.clone())
                    .unwrap()
                    .components()
                    .map(|c| c.as_os_str().to_string_lossy().to_string().to_lowercase())
                    .collect()
            }
            None => vec![],
        }
    }

    pub fn page_place_section(&self, args: &[Value]) -> Value {
        let id = args[0].to_string();
        let section_type = args[1].to_string();
        match self.pages.get(&id) {
            Some(page) => page
                .ast
                .iter()
                .filter_map(|child| {
                    if let Child::Section(sec) = &child {
                        if sec.r#type == section_type {
                            Some(Value::from_serializable(child))
                        } else {
                            None
                        }
                    } else if let Child::List(sec) = &child {
                        if sec.r#type == section_type {
                            Some(Value::from_serializable(child))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect(),
            // Value::from_serializable::<Vec<String>>(&vec![]),
            None => Value::from_serializable::<Vec<String>>(&vec![]),
        }
    }

    pub fn page_source_path(&self, args: &[Value]) -> Option<String> {
        let id = args[0].to_string();
        match self.pages.get(&id) {
            Some(page) => Some(page.source_path.display().to_string()),
            None => None,
        }
    }

    pub fn page_status(&self, args: &[Value]) -> Option<String> {
        let id = args[0].to_string();
        match self.pages.get(&id) {
            Some(page) => match page.ast.iter().find_map(|child| {
                if let Child::Section(section) = child {
                    if &section.r#type == "metadata" {
                        section.key_value_attributes.iter().find_map(|attr| {
                            if attr.0 == "status" {
                                Some(Some(attr.1.to_string()))
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
            }) {
                Some(type_from_metadata) => type_from_metadata,
                None => Some("published".to_string()),
            },
            None => None,
        }
    }

    pub fn page_template(&self, args: &[Value]) -> Option<String> {
        let id = args[0].to_string();
        if self.pages.contains_key(&id) {
            let template_searches = vec![
                format!(
                    "pages/{}/{}.jinja",
                    self.page_type(args).unwrap(),
                    self.page_status(args).unwrap(),
                ),
                format!("pages/{}/published.jinja", self.page_type(args).unwrap()),
                format!("pages/post/{}.jinja", self.page_status(args).unwrap()),
                format!("pages/post/published.jinja"),
            ];
            template_searches
                .iter()
                .find_map(|t| match self.templates.get(t) {
                    Some(_) => Some(t),
                    None => None,
                })
                .cloned()
        } else {
            None
        }
    }

    pub fn page_title(&self, args: &[Value]) -> Option<String> {
        let id = args[0].to_string();
        match self.get_cache("page-titles", &id) {
            Some(page_title_cache) => {
                if let CacheObject::OptionString(page_title) = page_title_cache {
                    page_title
                } else {
                    None
                }
            }
            None => {
                let title = match self.pages.get(&id) {
                    Some(page) => {
                        if let Some(title) = page_title_from_metadata(&page.ast) {
                            Some(title)
                        } else if let Some(title) = page_title_from_title_section(&page.ast) {
                            Some(title)
                        } else if let Some(title) = page_title_from_any_section(&page.ast) {
                            Some(title)
                        } else if let Some(title) = page_title_from_first_few_words(&page.ast) {
                            Some(title)
                        } else if let Some(title) = page_title_from_id(&page.ast) {
                            Some(title)
                        } else {
                            Some("no title".to_string())
                        }
                    }
                    None => Some("(missing page)".to_string()),
                };
                self.set_cache("page-titles", id, CacheObject::OptionString(title.clone()));
                title
            }
        }
    }

    pub fn page_type(&self, args: &[Value]) -> Option<String> {
        let id = args[0].to_string();
        match self.pages.get(&id) {
            Some(page) => match page.ast.iter().find_map(|child| {
                if let Child::Section(section) = child {
                    if &section.r#type == "metadata" {
                        section.key_value_attributes.iter().find_map(|attr| {
                            if attr.0 == "type" {
                                Some(Some(attr.1.to_string()))
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
            }) {
                Some(type_from_metadata) => type_from_metadata,
                None => Some("post".to_string()),
            },
            None => None,
        }
    }

    fn prep_cache(&self) {
        // NOTE: everything relies on the cache being set up. So,
        // everything unwraps directly. If something hasn't been
        // added yet it'll trigger an intended panic
        let mut c = self.cache.lock().unwrap();
        c.insert("page-titles".to_string(), BTreeMap::new());
        c.insert("menus".to_string(), BTreeMap::new());
    }
}

fn filter_section(sec: &Section) -> Option<String> {
    let SectionCategory::StandardSectionFull { containers } = &sec.category else {
        return None;
    };
    let first = containers.first()?;
    let Child::Block(thing) = first else {
        return None;
    };
    let spans = thing
        .iter()
        .flat_map(|span| get_span_words(&span))
        .collect::<String>();
    Some(spans)
}

fn get_span_words(span: &Span) -> Vec<String> {
    match span {
        Span::Word { text, .. } => {
            vec![text.to_string()]
        }
        Span::Space { .. } => vec![" ".to_string()],
        Span::StandardSpan { spans, .. } => spans
            .iter()
            .map(|span| get_span_words(&span))
            .collect::<Vec<Vec<String>>>()
            .concat(),
        _ => vec!["".to_string()],
    }
}

fn page_title_from_any_section(ast: &Vec<Child>) -> Option<String> {
    ast.iter().find_map(|child| match child {
        Child::Section(sec) => match sec.key_value_attributes.get("title") {
            Some(title) => Some(title.to_string()),
            None => None,
        },
        _ => None,
    })
}

fn page_title_from_first_few_words(ast: &Vec<Child>) -> Option<String> {
    ast.iter().find_map(|child| match child {
        Child::Section(sec) => {
            let SectionCategory::StandardSectionFull { containers } = &sec.category else {
                return None;
            };
            let first = containers.first()?;
            let Child::Block(thing) = first else {
                return None;
            };
            let spans = thing
                .iter()
                .flat_map(|span| get_span_words(&span))
                .take(11)
                .collect::<String>();
            Some(spans)
        }
        _ => None,
    })
}

fn page_title_from_metadata(ast: &Vec<Child>) -> Option<String> {
    ast.iter().find_map(|child| {
        if let Child::Section(section) = child {
            if &section.r#type == "metadata" {
                section.key_value_attributes.iter().find_map(|attr| {
                    if attr.0 == "title" {
                        Some(Some(attr.1.to_string()))
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
    })?
}

fn page_title_from_id(ast: &Vec<Child>) -> Option<String> {
    ast.iter().find_map(|child| {
        if let Child::Section(section) = child {
            if &section.r#type == "metadata" {
                section.key_value_attributes.iter().find_map(|attr| {
                    if attr.0 == "id" {
                        Some(Some(attr.1.to_string()))
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
    })?
}

fn page_title_from_title_section(ast: &Vec<Child>) -> Option<String> {
    ast.iter().find_map(|child| match child {
        Child::Section(sec) => {
            if sec.r#type == String::from("title") {
                filter_section(sec)
            } else {
                None
            }
        }
        _ => None,
    })
}

impl Display for Site {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

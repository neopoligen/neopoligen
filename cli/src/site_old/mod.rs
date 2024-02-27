pub mod mocks;
pub mod new;
pub mod object;

use crate::child::Child;
use crate::config::Config;
use crate::page::Page;
use crate::section::Section;
use crate::section_category::SectionCategory;
use crate::span::Span;
use minijinja::Value;
use serde::Serialize;
use std::collections::BTreeMap;
use std::fmt::Display;
use std::sync::Mutex;

#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub struct Site {
    pub pages: BTreeMap<String, Page>,
    pub cache: Mutex<BTreeMap<String, BTreeMap<String, Option<String>>>>,
    pub config: Config,
}

impl Site {
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
        match self.page_title(id) {
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

    pub fn page_output_path(&self, args: &[Value]) -> Option<String> {
        let id = args[0].to_string();
        match self.pages.get(&id) {
            Some(_) => Some(format!(
                "{}/{}/{}/index.html",
                self.config.folders.output_root.display(),
                self.config.default_language,
                &id,
            )),
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
            Some(format!(
                "pages/{}/{}.jinja",
                self.page_type(args).unwrap(),
                self.page_status(args).unwrap(),
            ))
        } else {
            None
        }
    }

    pub fn page_title(&self, id: &str) -> Option<String> {
        let mut cache = self.cache.lock().unwrap();
        let page_titles = cache.get_mut("page_title").unwrap();
        match page_titles.get(id) {
            Some(title) => title.clone(),
            None => {
                let title = match self.pages.get(id) {
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
                page_titles.insert(id.to_string(), title.clone());
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
        c.insert("page_title".to_string(), BTreeMap::new());
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

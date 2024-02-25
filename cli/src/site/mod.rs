pub mod mocks;
pub mod new;

use crate::child::Child;
use crate::page::Page;
use crate::section::Section;
use crate::section_category::SectionCategory;
use crate::span::Span;
use serde::Serialize;
use std::collections::BTreeMap;
use std::sync::Mutex;

#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub struct Site {
    pub pages: BTreeMap<String, Page>,
    pub cache: Mutex<BTreeMap<String, BTreeMap<String, Option<String>>>>,
}

impl Site {
    pub fn page_title(&self, id: &str) -> Option<String> {
        let mut cache = self.cache.lock().unwrap();
        let page_titles = cache.get_mut("page_title").unwrap();
        match page_titles.get(id) {
            Some(title) => title.clone(),
            None => {
                let title = match self.pages.get(id) {
                    Some(page) => get_title_section_title(&page.ast),
                    None => Some("(missing page)".to_string()),
                };
                page_titles.insert(id.to_string(), title.clone());
                title
            }
        }
    }

    pub fn page_title_dev(&self, id: &str) -> Option<String> {
        match self.pages.get(id) {
            Some(page) => get_title_from_metadata(&page.ast),
            None => None,
        }

        // let mut cache = self.cache.lock().unwrap();
        // let page_titles = cache.get_mut("page_title").unwrap();
        // match page_titles.get(id) {
        //     Some(title) => title.clone(),
        //     None => {
        //         let title = match self.pages.get(id) {
        //             Some(page) => get_title_section_title(&page.ast),
        //             None => Some("(missing page)".to_string()),
        //         };
        //         page_titles.insert(id.to_string(), title.clone());
        //         title
        //     }
        // }
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

fn get_title_from_metadata(ast: &Vec<Child>) -> Option<String> {
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

fn get_title_section_title(ast: &Vec<Child>) -> Option<String> {
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

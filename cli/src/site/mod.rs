pub mod mocks;

use crate::child::Child;
use crate::page::Page;
use crate::section::Section;
use crate::section_category::SectionCategory;
use crate::span::Span;
use std::collections::BTreeMap;
use std::sync::Mutex;

pub struct Site {
    pub pages: BTreeMap<String, Page>,
    pub cache: Mutex<BTreeMap<String, BTreeMap<String, String>>>,
}

impl Site {
    pub fn page_title(&self, id: &str) -> String {
        let title = match self.pages.get(id) {
            Some(page) => match get_title_section_title(&page.ast) {
                Some(t) => t,
                None => "Untitled".to_string(),
            },
            None => "(missing page)".to_string(),
        };
        title
        //get_title_section_title(&self.pages.)

        // let mut cache = self.cache.lock().unwrap();
        // match cache.get("page_title") {
        //     Some(c) => {}
        //     None => {}
        // }
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

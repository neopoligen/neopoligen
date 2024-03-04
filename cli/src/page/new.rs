use crate::ast::*;
use crate::child::Child;
use crate::config::Config;
use crate::page::*;
use crate::section::Section;
use crate::section_category::SectionCategory;
use crate::span::Span;
use std::path::PathBuf;

impl Page {
    pub fn new(source_path: PathBuf, source: String, config: &Config) -> Option<Page> {
        match ast(source.trim_start(), config) {
            Ok((remainder, ast)) => {
                if remainder == "" {
                    match ast.iter().find_map(|child| {
                        if let Child::Section(section) = child {
                            let section_type = &section.r#type;
                            if section_type == "metadata" {
                                section
                                    .key_value_attributes
                                    .get("id")
                                    .map(|value| value.to_string())
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }) {
                        Some(id) => {
                            let title = title(&ast);
                            Some(Page {
                                ast,
                                id,
                                source,
                                source_path,
                                title,
                            })
                        }
                        None => None,
                    }
                } else {
                    None
                }
            }
            Err(e) => {
                println!("{}", e);
                None
            }
        }
    }
}

fn title(ast: &Vec<Child>) -> Option<String> {
    if let Some(title) = title_from_metadata(ast) {
        Some(title)
    } else if let Some(title) = title_from_title_section(ast) {
        Some(title)
    } else if let Some(title) = title_from_any_section(ast) {
        Some(title)
    } else {
        None
    }
    //                     Some(title)
    //                 } else if let Some(title) = page_title_from_any_section(&page.ast) {
    //                     Some(title)
    //                 } else if let Some(title) = page_title_from_first_few_words(&page.ast) {
    //                     Some(title)
    //                 } else if let Some(title) = page_title_from_id(&page.ast) {
    //                     Some(title)
    //                 } else {
    //                     Some("no title".to_string())
    //                 }

    // let id = args[0].to_string();
    // let cache_id = format!("page-titles-{}", id);
    // match self.get_cache(&cache_id) {
    //     Some(page_title_cache) => {
    //         if let CacheObject::OptionString(page_title) = page_title_cache {
    //             page_title
    //         } else {
    //             None
    //         }
    //     }
    //     None => {
    //         let title = match self.pages.get(&id) {
    //             Some(page) => {
    //                 if let Some(title) = page_title_from_metadata(&page.ast) {
    //                     Some(title)
    //                 } else if let Some(title) = page_title_from_title_section(&page.ast) {
    //                     Some(title)
    //                 } else if let Some(title) = page_title_from_any_section(&page.ast) {
    //                     Some(title)
    //                 } else if let Some(title) = page_title_from_first_few_words(&page.ast) {
    //                     Some(title)
    //                 } else if let Some(title) = page_title_from_id(&page.ast) {
    //                     Some(title)
    //                 } else {
    //                     Some("no title".to_string())
    //                 }
    //             }
    //             None => Some("(missing page)".to_string()),
    //         };
    //         self.set_cache(cache_id, CacheObject::OptionString(title.clone()));
    //         title
    //     }
    // }
}

fn title_from_any_section(ast: &Vec<Child>) -> Option<String> {
    ast.iter().find_map(|child| match child {
        Child::Section(sec) => match sec.key_value_attributes.get("title") {
            Some(title) => Some(title.to_string()),
            None => None,
        },
        _ => None,
    })
}

fn title_from_metadata(ast: &Vec<Child>) -> Option<String> {
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

fn title_from_title_section(ast: &Vec<Child>) -> Option<String> {
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

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
                            let title = title(&id, &ast);
                            let href = href(&id, &ast, &title, &config.default_language);
                            let html_link = None;
                            Some(Page {
                                ast,
                                href,
                                html_link,
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

fn title(id: &String, ast: &Vec<Child>) -> Option<String> {
    if let Some(title) = title_from_metadata(ast) {
        Some(title)
    } else if let Some(title) = title_from_title_section(ast) {
        Some(title)
    } else if let Some(title) = title_from_any_section(ast) {
        Some(title)
    } else if let Some(title) = title_from_first_few_words(ast) {
        Some(title)
    } else {
        Some(id.clone())
    }
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

fn title_from_first_few_words(ast: &Vec<Child>) -> Option<String> {
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

fn href(
    id: &String,
    ast: &Vec<Child>,
    title: &Option<String>,
    default_language: &String,
) -> Option<String> {
    if let Some(response) = ast.iter().find_map(|child| {
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
        response
    } else {
        Some(format!(
            "/{}/{}/?{}",
            default_language.clone(),
            id,
            urlencoding::encode(
                &title
                    .as_ref()
                    .unwrap()
                    .clone()
                    .to_lowercase()
                    .replace(" ", "-")
                    .to_string()
            )
            .into_owned(),
        ))
    }
}

use crate::ast::*;
use crate::child::Child;
use crate::config::Config;
use crate::page::*;
use crate::section::Section;
use crate::section_category::SectionCategory;
use crate::span::Span;
use std::path::PathBuf;
use tracing::{event, instrument, Level};

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
                            let html_link = html_link(&href, &title);
                            let path_parts = path_parts(&source_path, config);
                            let folders = folders(&source_path, config);
                            let r#type = r#type(&ast, &folders);
                            let status = status(&ast);
                            let tags = tags(&id, &folders, &ast, r#type.clone(), status.clone());
                            let scripts = scripts(&ast);
                            let stylesheets = stylesheets(&ast);
                            let head = head(&ast);
                            Some(Page {
                                ast,
                                folders,
                                head,
                                href,
                                html_link,
                                id,
                                path_parts,
                                scripts,
                                source,
                                source_path,
                                status,
                                stylesheets,
                                tags,
                                title,
                                r#type,
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

fn head(ast: &Vec<Child>) -> Vec<String> {
    ast.iter()
        .filter_map(|child| {
            if let Child::Section(section) = child {
                if &section.r#type == "head" {
                    match &section.category {
                        SectionCategory::PreformattedSectionFull { text } => text.clone(),
                        SectionCategory::PreformattedSectionStart { text } => text.clone(),
                        _ => None,
                    }
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect()
}

fn scripts(ast: &Vec<Child>) -> Vec<String> {
    ast.iter()
        .filter_map(|child| {
            if let Child::Section(section) = child {
                if &section.r#type == "script" {
                    match &section.category {
                        SectionCategory::PreformattedSectionFull { text } => {
                            let attrs = &section
                                .key_value_attributes
                                .iter()
                                .map(|(k, v)| format!(r#" {}="{}""#, k, v))
                                .collect::<Vec<String>>()
                                .join("");
                            Some(format!(
                                "<script{}>{}</script>",
                                attrs,
                                text.clone().unwrap()
                            ))
                        }
                        SectionCategory::PreformattedSectionStart { text } => {
                            let attrs = &section
                                .key_value_attributes
                                .iter()
                                .map(|(k, v)| format!(r#" {}="{}""#, k, v))
                                .collect::<Vec<String>>()
                                .join("");
                            Some(format!(
                                "<script{}>{}</script>",
                                attrs,
                                text.clone().unwrap()
                            ))
                        }
                        _ => None,
                    }
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect()
}

fn scripts_old(ast: &Vec<Child>) -> Vec<String> {
    ast.iter()
        .filter_map(|child| {
            if let Child::Section(section) = child {
                if &section.r#type == "script" {
                    match &section.category {
                        SectionCategory::PreformattedSectionFull { text } => {
                            dbg!(&child);
                            Some(format!("<script>{}</script>", text.clone().unwrap()))
                        }
                        SectionCategory::PreformattedSectionStart { text } => {
                            dbg!(&child);
                            Some(format!("<script>{}</script>", text.clone().unwrap()))
                        }
                        _ => None,
                    }
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect()
}

fn stylesheets(ast: &Vec<Child>) -> Vec<String> {
    ast.iter()
        .filter_map(|child| {
            if let Child::Section(section) = child {
                if &section.r#type == "css" {
                    match &section.category {
                        SectionCategory::PreformattedSectionFull { text } => text.clone(),
                        SectionCategory::PreformattedSectionStart { text } => text.clone(),
                        _ => None,
                    }
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect()
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
                .collect::<Vec<String>>();
            if spans.len() > 28 {
                let mut title = spans
                    .iter()
                    .take(24)
                    .map(|s| s.to_string())
                    .collect::<String>()
                    .trim()
                    .replace("  ", " ");
                if title.ends_with(".") {
                    title.push_str("..");
                } else {
                    title.push_str("...");
                }
                Some(title)
            } else {
                Some(spans.join("").replace("  ", " "))
            }
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

fn folders(source_path: &PathBuf, config: &Config) -> Vec<String> {
    source_path
        .strip_prefix(config.folders.content_root.clone())
        .unwrap()
        .parent()
        .unwrap()
        .components()
        .map(|c| c.as_os_str().to_string_lossy().to_string().to_lowercase())
        .collect()
}

fn html_link(href: &Option<String>, title: &Option<String>) -> Option<String> {
    Some(format!(
        r#"<a href="{}">{}</a>"#,
        href.as_ref().unwrap().clone(),
        title.as_ref().unwrap().clone()
    ))
}

#[instrument]
fn path_parts(source_path: &PathBuf, config: &Config) -> Vec<String> {
    event!(Level::INFO, r#"path_part"#);
    source_path
        .clone()
        .strip_prefix(config.folders.content_root.clone())
        .unwrap()
        .components()
        .map(|c| c.as_os_str().to_string_lossy().to_string().to_lowercase())
        .collect()
}

fn status(ast: &Vec<Child>) -> Option<String> {
    match ast.iter().find_map(|child| {
        if let Child::Section(section) = child {
            if &section.r#type == "metadata" {
                section.key_value_attributes.iter().find_map(|attr| {
                    if attr.0 == "status" {
                        Some(attr.1.to_string())
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
        Some(s) => Some(s),
        None => Some("published".to_string()),
    }
}

fn tags(
    id: &String,
    folders: &Vec<String>,
    ast: &Vec<Child>,
    r#type: Option<String>,
    status: Option<String>,
) -> BTreeSet<String> {
    let mut tags = BTreeSet::new();
    tags.insert(id.to_string());
    folders.iter().for_each(|folder| {
        tags.insert(folder.to_string());
    });
    ast.iter().for_each(|child| {
        if let Child::Section(section) = child {
            if &section.r#type == "tags" {
                section.flag_attributes.iter().for_each(|attr| {
                    tags.insert(attr.to_string());
                });
            }
        }
    });
    if let Some(type_to_add) = r#type {
        tags.insert(type_to_add);
    }
    if let Some(status_to_add) = status {
        tags.insert(status_to_add);
    }
    tags
}

fn r#type(ast: &Vec<Child>, folders: &Vec<String>) -> Option<String> {
    match ast.iter().find_map(|child| {
        if let Child::Section(section) = child {
            if &section.r#type == "metadata" {
                section.key_value_attributes.iter().find_map(|attr| {
                    if attr.0 == "type" {
                        Some(attr.1.to_string())
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
        Some(t) => Some(t),
        None => {
            if folders.len() > 0 {
                Some(folders[0].clone())
            } else {
                Some("post".to_string())
            }
        }
    }
}

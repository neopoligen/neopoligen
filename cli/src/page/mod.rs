// use crate::page_error::PageError;
use crate::ast::ast;
use crate::neo_error::*;
use crate::section::*;
use crate::site_config::SiteConfig;
use crate::span::Span;
use regex::Regex;
use serde::Serialize;
use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub struct Page {
    pub ast: Option<Vec<Section>>,           // Migration DONE
    pub do_render: bool,                     // Migration N/A
    pub error: Option<NeoError>,             // TODO
    pub folders: Vec<String>,                // TODO
    pub full_output_path: Option<PathBuf>,   // Migration N/A
    pub full_cache_path: Option<PathBuf>,    // Migration N/A
    pub href: Option<String>,                // Migration DONE
    pub id: Option<String>,                  // Migration DONE
    pub rel_output_path: Option<PathBuf>,    // Migration DONE
    pub source_path: PathBuf,                // Migration DONE
    pub source_text: String,                 // Migration DONE
    pub tags: Vec<String>,                   // TODO
    pub title_as_plain_text: Option<String>, // Migration DONE
    pub title_for_url: Option<String>,       // Migration DONE
}

impl Page {
    pub fn new(source_text: String, source_path: PathBuf, config: &SiteConfig) -> Page {
        match ast(&source_text, &config.sections.clone(), &config.spans) {
            Ok(ast) => {
                match get_page_id(&ast, &source_text) {
                    Ok(id) => {
                        let title_as_plain_text = title_as_plain_text(&id, &ast);
                        let title_for_url = title_for_url(&title_as_plain_text);
                        let rel_output_path = get_rel_output_path(&id, &ast, &config);
                        let href = href(&ast, &title_for_url, &rel_output_path);
                        Page {
                            ast: Some(ast),
                            do_render: true,
                            error: None,
                            folders: vec![],
                            full_cache_path: None,
                            full_output_path: None,
                            href,
                            id: Some(id),
                            rel_output_path,
                            source_path,
                            source_text,
                            tags: vec![],
                            title_as_plain_text,
                            title_for_url,
                        }
                    }
                    Err(e) => {
                        // let output_path = replace_path(
                        //     &source_path,
                        //     config.paths.get("content_root").unwrap(),
                        //     config.paths.get("errors_root").unwrap(),
                        // )
                        // .unwrap();
                        Page {
                            ast: None,
                            do_render: false,
                            error: Some(e),
                            folders: vec![],
                            full_cache_path: None,
                            full_output_path: None,
                            id: None,
                            href: None,
                            // output_path: Some(output_path.with_extension("txt")),
                            rel_output_path: None,
                            source_path,
                            source_text,
                            tags: vec![],
                            title_as_plain_text: None,
                            title_for_url: None,
                        }
                    }
                }
            }
            Err(error) => {
                // let output_path = replace_path(
                //     &source_path,
                //     config.paths.get("content_root").unwrap(),
                //     config.paths.get("errors_root").unwrap(),
                // )
                // .unwrap();
                Page {
                    ast: None,
                    do_render: false,
                    error: Some(error),
                    folders: vec![],
                    full_cache_path: None,
                    full_output_path: None,
                    href: None,
                    id: None,
                    // output_path: Some(output_path.with_extension("txt")),
                    rel_output_path: None,
                    source_path,
                    source_text,
                    tags: vec![],
                    title_as_plain_text: None,
                    title_for_url: None,
                }
            }
        }
    }

    pub fn plain_text_from_spans(spans: &Vec<Span>) -> Option<String> {
        let strings = spans
            .iter()
            .filter_map(|s| match s {
                Span::WordPart { text, .. } => Some(text.to_string()),
                Span::Space { .. } => Some(" ".to_string()),
                Span::KnownSpan { spans, .. } => Page::plain_text_from_spans(&spans),
                _ => None,
            })
            .collect::<Vec<String>>();
        if strings.len() > 0 {
            Some(strings.join(""))
        } else {
            None
        }
    }

    //
}

fn get_page_id(ast: &Vec<Section>, source_text: &str) -> Result<String, NeoError> {
    match ast.iter().find_map(|sec_enum| {
        // dbg!(&sec_enum);
        if let Section::Yaml { r#type, attrs, .. } = sec_enum {
            if r#type == "metadata" {
                attrs.iter().find_map(|attr| {
                    if attr.0 == "id" {
                        Some(attr.1.trim().to_string())
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
        Some(v) => Ok(v),
        None => Err(NeoError {
            kind: NeoErrorKind::MissingIdError {
                source: source_text.to_string(),
            },
        }),
    }
}

fn get_page_path(ast: &Vec<Section>) -> Option<PathBuf> {
    //! TODO: prevent '..' directory movement
    ast.iter().find_map(|sec_enum| {
        if let Section::Yaml { r#type, attrs, .. } = sec_enum {
            if r#type == "metadata" {
                attrs.iter().find_map(|attr| {
                    if attr.0 == "path" {
                        Some(PathBuf::from(attr.1.trim().to_string()))
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
    })
}

fn get_rel_output_path(id: &str, ast: &Vec<Section>, config: &SiteConfig) -> Option<PathBuf> {
    match get_page_path(ast) {
        Some(mut path) => {
            if path.is_absolute() {
                path = path.strip_prefix("/").unwrap().to_path_buf();
            }
            let full_path = PathBuf::from("/").join(path);
            match full_path.extension() {
                Some(_) => Some(full_path),
                None => Some(full_path.join(PathBuf::from("index.html"))),
            }
        }
        None => Some(PathBuf::from("/").join(format!(
            "{}/{}/index.html",
            config.default_language.clone(),
            id
        ))),
    }
}

fn href(ast: &Vec<Section>, title: &Option<String>, base_url: &Option<PathBuf>) -> Option<String> {
    //! This is the href that is used for linking to the
    //! page. It's the path to the directory followed by the
    //! page title formatted for the URL as a query param
    if let Some(p) = get_page_path(ast) {
        Some(p.to_string_lossy().to_string())
    } else {
        if let (Some(t), Some(p)) = (title, base_url) {
            let output = p.parent().unwrap().join(format!("?{}", t));
            Some(output.to_string_lossy().to_string())
        } else {
            None
        }
    }
}

fn title_as_plain_text(id: &String, ast: &Vec<Section>) -> Option<String> {
    //! This is the main function that produces the titles use
    //! in the app. It tries to get the title from the metadata,
    //! a title section, any section with a title attribute, or
    //! the first few words of a basic section. If none of those
    //! work it falls back to using the id of the page
    let text = if let Some(title) = title_from_metadata(ast) {
        title
    } else if let Some(title) = title_from_title_section(ast) {
        title
    } else if let Some(title) = title_from_any_section(ast) {
        title
    } else if let Some(title) = title_from_first_few_words(ast) {
        title
    } else {
        id.to_string()
    };
    Some(text.trim().to_string())
}

fn title_for_url(plain_text_title: &Option<String>) -> Option<String> {
    if let Some(original) = plain_text_title {
        let re1 = Regex::new(r"\W").unwrap();
        let re2 = Regex::new(r"-+").unwrap();
        let re3 = Regex::new(r"^-").unwrap();
        let re4 = Regex::new(r"-$").unwrap();
        let mut updated = original.to_lowercase();
        updated = re1.replace_all(&updated, "-").to_string();
        updated = re2.replace_all(&updated, "-").to_string();
        updated = re3.replace_all(&updated, "").to_string();
        updated = re4.replace_all(&updated, "").to_string();
        Some(updated.to_string())
    } else {
        None
    }
}

fn title_from_any_section(ast: &Vec<Section>) -> Option<String> {
    //! Support for title_as_plain_text
    ast.iter().find_map(|child| {
        match child {
            Section::Basic { attrs, .. } => attrs.get("title"),
            _ => None,
        }
        .cloned()
    })
}

fn title_from_first_few_words(ast: &Vec<Section>) -> Option<String> {
    //! Support for title_as_plain_text
    ast.iter().find_map(|sec_enum| match sec_enum {
        Section::Basic { children, .. } => {
            if children.len() > 0 {
                if let Section::Block { spans, .. } = &children[0] {
                    if let Some(full_block) = Page::plain_text_from_spans(&spans) {
                        let words = full_block.split(" ").take(9).collect::<Vec<&str>>();
                        Some(words.join(" "))
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        }
        _ => None,
    })
}

fn title_from_metadata(ast: &Vec<Section>) -> Option<String> {
    //! Support for title_as_plain_text
    ast.iter().find_map(|sec_enum| {
        if let Section::Yaml { r#type, attrs, .. } = sec_enum {
            if r#type == "metadata" {
                attrs.iter().find_map(|attr| {
                    if attr.0 == "title" {
                        Some(attr.1.trim().to_string())
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
    })
}

fn title_from_title_section(ast: &Vec<Section>) -> Option<String> {
    //! Support for title_as_plain_text
    ast.iter().find_map(|sec_enum| match sec_enum {
        Section::Basic {
            r#type, children, ..
        } => {
            if *r#type == String::from("title") {
                if children.len() > 0 {
                    if let Section::Block { spans, .. } = &children[0] {
                        Page::plain_text_from_spans(&spans)
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        }
        _ => None,
    })
}

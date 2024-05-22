// use crate::page_error::PageError;
use crate::ast::ast;
use crate::error::*;
use crate::section::*;
use crate::site_config::SiteConfig;
use crate::span::Span;
use serde::Serialize;
use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub struct Page {
    pub ast: Option<Vec<Section>>,
    pub error: Option<Error>,
    pub folders: Vec<String>,
    pub id: Option<String>,
    pub rel_output_path: Option<PathBuf>,
    pub source_path: PathBuf,
    pub source_text: String,
    pub tags: Vec<String>,
    pub title_as_plain_text: Option<String>,
}

impl Page {
    pub fn new(source_text: String, source_path: PathBuf, config: &SiteConfig) -> Page {
        match ast(&source_text, &config.sections.clone(), &config.spans) {
            Ok(ast) => {
                // dbg!(&ast);
                match get_page_id(&ast, &source_text) {
                    Ok(id) => {
                        let title_as_plain_text = get_title_as_plain_text(&id, &ast);
                        let rel_output_path = get_rel_output_path(&id, &ast, &config);
                        Page {
                            ast: Some(ast),
                            error: None,
                            folders: vec![],
                            id: Some(id),
                            rel_output_path,
                            source_path,
                            source_text,
                            tags: vec![],
                            title_as_plain_text,
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
                            error: Some(e),
                            folders: vec![],
                            id: None,
                            // output_path: Some(output_path.with_extension("txt")),
                            rel_output_path: None,
                            source_path,
                            source_text,
                            tags: vec![],
                            title_as_plain_text: None,
                        }
                    }
                }
            }
            Err(error) => {
                // dbg!(&error);
                // let output_path = replace_path(
                //     &source_path,
                //     config.paths.get("content_root").unwrap(),
                //     config.paths.get("errors_root").unwrap(),
                // )
                // .unwrap();
                Page {
                    ast: None,
                    error: Some(error),
                    folders: vec![],
                    id: None,
                    // output_path: Some(output_path.with_extension("txt")),
                    rel_output_path: None,
                    source_path,
                    source_text,
                    tags: vec![],
                    title_as_plain_text: None,
                }
            }
        }
    }
}

fn get_page_id(ast: &Vec<Section>, source_text: &str) -> Result<String, Error> {
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
        None => Err(Error {
            kind: ErrorKind::MissingIdError {
                source: source_text.to_string(),
            },
        }),
    }
}

fn get_page_path(ast: &Vec<Section>) -> Option<PathBuf> {
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

// fn replace_path(path: &PathBuf, find: &PathBuf, replace: &PathBuf) -> Result<PathBuf, String> {
//     match path.strip_prefix(find) {
//         Ok(path_part) => Ok(replace.clone().join(path_part)),
//         Err(e) => Err(format!("{}", e)), // todo make this a better error
//     }
// }

fn get_rel_output_path(id: &str, ast: &Vec<Section>, config: &SiteConfig) -> Option<PathBuf> {
    match get_page_path(ast) {
        Some(mut path) => {
            // let mut full_path = config.paths.get("output_root").unwrap().join(path);
            if path.is_absolute() {
                path = path.strip_prefix("/").unwrap().to_path_buf();
            }
            let full_path = PathBuf::from("/").join(path);
            match full_path.extension() {
                Some(_) => Some(full_path),
                None => Some(full_path.join(PathBuf::from("index.html"))),
            }
            // Some(full_path)
            // Some(config.paths.get("output_root").unwrap().join(format!(
            // "{}/{}/index.html",
            // config.default_language.clone(),
            // id
            // )))
        }
        None => Some(PathBuf::from("/").join(format!(
            "{}/{}/index.html",
            config.default_language.clone(),
            id
        ))),
    }
}

fn get_title_as_plain_text(id: &String, ast: &Vec<Section>) -> Option<String> {
    //title_from_metadata(ast)
    title_from_title_section(ast)
}

fn title_from_metadata(ast: &Vec<Section>) -> Option<String> {
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
    ast.iter().find_map(|sec_enum| match sec_enum {
        Section::Basic {
            r#type, children, ..
        } => {
            if *r#type == String::from("title") {
                if children.len() > 0 {
                    if let Section::Block { spans, .. } = &children[0] {
                        Some(
                            spans
                                .iter()
                                .filter_map(|s| match s {
                                    Span::WordPart { text, .. } => Some(text.to_string()),
                                    Span::Space { .. } => Some(" ".to_string()),
                                    Span::Newline { .. } => Some(" ".to_string()),
                                    _ => None,
                                })
                                .collect::<Vec<String>>()
                                .join(""),
                        )
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

// fn get_page_path(ast: &Vec<Section>) -> Option<PathBuf> {
//     ast.iter().find_map(|sec_enum| {
//         if let Section::Yaml { r#type, attrs, .. } = sec_enum {
//             if r#type == "metadata" {
//                 attrs.iter().find_map(|attr| {
//                     if attr.0 == "path" {
//                         Some(PathBuf::from(attr.1.trim().to_string()))
//                     } else {
//                         None
//                     }
//                 })
//             } else {
//                 None
//             }
//         } else {
//             None
//         }
//     })
// }

// fn get_output_path(id: &str, ast: &Vec<Section>, config: &SiteConfig) -> Option<PathBuf> {
//     match get_page_path(ast) {
//         Some(mut path) => {
//             if path.is_absolute() {
//                 path = path.strip_prefix("/").unwrap().to_path_buf();
//             }
//             let full_path = config.paths.get("output_root").unwrap().join(path);
//             match full_path.extension() {
//                 Some(_) => Some(full_path),
//                 None => Some(full_path.join(PathBuf::from("index.html"))),
//             }
//             // Some(full_path)
//             // Some(config.paths.get("output_root").unwrap().join(format!(
//             // "{}/{}/index.html",
//             // config.default_language.clone(),
//             // id
//             // )))
//         }
//         None => Some(config.paths.get("output_root").unwrap().join(format!(
//             "{}/{}/index.html",
//             config.default_language.clone(),
//             id
//         ))),
//     }
// }

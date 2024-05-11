// use crate::page_error::PageError;
use crate::ast::ast;
use crate::error::*;
use crate::section::*;
use crate::site_config::SiteConfig;
use serde::Serialize;
use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub struct Page {
    pub ast: Option<Vec<Section>>,
    pub error: Option<Error>,
    pub folders: Vec<String>,
    pub id: Option<String>,
    pub output_path: Option<PathBuf>,
    pub source_path: PathBuf,
    pub source_text: String,
    pub tags: Vec<String>,
}

impl Page {
    pub fn new(source_text: String, source_path: PathBuf, config: &SiteConfig) -> Page {
        match ast(&source_text, &config.sections.clone(), &config.spans) {
            Ok(ast) => match get_page_id(&ast, &source_text) {
                Ok(id) => {
                    let output_path = get_output_path(&id, &ast, &config);
                    Page {
                        ast: Some(ast),
                        error: None,
                        folders: vec![],
                        id: Some(id),
                        output_path,
                        source_path,
                        source_text,
                        tags: vec![],
                    }
                }
                Err(e) => {
                    let output_path = replace_path(
                        &source_path,
                        config.paths.get("content_root").unwrap(),
                        config.paths.get("errors_root").unwrap(),
                    )
                    .unwrap();
                    Page {
                        ast: None,
                        error: Some(e),
                        folders: vec![],
                        id: None,
                        output_path: Some(output_path.with_extension("txt")),
                        source_path,
                        source_text,
                        tags: vec![],
                    }
                }
            },
            Err(error) => {
                let output_path = replace_path(
                    &source_path,
                    config.paths.get("content_root").unwrap(),
                    config.paths.get("errors_root").unwrap(),
                )
                .unwrap();
                Page {
                    ast: None,
                    error: Some(error),
                    folders: vec![],
                    id: None,
                    output_path: Some(output_path.with_extension("txt")),
                    source_path,
                    source_text,
                    tags: vec![],
                }
            }
        }
    }
}

fn get_page_id(ast: &Vec<Section>, source_text: &str) -> Result<String, Error> {
    match ast.iter().find_map(|sec_enum| {
        if let Section::Json { r#type, attrs, .. } = sec_enum {
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

fn replace_path(path: &PathBuf, find: &PathBuf, replace: &PathBuf) -> Result<PathBuf, String> {
    match path.strip_prefix(find) {
        Ok(path_part) => Ok(replace.clone().join(path_part)),
        Err(e) => Err(format!("{}", e)), // todo make this a better error
    }
}

fn get_output_path(id: &str, ast: &Vec<Section>, config: &SiteConfig) -> Option<PathBuf> {
    match get_page_path(ast) {
        Some(mut path) => {
            if path.is_absolute() {
                path = path.strip_prefix("/").unwrap().to_path_buf();
            }
            let full_path = config.paths.get("output_root").unwrap().join(path);
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
        None => Some(config.paths.get("output_root").unwrap().join(format!(
            "{}/{}/index.html",
            config.default_language.clone(),
            id
        ))),
    }
}

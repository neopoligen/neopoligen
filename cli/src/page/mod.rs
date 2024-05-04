// use crate::page_error::PageError;
use crate::ast::ast;
use crate::error::*;
use crate::section::*;
use crate::section_attr::SectionAttr;
use crate::site_config::SiteConfigV2;
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
    pub source_text: String,
    pub tags: Vec<String>,
}

impl Page {
    pub fn new(source_text: String, source_path: PathBuf, config: &SiteConfigV2) -> Page {
        match ast(&source_text, &config.sections) {
            Ok(ast) => match get_page_id(&ast) {
                Ok(id) => {
                    let output_path = Some(config.paths.get("output_root").unwrap().join(format!(
                        "{}/{}/index.html",
                        config.default_language.clone(),
                        id.clone()
                    )));
                    Page {
                        ast: Some(ast),
                        error: None,
                        folders: vec![],
                        id: Some(id),
                        output_path,
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
                        output_path: Some(output_path),
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
                    output_path: Some(output_path),
                    source_text,
                    tags: vec![],
                }
            }
        }
    }
}

fn get_page_id(ast: &Vec<Section>) -> Result<String, Error> {
    match ast.iter().find_map(|sec_enum| {
        if let Section::Json { r#type, attrs, .. } = sec_enum {
            if r#type == "metadata" {
                attrs.iter().find_map(|attr| {
                    if let SectionAttr::KeyValue { key, value } = attr {
                        if key == "id" {
                            Some(value.trim().to_string())
                        } else {
                            None
                        }
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
            kind: ErrorKind::MissingIdError {},
        }),
    }
}

fn replace_path(path: &PathBuf, find: &PathBuf, replace: &PathBuf) -> Result<PathBuf, String> {
    match path.strip_prefix(find) {
        Ok(path_part) => Ok(replace.clone().join(path_part)),
        Err(e) => Err("Problem".to_string()), // todo make this a better error
    }
}

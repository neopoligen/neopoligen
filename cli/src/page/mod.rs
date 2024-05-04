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
    pub fn new(source_text: String, config: &SiteConfigV2) -> Page {
        match ast(&source_text, &config.sections) {
            Ok(ast) => match get_page_id(&ast) {
                Ok(id) => Page {
                    ast: Some(ast),
                    error: None,
                    folders: vec![],
                    id: Some(id),
                    output_path: None,
                    source_text,
                    tags: vec![],
                },
                Err(e) => Page {
                    ast: None,
                    error: Some(e),
                    folders: vec![],
                    id: None,
                    output_path: None,
                    source_text,
                    tags: vec![],
                },
            },
            Err(error) => Page {
                ast: None,
                error: Some(error),
                folders: vec![],
                id: None,
                output_path: None,
                source_text,
                tags: vec![],
            },
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

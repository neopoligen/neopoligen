pub mod mocks;

use crate::ast::ast;
use crate::section::Section;
use crate::site_config::SiteConfig;
use serde::Serialize;
use std::path::PathBuf;

#[derive(Clone, Debug, Serialize)]
pub struct PageV2 {
    pub ast: Vec<Section>,
    pub cached_hash: Option<String>,
    pub source_path: Option<PathBuf>,
    pub source_content: Option<String>,
}

impl PageV2 {
    pub fn generate_ast(&mut self, config: &SiteConfig) {
        match ast(
            &self.source_content.clone().unwrap(),
            &config.sections.clone(),
            &config.spans,
        ) {
            Ok(ast) => self.ast = ast,
            Err(_) => {}
        }
    }

    pub fn id(&self) -> Option<String> {
        self.ast.iter().find_map(|sec_enum| {
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
        })
    }

    pub fn new_from_cache(source_path: String, cached_hash: String, _source_ast: String) -> PageV2 {
        PageV2 {
            ast: vec![], // TODO: load in the cached AST here
            cached_hash: Some(cached_hash),
            source_path: Some(PathBuf::from(source_path)),
            source_content: None,
        }
    }

    pub fn new_from_filesystem(source_path: PathBuf, source_content: String) -> PageV2 {
        PageV2 {
            ast: vec![],
            cached_hash: None,
            source_path: Some(source_path),
            source_content: Some(source_content),
        }
    }

    // pub fn hash(&self) -> String {
    //     sha256::digest(&self.source_content)
    // }
}

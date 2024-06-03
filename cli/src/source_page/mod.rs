use crate::ast::parse_ast;
use crate::section::Section;
use crate::site_config::SiteConfig;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SourcePage {
    ast: Option<Vec<Section>>,
    config: Option<SiteConfig>,
    source_content: Option<String>,
    source_path: Option<PathBuf>,
}

impl SourcePage {
    pub fn new_from_source_path(path: &PathBuf, config: SiteConfig) -> Result<SourcePage> {
        let content = fs::read_to_string(path)?;
        Ok(SourcePage {
            ast: None,
            config: Some(config),
            source_content: Some(content),
            source_path: Some(path.clone()),
        })
    }
}

impl SourcePage {
    pub fn generate_ast(&mut self) -> Result<()> {
        let ast = parse_ast(
            self.source_content.as_ref().unwrap(),
            self.config.as_ref().unwrap().sections.clone(),
        )?;
        self.ast = Some(ast);
        Ok(())
    }
}

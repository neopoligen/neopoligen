use crate::ast::parse_ast;
use crate::neo_error::{NeoError, NeoErrorKind};
use crate::section::Section;
use crate::site_config::SiteConfig;
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
    pub fn new_from_source_path(
        path: &PathBuf,
        config: SiteConfig,
    ) -> Result<SourcePage, NeoError> {
        match fs::read_to_string(path) {
            Ok(content) => Ok(SourcePage {
                ast: None,
                config: Some(config),
                source_content: Some(content),
                source_path: Some(path.clone()),
            }),
            Err(e) => Err(NeoError {
                kind: NeoErrorKind::ForwardError { msg: e.to_string() },
            }),
        }
    }
}

impl SourcePage {
    pub fn generate_ast(&mut self) -> Result<(), NeoError> {
        let ast = parse_ast(
            self.source_content.as_ref().unwrap(),
            self.config.as_ref().unwrap().sections.clone(),
        )?;
        self.ast = Some(ast);
        Ok(())
    }
}

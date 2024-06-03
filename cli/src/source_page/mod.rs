pub mod mocks;

use crate::ast::parse_ast;
use crate::neo_error::{NeoError, NeoErrorKind};
use crate::section::Section;
use crate::site_config::SiteConfig;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SourcePage {
    pub ast: Option<Vec<Section>>,
    pub config: Option<SiteConfig>,
    pub source_content: Option<String>,
    pub source_path: Option<PathBuf>,
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

    pub fn new_mock_from_str(source: &str) -> SourcePage {
        let mut p = SourcePage {
            ast: None,
            config: Some(SiteConfig::mock1_basic()),
            source_content: Some(source.to_string()),
            source_path: Some(PathBuf::from("/test/mocks/content/mock-file.neo")),
        };
        let _ = p.generate_ast();
        p
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

    pub fn id(&self) -> Option<String> {
        None
    }
}

#[cfg(test)]
mod test {}

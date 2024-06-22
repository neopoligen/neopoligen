// DEPRECATED: maybe: not sure if going to use this
use crate::ast::parse_ast;
use crate::neo_error::{NeoError, NeoErrorKind};
use crate::section::Section;
use crate::site_config::SiteConfig;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ThemeTest {
    pub ast: Option<Vec<Section>>,
    pub config: Option<SiteConfig>,
    pub source_content: Option<String>,
    pub source_path: PathBuf,
}

impl ThemeTest {
    pub fn new_from_source_path(
        source_path: PathBuf,
        config: SiteConfig,
    ) -> Result<ThemeTest, NeoError> {
        match fs::read_to_string(&source_path) {
            Ok(source_content) => {
                let mut tt = ThemeTest {
                    ast: None,
                    config: Some(config),
                    source_path,
                    source_content: Some(source_content),
                };
                match tt.generate_ast() {
                    Ok(()) => Ok(tt),
                    Err(e) => Err(NeoError {
                        kind: NeoErrorKind::GenericErrorWithSourcePath {
                            source_path: tt.source_path.clone(),
                            msg: format!("Could not load AST for theme test: {}", e),
                        },
                    }),
                }
            }
            Err(e) => Err(NeoError {
                kind: NeoErrorKind::GenericErrorWithSourcePath {
                    source_path: source_path.clone(),
                    msg: format!("Could not load template test file: {}", e),
                },
            }),
        }
    }

    pub fn generate_ast(&mut self) -> Result<(), NeoError> {
        let ast = parse_ast(
            self.source_content.as_ref().unwrap(),
            self.config.as_ref().unwrap().sections.clone(),
        )?;
        self.ast = Some(ast);
        Ok(())
    }
}

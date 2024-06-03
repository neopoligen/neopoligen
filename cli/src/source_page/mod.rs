pub mod mocks;

use crate::ast::parse_ast;
use crate::neo_error::{NeoError, NeoErrorKind};
use crate::section::{Section, SectionKind};
use crate::section_attr::SectionAttrKind;
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

    pub fn get_metadata_item(&self, target: &str) -> Option<String> {
        if let Some(ast) = &self.ast {
            ast.iter().find_map(|section| match &section.kind {
                SectionKind::Yaml {} => {
                    if section.r#type.as_str() == "metadata" {
                        section.attrs.iter().find_map(|attr| match &attr.kind {
                            SectionAttrKind::KeyValue { key, value } => {
                                if key.as_str() == target {
                                    Some(value.to_string())
                                } else {
                                    None
                                }
                            }
                            _ => None,
                        })
                    } else {
                        None
                    }
                }
                _ => None,
            })
        } else {
            None
        }
    }

    pub fn id(&self) -> Option<String> {
        self.get_metadata_item("id")
    }

    pub fn rel_file_path(&self) -> Option<PathBuf> {
        Some(PathBuf::from(format!(
            "{}/{}/index.html",
            self.config.as_ref().unwrap().default_language,
            self.id().unwrap()
        )))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn id_check() {
        let p = SourcePage::mock1_20240101_alfa1234_minimal();
        let left = "20240101_alfa1234".to_string();
        let right = p.id().unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn rel_file_path_default() {
        let p = SourcePage::mock1_20240101_alfa1234_minimal();
        let left = PathBuf::from("en/20240101_alfa1234/index.html");
        let right = p.rel_file_path().unwrap();
        assert_eq!(left, right);
    }
}

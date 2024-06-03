use crate::section::Section;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SourcePage {
    ast: Option<Vec<Section>>,
    source_content: Option<String>,
    source_path: Option<PathBuf>,
}

impl SourcePage {
    pub fn new_from_source_path(path: &PathBuf) -> Result<SourcePage> {
        let content = fs::read_to_string(path)?;
        Ok(SourcePage {
            ast: None,
            source_content: Some(content),
            source_path: Some(path.clone()),
        })
    }
}

impl SourcePage {
    pub fn generate_ast(&mut self) -> Result<()> {
        Ok(())
    }
}

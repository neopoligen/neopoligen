use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SourcePage {
    source_path: Option<PathBuf>,
    source_content: Option<String>,
}

impl SourcePage {
    pub fn new_from_source_path(path: &PathBuf) -> Result<SourcePage> {
        let content = fs::read_to_string(path)?;
        Ok(SourcePage {
            source_content: Some(content),
            source_path: Some(path.clone()),
        })
    }
}

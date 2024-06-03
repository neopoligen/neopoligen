use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;
use std::path::PathBuf;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct EngineConfig {
    pub active_site: String,
}

impl EngineConfig {
    pub fn new_from_file(path: &PathBuf) -> Result<EngineConfig> {
        let text = fs::read_to_string(path)?;
        let config = serde_json::from_str::<EngineConfig>(&text)?;
        Ok(config)
    }
}

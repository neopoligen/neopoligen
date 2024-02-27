pub mod new;

use crate::config::Config;
use crate::page::Page;
use serde::Serialize;
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::Mutex;

#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub struct Site {
    pub pages: BTreeMap<String, Page>,
    pub templates: BTreeMap<String, String>,
    pub cache: Mutex<BTreeMap<String, BTreeMap<String, Option<String>>>>,
    pub config: Config,
}

impl Site {
    pub fn output_files(&self) -> BTreeMap<PathBuf, String> {
        BTreeMap::new()
    }
}

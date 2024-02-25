pub mod mocks;

use crate::child::Child;
use serde::Serialize;
use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub struct Page {
    pub ast: Vec<Child>,
    pub source: String,
    pub source_path: PathBuf,
}

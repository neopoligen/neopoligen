pub mod new;

use crate::child::Child;
use serde::Serialize;
use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub struct Page {
    pub id: String,
    pub ast: Vec<Child>,
    pub source: String,
    pub source_path: PathBuf,
    pub publish: bool,
    // these items are generated on creation
    pub title: Option<String>,
}

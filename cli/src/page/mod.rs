use crate::page_error::PageError;
use crate::section::*;
use serde::Serialize;
use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub struct Page {
    pub ast: Vec<Section>,
    pub id: String,
    // pub tags: Vec<String>,
    pub source_path: PathBuf,
    pub output_path: PathBuf,
    // pub folders: Vec<String>,
}

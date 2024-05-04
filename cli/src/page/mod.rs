use crate::page_error::PageError;
use crate::section::*;
use serde::Serialize;
use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub struct Page {
    ast: Vec<Section>,
    id: String,
    raw_source: String,
    errors: Vec<PageError>,
    tags: Vec<String>,
    source_path: PathBuf,
    output_path: PathBuf,
    folders: Vec<String>,
}

use serde::Serialize;
use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub struct Image {
    pub file_name: String,
    pub file_stem: String,
    pub raw_href: String,
    pub source_path: PathBuf,
}

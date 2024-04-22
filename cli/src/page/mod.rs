pub mod new;

use crate::child::Child;
use serde::Serialize;
use std::collections::BTreeSet;
use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub struct Page {
    pub ast: Vec<Child>,
    pub base_template: Option<String>,
    pub folders: Vec<String>,
    pub head: Vec<String>,
    pub href: Option<String>,
    pub html_link: Option<String>,
    pub id: String,
    pub output_file_path: Option<String>,
    pub path_parts: Vec<String>,
    pub scripts: Vec<String>,
    pub source: String,
    pub source_path: PathBuf,
    pub status: Option<String>,
    pub stylesheets: Vec<String>,
    pub tags: BTreeSet<String>,
    pub title: Option<String>,
    pub r#type: Option<String>,
}

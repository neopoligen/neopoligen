// use minijinja::Value;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum NeoErrorV42 {
    AstParsingError,
    NoParsedPageDataForKey(String),
    TemplateLoadError {
        message: String,
        details: String,
    }, // FullPathWith { path: PathBuf, err: String },
    PageError {
        id: Option<String>,
        href: Option<PathBuf>,
        source_path: Option<PathBuf>,
        message: Option<String>,
        details: Option<String>,
    },
}

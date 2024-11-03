// use minijinja::Value;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "template", rename_all = "lowercase")]
pub enum NeoErrorV42 {
    AstParsingError,
    NoParsedPageDataForKey(String),
    TemplateLoadError {
        message: String,
        details: String,
    }, // FullPathWith { path: PathBuf, err: String },
    #[serde(rename = "built-in/errors-page-error.neoj")]
    PageError {
        id: Option<String>,
        href: Option<PathBuf>,
        rel_path: PathBuf,
        parts: Vec<String>,
    },
}

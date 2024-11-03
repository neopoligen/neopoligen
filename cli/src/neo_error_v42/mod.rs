// use minijinja::Value;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum NeoErrorV42 {
    AstParsingError,
    NoParsedPageDataForKey(String),
    TemplateLoadError {
        message: String,
        details: String,
    }, // FullPathWith { path: PathBuf, err: String },
    PageIssue {
        id: Option<String>,
        href: Option<PathBuf>,
        rel_path: PathBuf,
        parts: Vec<String>,
        template_list: Vec<String>,
    },
}

// #[serde(tag = "template", rename_all = "lowercase")]
// #[serde(rename = "built-in/issues/page-issue/default.neoj")]

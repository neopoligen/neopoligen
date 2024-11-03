// use minijinja::Value;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::page_v42::PageV42b;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum NeoErrorV42 {
    AstParsingError,
    NoParsedPageDataForKey(String),
    TemplateLoadError {
        message: String,
        details: String,
    },
    PageIssue {
        id: Option<String>,
        href: Option<PathBuf>,
        rel_path: PathBuf,
        notes: Vec<String>,
        template_list: Vec<String>,
    },
}

impl NeoErrorV42 {
    pub fn new_page_issue(page: &PageV42b, notes: Vec<String>) -> NeoErrorV42 {
        NeoErrorV42::PageIssue {
            id: page.id.clone(),
            href: page.href.clone(),
            rel_path: page.source_path.clone(),
            notes,
            template_list: vec![
                "issues/page-issue/default.neoj".to_string(),
                "built-in/issues/page-issue/default.neoj".to_string(),
            ],
        }
    }
}

// #[serde(tag = "template", rename_all = "lowercase")]
// #[serde(rename = "built-in/issues/page-issue/default.neoj")]

// pub mod mocks;
pub mod new;
pub mod parse;

use crate::child::Child;
use serde::Serialize;
use std::path::PathBuf;

pub enum ParsedPage {
    ValidPage(Page),
    InvalidPage {
        path: PathBuf,
        source: String,
        remainder: Option<String>,
        error: Option<String>,
    },
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub struct Page {
    pub id: String,
    pub ast: Vec<Child>,
    pub source: String,
    pub source_path: PathBuf,
    pub publish: bool,
}

#[cfg(test)]
mod page_functions {
    // use super::*;
    // use pretty_assertions::assert_eq;
}

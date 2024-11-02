// use minijinja::Value;
use serde::{Deserialize, Serialize};
// use std::path::PathBuf;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum NeoErrorV42 {
    AstParsingError,
    // FullPathWith { path: PathBuf, err: String },
}

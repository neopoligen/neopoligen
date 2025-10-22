use minijinja::Error;
use serde::Deserialize;
use serde::Serialize;
use std::path::PathBuf;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PageError {
    pub id: Option<String>,
    pub source_path: Option<PathBuf>,
    pub output_path: Option<PathBuf>,
    pub message: String,
    pub details: String,
}

impl PageError {
    pub fn new(
        id: Option<String>,
        source_path: Option<PathBuf>,
        output_path: Option<PathBuf>,
        err: &Error,
    ) -> PageError {
        PageError {
            id,
            source_path,
            output_path,
            message: err.to_string(),
            details: err.display_debug_info().to_string(),
        }
    }
}

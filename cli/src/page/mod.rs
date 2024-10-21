use crate::neo_error::NeoError;
use crate::neo_error::NeoErrorKind;
use crate::section::SectionV41;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PageV41 {
    pub ast: Option<Vec<SectionV41>>,
    pub raw_source: String,
    pub fs_path: PathBuf,
}

impl PageV41 {
    pub fn new_from_fs_path(path: &PathBuf) -> Result<PageV41, NeoError> {
        match fs::read_to_string(path) {
            Ok(raw_source) => Ok(PageV41 {
                ast: None,
                fs_path: path.clone(),
                raw_source,
            }),
            Err(e) => Err(NeoError {
                kind: NeoErrorKind::ForwardErrorWithSourcePath {
                    source_path: path.clone(),
                    msg: e.to_string(),
                },
            }),
        }
    }
}

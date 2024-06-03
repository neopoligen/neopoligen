use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PagePayload {
    pub id: String,
    pub rel_file_path: Option<PathBuf>,
}

impl PagePayload {
    pub fn new_from_id(id: &str) -> PagePayload {
        PagePayload {
            id: id.to_string(),
            rel_file_path: None,
        }
    }
}

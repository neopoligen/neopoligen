use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PagePayload {
    pub id: String,
    pub page_type: Option<String>,
    pub rel_file_path: Option<PathBuf>,
    pub status: Option<String>,
    pub template_list: Vec<String>,
}

impl PagePayload {
    pub fn new_from_id(id: &str) -> PagePayload {
        PagePayload {
            id: id.to_string(),
            page_type: None,
            rel_file_path: None,
            status: None,
            template_list: vec![],
        }
    }
}

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PagePayload {
    pub id: String,
    pub rel_file_path: Option<PathBuf>,
    pub r#type: Option<String>,
    pub status: Option<String>,
    pub template_list: Vec<String>,
    pub used_template: Option<String>,
}

impl PagePayload {
    pub fn new_from_id(id: &str) -> PagePayload {
        PagePayload {
            id: id.to_string(),
            r#type: None,
            rel_file_path: None,
            status: None,
            template_list: vec![],
            used_template: None,
        }
    }
}

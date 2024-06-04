use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::section::Section;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PagePayload {
    pub id: String,
    pub rel_file_path: Option<PathBuf>,
    pub r#type: Option<String>,
    pub sections: Vec<Section>,
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
            sections: vec![],
            status: None,
            template_list: vec![],
            used_template: None,
        }
    }
}

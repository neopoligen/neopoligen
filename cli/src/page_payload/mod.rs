use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::{
    neo_error::{NeoError, NeoErrorKind},
    payload_section::PayloadSection,
    payload_span::PayloadSpan,
    source_page::SourcePage,
};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PagePayload {
    pub id: String,
    pub rel_file_path: Option<PathBuf>,
    pub r#type: Option<String>,
    pub sections: Vec<PayloadSection>,
    pub status: Option<String>,
    pub template_list: Vec<String>,
    pub title: Vec<PayloadSpan>,
    pub used_template: Option<String>,
}

impl PagePayload {
    pub fn new_from_source_page(source: &SourcePage) -> Result<PagePayload, NeoError> {
        if let Some(id) = source.id() {
            let mut p = PagePayload {
                id,
                r#type: None,
                rel_file_path: None,
                sections: vec![],
                status: None,
                template_list: vec![],
                title: vec![], // TODO: Add title spans
                used_template: None,
            };
            p.rel_file_path = source.rel_file_path();
            p.template_list = source.template_list();
            p.status = source.status();
            p.r#type = source.r#type();
            p.sections = source.sections();
            Ok(p)
        } else {
            Err(NeoError {
                kind: NeoErrorKind::GenericErrorWithSourcePath {
                    source_path: source.source_path.clone().expect("get source path"),
                    msg: "could not get id for file".to_string(),
                },
            })
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    #[ignore]
    fn add_page_tags_to_sections() {
        // TODO
    }

    #[test]
    #[ignore]
    fn add_section_tags_to_page() {
        // TODO
    }

    #[test]
    #[ignore]
    fn add_page_created_and_updated_to_sections() {
        // TODO
    }

    #[test]
    #[ignore]
    fn add_page_status_to_sections() {
        // TODO
    }
}

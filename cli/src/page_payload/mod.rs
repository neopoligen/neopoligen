use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::{
    helpers::flatten_payload_spans,
    neo_error::{NeoError, NeoErrorKind},
    payload_section::PayloadSection,
    payload_span::PayloadSpan,
    source_page::SourcePage,
};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PagePayload {
    pub id: Option<String>,
    pub language: Option<String>,
    // TODO: Rename rel_file_path to rel_dest_path;
    pub rel_file_path: Option<PathBuf>,
    pub rel_source_path: Option<PathBuf>,
    pub r#type: Option<String>,
    pub sections: Vec<PayloadSection>,
    // DEPRECATED: Remove source path and replace with rel_source_path
    pub source_path: Option<PathBuf>,
    pub status: Option<String>,
    pub template_list: Vec<String>,
    pub title: Vec<PayloadSpan>,
    pub used_template: Option<String>,
}

impl PagePayload {
    pub fn new_from_source_page(
        source_path: &PathBuf,
        source: &SourcePage,
    ) -> Result<PagePayload, NeoError> {
        let sections = source
            .ast
            .as_ref()
            .unwrap()
            .iter()
            .map(|section| {
                let p = PayloadSection::new_from_section(&section, source.config.as_ref().unwrap());
                p
            })
            .collect::<Vec<PayloadSection>>();
        let mut p = PagePayload {
            id: None,
            language: None,
            rel_source_path: None,
            r#type: Some("post".to_string()),
            rel_file_path: None,
            sections,
            source_path: Some(source_path.clone()),
            status: Some("published".to_string()),
            template_list: vec![],
            title: vec![], // TODO: Add title spans
            used_template: None,
        };

        p.get_id();

        match p.id {
            Some(_) => {
                p.get_language(&source);
                p.get_type();
                p.get_status();
                p.get_rel_source_path(&source);
                p.get_rel_file_path();
                Ok(p)
            }
            None => Err(NeoError {
                kind: NeoErrorKind::GenericErrorWithSourcePathAndPayloadSections {
                    source_path: source_path.clone(),
                    msg: "Could not get id when making page payload".to_string(),
                    sections: Some(p.sections.clone()),
                },
            }),
        }

        // if let Some(id) = source.id() {
        //     let mut p = PagePayload {
        //         id,
        //         r#type: None,
        //         rel_file_path: None,
        //         sections: vec![],
        //         status: None,
        //         template_list: vec![],
        //         title: vec![], // TODO: Add title spans
        //         used_template: None,
        //     };
        //     p.rel_file_path = source.rel_file_path();
        //     p.template_list = source.template_list();
        //     p.status = source.status();
        //     p.r#type = source.r#type();
        //     p.sections = source.sections();
        //     Ok(p)
        // } else {
        //     Err(NeoError {
        //         kind: NeoErrorKind::GenericErrorWithSourcePath {
        //             source_path: source.source_path.clone().expect("get source path"),
        //             msg: "could not get id for file".to_string(),
        //         },
        //     })
        // }
    }
}

impl PagePayload {
    pub fn get_id(&mut self) {
        self.sections.iter().for_each(|section| {
            if section.r#type == "metadata" {
                self.id = section.id.clone();
            }
        });
    }

    pub fn get_language(&mut self, source: &SourcePage) {
        self.language = Some(source.config.as_ref().unwrap().default_language.clone());
    }

    pub fn get_rel_source_path(&mut self, source: &SourcePage) {
        let sp = self.source_path.clone().unwrap();
        match sp.strip_prefix(source.config.as_ref().unwrap().content_source_dir()) {
            Ok(p) => self.rel_source_path = Some(p.to_path_buf()),
            Err(_) => {}
        }
    }

    pub fn get_rel_file_path(&mut self) {
        self.rel_file_path = Some(PathBuf::from(format!(
            "{}/{}/index.html",
            self.language.as_ref().unwrap(),
            self.id.as_ref().unwrap()
        )));
    }

    pub fn get_status(&mut self) {
        self.sections.iter().for_each(|section| {
            if section.r#type == "metadata" {
                match &section.status {
                    Some(s) => self.status = Some(s.to_string()),
                    None => {}
                }
            }
        });
    }

    pub fn get_type(&mut self) {
        self.sections.iter().for_each(|section| {
            if section.r#type == "metadata" {
                match &section.attrs {
                    Some(attrs) => {
                        attrs.iter().for_each(|(key, spans)| {
                            if key.eq("type") {
                                self.r#type = Some(flatten_payload_spans(&spans.clone()));
                            }
                        });
                    }
                    None => {}
                }
            }
        });
    }

    //
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::path::PathBuf;

    #[test]
    fn id_basic_check() {
        let p = PagePayload::new_from_source_page(
            &PathBuf::from("/test/mocks/source/filename.neo"),
            &SourcePage::mock1_20240101_alfa1234_minimal(),
        )
        .unwrap();
        let left = "20240101_alfa1234".to_string();
        let right = p.id.unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn language_basic_check() {
        let p = PagePayload::new_from_source_page(
            &PathBuf::from("/test/mocks/source/filename.neo"),
            &SourcePage::mock1_20240101_alfa1234_minimal(),
        )
        .unwrap();
        let left = "en".to_string();
        let right = p.language.unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn rel_file_path_default() {
        let p = PagePayload::new_from_source_page(
            &PathBuf::from("/test/mocks/source/filename.neo"),
            &SourcePage::mock1_20240101_alfa1234_minimal(),
        )
        .unwrap();
        let left = PathBuf::from("en/20240101_alfa1234/index.html");
        let right = p.rel_file_path.unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn rel_source_path_check() {
        let p = PagePayload::new_from_source_page(
            &PathBuf::from("/test/mocks/source/subdir/filename.neo"),
            &SourcePage::mock1_20240101_alfa1234_minimal(),
        )
        .unwrap();
        let left = PathBuf::from("subdir/filename.neo");
        let right = p.rel_source_path.unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn status_custom_check() {
        let p = PagePayload::new_from_source_page(
            &PathBuf::from("/test/mocks/source/filename.neo"),
            &SourcePage::mock4_20240104_delta123_type_and_status(),
        )
        .unwrap();
        let left = "custom-status".to_string();
        let right = p.status.unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn status_default_check() {
        let p = PagePayload::new_from_source_page(
            &PathBuf::from("/test/mocks/source/filename.neo"),
            &SourcePage::mock1_20240101_alfa1234_minimal(),
        )
        .unwrap();
        let left = "published".to_string();
        let right = p.status.unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn type_default_check() {
        let p = PagePayload::new_from_source_page(
            &PathBuf::from("/test/mocks/source/filename.neo"),
            &SourcePage::mock1_20240101_alfa1234_minimal(),
        )
        .unwrap();
        let left = "post".to_string();
        let right = p.r#type.unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn type_custom_check() {
        let p = PagePayload::new_from_source_page(
            &PathBuf::from("/test/mocks/source/filename.neo"),
            &SourcePage::mock4_20240104_delta123_type_and_status(),
        )
        .unwrap();
        let left = "custom-type".to_string();
        let right = p.r#type.unwrap();
        assert_eq!(left, right);
    }

    // #[test]
    // #[ignore]
    // fn add_page_tags_to_sections() {
    //     // TODO
    // }

    // #[test]
    // #[ignore]
    // fn add_section_tags_to_page() {
    //     // TODO
    // }

    // #[test]
    // #[ignore]
    // fn add_page_created_and_updated_to_sections() {
    //     // TODO
    // }

    // #[test]
    // #[ignore]
    // fn add_page_status_to_sections() {
    //     // TODO
    // }

    //
}

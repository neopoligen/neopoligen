use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tracing::{event, instrument, Level};

use crate::{
    helpers::*,
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
    /// theme_test_or_page
    /// the same builder is used for the main page output
    /// and the theme tests so that template work the same
    /// way. This flag is used to help ensure you know
    /// what part of the process things are in. This is
    /// specifically used to make an update for the
    /// rel_source_path based off the content dir
    pub theme_test_or_page: ThemeTestOrPage,
    pub title: Vec<PayloadSpan>,
    pub used_template: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum ThemeTestOrPage {
    ThemeTest,
    Page,
}

impl PagePayload {
    pub fn new_from_source_page(
        source_path: &PathBuf,
        source: &SourcePage,
        theme_test_or_page: ThemeTestOrPage,
    ) -> Result<PagePayload, NeoError> {
        match &source.ast {
            Some(_ast) => {
                let sections = source
                    .ast
                    .as_ref()
                    .unwrap()
                    .iter()
                    .map(|section| {
                        let p = PayloadSection::new_from_section(
                            &section,
                            source.config.as_ref().unwrap(),
                        );
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
                    theme_test_or_page,
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
                        p.get_template_list();
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
            }
            None => Err(NeoError {
                kind: NeoErrorKind::ForwardErrorWithSourcePath {
                    source_path: source_path.clone(),
                    msg: "No AST available".to_string(),
                },
            }),
        }
    }
}

impl PagePayload {
    pub fn get_id(&mut self) {
        self.sections.iter().for_each(|section| {
            if section.r#type == "metadata" {
                self.id = section.clone().id();
            }
        });
    }

    pub fn get_language(&mut self, source: &SourcePage) {
        self.language = Some(source.config.as_ref().unwrap().default_language.clone());
    }

    #[instrument(skip(self, source))]
    pub fn get_rel_source_path(&mut self, source: &SourcePage) {
        let sp = self.source_path.clone().unwrap();
        match self.theme_test_or_page {
            ThemeTestOrPage::Page => {
                match sp.strip_prefix(source.config.as_ref().unwrap().content_source_dir()) {
                    Ok(p) => self.rel_source_path = Some(p.to_path_buf()),
                    Err(e) => {
                        event!(Level::ERROR, "get_rel_source_path ERROR: {}", e);
                    }
                }
            }
            ThemeTestOrPage::ThemeTest => {
                match sp.strip_prefix(source.config.as_ref().unwrap().theme_dir()) {
                    Ok(p) => self.rel_source_path = Some(p.to_path_buf()),
                    Err(e) => {
                        event!(Level::ERROR, "get_rel_source_path ERROR: {}", e);
                    }
                }
            }
        }
    }

    pub fn get_rel_file_path(&mut self) {
        self.rel_file_path = Some(PathBuf::from(format!(
            "{}/{}/index.html",
            self.language.as_ref().unwrap(),
            self.id.as_ref().unwrap()
        )));
        // update again if there's a metadata path
        // TODO: Make this all happen in one go
        // at some point instead of two passes
        self.sections.iter().for_each(|section| {
            if section.r#type == "metadata" {
                let _ = &section.attr_spans.iter().for_each(|(key, spans)| {
                    if key.eq("path") {
                        self.rel_file_path = Some(
                            scrub_rel_file_path(&flatten_payload_spans(&spans.clone()))
                                .expect("get filepath"),
                        );
                    }
                });
            }
        });
        // Now override if there's a `full-path`.
        self.sections.iter().for_each(|section| {
            if section.r#type == "metadata" {
                let _ = &section.attr_spans.iter().for_each(|(key, spans)| {
                    if key.eq("full-path") {
                        let the_path = &flatten_payload_spans(&spans.clone());
                        self.rel_file_path = Some(the_path.trim_start_matches("/").into());
                    }
                });
            }
        });
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

    pub fn get_template_list(&mut self) {
        if let (Some(t), Some(s)) = (self.r#type.clone(), self.status.clone()) {
            let new_path = format!("pages/{}/{}.neoj", t, s);
            if !self.template_list.contains(&new_path) {
                self.template_list.push(new_path);
            }
        }
        if let Some(t) = self.r#type.clone() {
            let new_path = format!("pages/{}/published.neoj", t);
            if !self.template_list.contains(&new_path) {
                self.template_list.push(new_path);
            }
        }
        if let Some(s) = self.status.clone() {
            let new_path = format!("pages/post/{}.neoj", s);
            if !self.template_list.contains(&new_path) {
                self.template_list.push(new_path);
            }
        }
        let new_path = format!("pages/post/published.neoj");
        if !self.template_list.contains(&new_path) {
            self.template_list.push(new_path);
        }
    }

    pub fn get_type(&mut self) {
        self.sections.iter().for_each(|section| {
            if section.r#type == "metadata" {
                let _ = &section.attr_spans.iter().for_each(|(key, spans)| {
                    if key.eq("type") {
                        self.r#type = Some(flatten_payload_spans(&spans.clone()));
                    }
                });
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
            ThemeTestOrPage::Page,
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
            ThemeTestOrPage::Page,
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
            ThemeTestOrPage::Page,
        )
        .unwrap();
        let left = PathBuf::from("en/20240101_alfa1234/index.html");
        let right = p.rel_file_path.unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn rel_file_path_home_page() {
        let p = PagePayload::new_from_source_page(
            &PathBuf::from("/test/mocks/source/filename.neo"),
            &SourcePage::mock2_20240102_bravo123_home_page_path(),
            ThemeTestOrPage::Page,
        )
        .unwrap();
        let left = PathBuf::from("index.html");
        let right = p.rel_file_path.unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn rel_file_full_path() {
        let p = PagePayload::new_from_source_page(
            &PathBuf::from("/test/mocks/source/filename.neo"),
            &SourcePage::mock6_20240106_foxtrot8_full_path(),
            ThemeTestOrPage::Page,
        )
        .unwrap();
        let left = PathBuf::from("CNAME");
        let right = p.rel_file_path.unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn rel_source_path_check_for_page() {
        let p = PagePayload::new_from_source_page(
            &PathBuf::from("/test/mocks/content/subdir/filename.neo"),
            &SourcePage::mock1_20240101_alfa1234_minimal(),
            ThemeTestOrPage::Page,
        )
        .unwrap();
        let left = PathBuf::from("subdir/filename.neo");
        let right = p.rel_source_path.unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn rel_source_path_check_for_theme_test() {
        let p = PagePayload::new_from_source_page(
            &PathBuf::from("/test/mocks/themes/test-theme/templates/filename.neo"),
            &SourcePage::mock1_20240101_alfa1234_minimal(),
            ThemeTestOrPage::ThemeTest,
        )
        .unwrap();
        let left = PathBuf::from("templates/filename.neo");
        let right = p.rel_source_path.unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn status_custom_check() {
        let p = PagePayload::new_from_source_page(
            &PathBuf::from("/test/mocks/source/filename.neo"),
            &SourcePage::mock4_20240104_delta123_type_and_status(),
            ThemeTestOrPage::Page,
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
            ThemeTestOrPage::Page,
        )
        .unwrap();
        let left = "published".to_string();
        let right = p.status.unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn template_list_default_check() {
        let p = PagePayload::new_from_source_page(
            &PathBuf::from("/test/mocks/source/filename.neo"),
            &SourcePage::mock1_20240101_alfa1234_minimal(),
            ThemeTestOrPage::Page,
        )
        .unwrap();
        let left = vec!["pages/post/published.neoj".to_string()];
        let right = p.template_list;
        assert_eq!(left, right);
    }

    #[test]
    fn solo_template_list_with_type_and_status() {
        let p = PagePayload::new_from_source_page(
            &PathBuf::from("/test/mocks/source/filename.neo"),
            &SourcePage::mock4_20240104_delta123_type_and_status(),
            ThemeTestOrPage::Page,
        )
        .unwrap();
        let left = vec![
            "pages/custom-type/custom-status.neoj".to_string(),
            "pages/custom-type/published.neoj".to_string(),
            "pages/post/custom-status.neoj".to_string(),
            "pages/post/published.neoj".to_string(),
        ];
        let right = p.template_list;
        assert_eq!(left, right);
    }

    #[test]
    fn type_default_check() {
        let p = PagePayload::new_from_source_page(
            &PathBuf::from("/test/mocks/source/filename.neo"),
            &SourcePage::mock1_20240101_alfa1234_minimal(),
            ThemeTestOrPage::Page,
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
            ThemeTestOrPage::Page,
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

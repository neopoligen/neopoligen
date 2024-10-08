use html_escape::*;
use minijinja::value::Object;
use minijinja::value::Value;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use tracing::{event, instrument, Level};

use crate::{
    helpers::*,
    neo_error::{NeoError, NeoErrorKind},
    payload_section::PayloadSection,
    // payload_span::PayloadSpan,
    source_page::SourcePage,
};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PagePayload {
    pub absolute_url: Option<String>,
    pub flags: Vec<String>,
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
    pub title: Option<String>,
    pub used_template: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum ThemeTestOrPage {
    ThemeTest,
    Page,
}

// This is an attempt to send the values as an object
// instead of having to serialize it.
// TODO: See if you can get this to work:
// https://docs.rs/minijinja/latest/minijinja/value/trait.Object.html
impl Object for PagePayload {
    fn get_value(self: &Arc<Self>, field: &Value) -> Option<Value> {
        match field.as_str()? {
            "title" => Some(Value::from(self.get_title_v2())),
            // TODO: Add all the rest of the stuff here
            _ => None,
        }
    }
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
                    absolute_url: None,
                    flags: vec![],
                    id: None,
                    language: None,
                    r#type: Some("post".to_string()),
                    rel_file_path: None,
                    rel_source_path: None,
                    sections,
                    source_path: Some(source_path.clone()),
                    status: Some("published".to_string()),
                    theme_test_or_page,
                    template_list: vec![],
                    title: None,
                    used_template: None,
                };
                p.get_id();
                match p.id {
                    Some(_) => {
                        p.get_flags();
                        p.get_language(&source);
                        p.get_type();
                        p.get_status();
                        p.get_template_list();
                        p.get_rel_source_path(&source);
                        p.get_rel_file_path();
                        p.get_title();
                        p.get_rel_url_path();
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

        // // update again if there's a metadata path
        // // TODO: Make this all happen in one go
        // // at some point instead of two passes
        if let Some((_, spans)) = self
            .sections
            .iter()
            .rfind(|section| section.r#type == "metadata")
            .and_then(|sec| sec.attr_spans.iter().find(|attr| *attr.0 == "path"))
        {
            self.rel_file_path = Some(
                scrub_rel_file_path(&flatten_payload_spans(&spans.clone())).expect("get filepath"),
            )
        }

        // Now override if there's a `full-path`.
        // TODO: Needs test
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

    // TODO: If you can't make a url_path, throw an error
    // TODO: Set up for path override
    // TODO: Set up for home page
    pub fn get_rel_url_path(&mut self) {
        if let (Some(lang), Some(id), Some(title)) =
            (self.language.clone(), self.id.clone(), self.title.clone())
        {
            self.absolute_url = Some(format!(
                "/{}/{}/?page={}",
                lang,
                id,
                url_from_string(&title)
            ));
        }
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

    pub fn get_title(&mut self) {
        self.sections.iter().for_each(|section| {
            if section.r#type == "title" {
                match section.children.first() {
                    Some(stuff) => {
                        self.title = Some(
                            encode_safe(&flatten_payload_spans(&stuff.spans.clone())).to_string(),
                        )
                    }
                    _ => (),
                }
            }
        })
    }

    pub fn get_title_v2(&self) -> Option<Value> {
        self.sections.iter().find_map(|section| {
            if section.r#type == "title" {
                match section.children.first() {
                    Some(stuff) => Some(Value::from(
                        encode_safe(&flatten_payload_spans(&stuff.spans.clone())).to_string(),
                    )),
                    _ => None,
                }
            } else {
                None
            }
        })
    }

    pub fn get_flags(&mut self) {
        self.sections.iter().for_each(|section| {
            if section.r#type == "metadata" {
                let _ = &section
                    .flags
                    .iter()
                    .for_each(|flag| self.flags.push(flag.clone()));
            }
        });
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
    fn absolute_url_check() {
        let p = PagePayload::new_from_source_page(
            &PathBuf::from("/test/mocks/source/filename.neo"),
            &SourcePage::mock1_20240101_alfa1234_minimal(),
            ThemeTestOrPage::Page,
        )
        .unwrap();
        let left = "/en/20240101_alfa1234/?page=alfa-alfa-alfa".to_string();
        let right = p.absolute_url.unwrap();
        assert_eq!(left, right);
    }

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

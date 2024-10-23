pub mod mocks;

use crate::ast::parse_ast;
use crate::helpers::flatten_spans;
use crate::neo_error::NeoError;
use crate::neo_error::NeoErrorKind;
use crate::section::Section;
use crate::section_attr::SectionAttr;
use crate::section_attr::SectionAttrKind;
use crate::site_config::SiteConfigV42;
use minijinja::value::Object;
use minijinja::Error;
use minijinja::Value;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PageV42 {
    pub ast: Option<Vec<Section>>,
    pub raw_source: String,
    pub fs_path: PathBuf,
}

impl PageV42 {
    pub fn new_from_fs_path(path: &PathBuf) -> Result<PageV42, NeoError> {
        match fs::read_to_string(path) {
            Ok(raw_source) => Ok(PageV42 {
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

impl PageV42 {
    pub fn ast(&self) -> Result<Value, Error> {
        Ok(Value::from_serialize(&self.ast))
    }

    pub fn generate_ast(&mut self, site_config: &SiteConfigV42) -> Result<(), NeoError> {
        match parse_ast(&self.raw_source, site_config.sections.clone()) {
            Ok(ast) => {
                self.ast = Some(ast);
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub fn get_pagedata_attr(&self, attr_key: &str) -> Result<String, NeoError> {
        match &self.ast {
            Some(sections) => {
                let tmp = sections
                    .iter()
                    .find_map(|section| match section.r#type.as_str() {
                        "pagedata" => Some(section.attrs.iter().find_map(|attr| match attr {
                            SectionAttr { kind, .. } => match kind {
                                SectionAttrKind::KeyValueSpans { key, spans } => {
                                    if key.as_str() == attr_key {
                                        Some(Ok::<String, NeoError>(flatten_spans(spans)))
                                    } else {
                                        None
                                    }
                                }
                                _ => None,
                            },
                        })),
                        _ => None,
                    });
                match tmp {
                    Some(status) => match status {
                        Some(s) => Ok(s?),
                        None => Err(NeoError {
                            kind: NeoErrorKind::GenericErrorWithSourcePath {
                                source_path: self.fs_path.clone(),
                                msg: format!("Could not get attr from pagedata: {}", attr_key),
                            },
                        }),
                    },
                    None => Err(NeoError {
                        kind: NeoErrorKind::GenericErrorWithSourcePath {
                            source_path: self.fs_path.clone(),
                            msg: format!("Could not get attr from pagedata: {}", attr_key),
                        },
                    }),
                }
            }
            None => Err(NeoError {
                kind: NeoErrorKind::GenericErrorWithSourcePath {
                    source_path: self.fs_path.clone(),
                    msg: format!("Could not get attr from pagedata: {}", attr_key),
                },
            }),
        }
    }

    pub fn id(&self) -> Result<String, NeoError> {
        match &self.ast {
            Some(sections) => {
                let tmp = sections
                    .iter()
                    .find_map(|section| match section.r#type.as_str() {
                        "pagedata" => Some(section.attrs.iter().find_map(|attr| match attr {
                            SectionAttr { kind, .. } => match kind {
                                SectionAttrKind::KeyValueSpans { key, spans } => match key.as_str()
                                {
                                    "id" => Some(Ok::<String, NeoError>(flatten_spans(spans))),
                                    _ => None,
                                },
                                _ => None,
                            },
                        })),
                        _ => None,
                    });
                match tmp {
                    Some(status) => match status {
                        Some(s) => Ok(s?),
                        None => Err(NeoError {
                            kind: NeoErrorKind::GenericErrorWithSourcePath {
                                source_path: self.fs_path.clone(),
                                msg: "could not get status for file".to_string(),
                            },
                        }),
                    },
                    None => Err(NeoError {
                        kind: NeoErrorKind::GenericErrorWithSourcePath {
                            source_path: self.fs_path.clone(),
                            msg: "Could not get status from pagedata".to_string(),
                        },
                    }),
                }
            }
            None => Err(NeoError {
                kind: NeoErrorKind::GenericErrorWithSourcePath {
                    source_path: self.fs_path.clone(),
                    msg: "Could not get id from pagedata".to_string(),
                },
            }),
        }
    }

    pub fn output_fs_rel_path(&self, base_rel_path: &PathBuf) -> Result<PathBuf, NeoError> {
        // TODO: handle override paths from metadata
        if let Ok(mut path_override) = self.get_pagedata_attr("path") {
            path_override = path_override.strip_prefix("/").unwrap().to_string();
            Ok(PathBuf::from(path_override))
        } else {
            match self.id() {
                Ok(id) => {
                    let mut path = base_rel_path.clone();
                    path = path.join(id).join("index.html");
                    Ok(path)
                }
                _ => Err(NeoError {
                    kind: NeoErrorKind::GenericErrorWithSourcePath {
                        source_path: self.fs_path.clone(),
                        msg: "Could not find id to make output path".to_string(),
                    },
                }),
            }
        }
    }

    pub fn sections(&self) -> Result<Value, Error> {
        Ok(Value::from_serialize(
            self.ast
                .clone()
                .unwrap()
                .iter()
                .map(|s| Value::from_object(s.clone()))
                .collect::<Value>(),
        ))
    }

    // NOTE: This feels really hacky. TODO is look at cleaning
    // it up at some point.
    pub fn status(&self) -> Result<String, NeoError> {
        match &self.ast {
            Some(sections) => {
                let tmp = sections
                    .iter()
                    .find_map(|section| match section.r#type.as_str() {
                        "pagedata" => Some(section.attrs.iter().find_map(|attr| match attr {
                            SectionAttr { kind, .. } => match kind {
                                SectionAttrKind::KeyValueSpans { key, spans } => match key.as_str()
                                {
                                    "status" => Some(Ok::<String, NeoError>(
                                        flatten_spans(spans).trim().to_string(),
                                    )),
                                    _ => None,
                                },
                                _ => None,
                            },
                        })),
                        _ => None,
                    });
                match tmp {
                    Some(status) => match status {
                        Some(s) => Ok(s?),
                        None => Err(NeoError {
                            kind: NeoErrorKind::GenericErrorWithSourcePath {
                                source_path: self.fs_path.clone(),
                                msg: "could not get status for file".to_string(),
                            },
                        }),
                    },
                    None => Err(NeoError {
                        kind: NeoErrorKind::GenericErrorWithSourcePath {
                            source_path: self.fs_path.clone(),
                            msg: "Could not get status from pagedata".to_string(),
                        },
                    }),
                }
            }
            None => Err(NeoError {
                kind: NeoErrorKind::GenericErrorWithSourcePath {
                    source_path: self.fs_path.clone(),
                    msg: "Could not get id from pagedata".to_string(),
                },
            }),
        }
    }

    pub fn template(&self) -> Result<String, NeoError> {
        self.get_pagedata_attr("template")
    }

    pub fn template_list(&self) -> Vec<String> {
        if let (Ok(template), Ok(status)) = (self.template(), self.status()) {
            vec![
                format!("page-templates/{}/{}.neoj", template, status),
                format!("page-templates/{}/done.neoj", template),
                format!("page-templates/post/{}.neoj", status),
                format!("page-templates/post/done.neoj"),
            ]
        } else if let Ok(status) = self.status() {
            vec![
                format!("page-templates/post/{}.neoj", status),
                format!("page-templates/post/done.neoj"),
            ]
        } else {
            vec![]
        }
    }
}

impl Object for PageV42 {
    fn call_method(
        self: &Arc<PageV42>,
        _state: &minijinja::State,
        name: &str,
        _args: &[Value],
    ) -> Result<Value, Error> {
        match name {
            "ast" => self.ast(),
            "id" => Ok(Value::from(self.id().unwrap())),
            "sections" => self.sections(),
            _ => Ok(Value::from("")),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn id_test_get_basic_id() {
        let page = PageV42::mock1_basic_page();
        //dbg!(&page.ast());
        let left = "mock1_basic_page".to_string();
        let right = page.id().unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn id_should_error_if_missing() {
        let page = PageV42::mock2_missing_id_and_status();
        assert!(page.id().is_err());
    }

    #[test]
    fn output_rel_path_test_mock1() {
        let page = PageV42::mock1_basic_page();
        let left = PathBuf::from("en/mock1_basic_page/index.html");
        let right = page.output_fs_rel_path(&PathBuf::from("en")).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn solo_output_rel_path_with_path_attr() {
        let page = PageV42::mock3_custom_path();
        let left = PathBuf::from("index.html");
        let right = page.output_fs_rel_path(&PathBuf::from("en")).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn status_test_get_basic_status() {
        let page = PageV42::mock1_basic_page();
        let left = "scratch".to_string();
        let right = page.status().unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn status_should_error_if_missing() {
        let page = PageV42::mock2_missing_id_and_status();
        assert!(page.status().is_err());
    }

    #[test]
    fn template_list_with_found_template_and_status() {
        let page = PageV42::mock3_custom_path();
        let left = vec![
            "page-templates/home-page/draft.neoj".to_string(),
            "page-templates/home-page/done.neoj".to_string(),
            "page-templates/post/draft.neoj".to_string(),
            "page-templates/post/done.neoj".to_string(),
        ];
        let right = page.template_list();
        assert_eq!(left, right);
    }

    #[test]
    fn template_list_without_custom_template_attr() {
        let page = PageV42::mock1_basic_page();
        let left = vec![
            "page-templates/post/scratch.neoj".to_string(),
            "page-templates/post/done.neoj".to_string(),
        ];
        let right = page.template_list();
        assert_eq!(left, right);
    }

    //
}

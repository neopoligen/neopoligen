pub mod mocks;

use crate::ast::parse_ast;
use crate::helpers::flatten_spans;
use crate::neo_error::NeoError;
use crate::neo_error::NeoErrorKind;
use crate::section::Section;
use crate::section_attr::SectionAttr;
use crate::section_attr::SectionAttrKind;
use crate::site_config::SiteConfig;
use minijinja::value::Object;
use minijinja::Error;
use minijinja::Value;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PageV41 {
    pub ast: Option<Vec<Section>>,
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

impl PageV41 {
    pub fn dev_ping(&self, args: &[Value]) -> Value {
        Value::from(format!("ALFA THIS IS PAGE DEV PING: {}", args[0]))
    }

    pub fn dev_ping_page(&self, args: &[Value]) -> Result<Value, Error> {
        Ok(Value::from(format!(
            "BRAVO PAGE PING TEST PING: {}",
            args[0]
        )))
    }

    pub fn generate_ast(&mut self, site_config: &SiteConfig) -> Result<(), NeoError> {
        match parse_ast(&self.raw_source, site_config.sections.clone()) {
            Ok(ast) => {
                self.ast = Some(ast);
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    // TODO: Make return Value
    pub fn id(&self) -> Result<String, NeoError> {
        Ok("dev-alfa".to_string())
    }

    pub fn id_raw(&self) -> Result<String, NeoError> {
        match &self.ast {
            Some(sections) => {
                sections
                    .iter()
                    .for_each(|section| match section.r#type.as_str() {
                        "pagedata" => {
                            section.attrs.iter().for_each(|attr| match attr {
                                SectionAttr { kind, .. } => match kind {
                                    SectionAttrKind::KeyValueSpans { key, spans } => {
                                        match key.as_str() {
                                            "id" => {
                                                let x = flatten_spans(spans);
                                                dbg!(x);
                                                ()
                                            }
                                            _ => (),
                                        }
                                        ()
                                    }
                                    _ => (),
                                },
                            });
                            ()
                        }
                        _ => (),
                    });
                Ok("mock1_basic".to_string())
            }
            None => Err(NeoError {
                kind: NeoErrorKind::GenericErrorWithoutSourcePath {
                    msg: "could not get id".to_string(),
                },
            }),
        }
        // Ok("mock1_basic".to_string())
    }

    pub fn output_fs_rel_path(&self, base_rel_path: &PathBuf) -> PathBuf {
        // TODO: handle override paths from metadata
        let mut path = base_rel_path.clone();
        path = path.join(self.id().unwrap());
        path = path.join("index.html");
        path
    }

    // pub fn sections(&self) -> Value {}
    pub fn template_list(&self) -> Vec<String> {
        vec!["dev-test".to_string()]
    }
}

impl Object for PageV41 {
    fn call_method(
        self: &Arc<PageV41>,
        _state: &minijinja::State,
        name: &str,
        args: &[Value],
    ) -> Result<Value, Error> {
        match name {
            "dev_ping_page" => self.dev_ping_page(args),
            _ => Ok(Value::from("")),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn solo_id_test_get_basic_id() {
        let page = PageV41::mock1_basic_page();
        let left = "mock1_basic".to_string();
        let right = page.id_raw().unwrap();
        assert_eq!(left, right);
    }
}

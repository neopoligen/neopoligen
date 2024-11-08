#![allow(unused)]
pub mod helpers;
pub mod inits;
pub mod mocks;

use crate::ast_v42::parse_ast_v42;
use crate::helpers::flatten_spans_v42;
use crate::neo_error_v42::NeoErrorV42;
use crate::page_v42::helpers::*;
use crate::section_key_value_attr_payload_v42::SectionKeyValueAttrPayloadV42;
use crate::section_v42::SectionV42;
use crate::site_config::SiteConfigV42;
use crate::span_v42::plaintext::PlainTextSpan;
use crate::span_v42::SpanV42;
use minijinja::value::Object;
use minijinja::Error;
use minijinja::Value;
use nom_supreme::error::GenericErrorTree;
use nom_supreme::final_parser::Location;
use nom_supreme::final_parser::RecreateContext;
use serde::Deserialize;
use serde::Serialize;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PageV42b {
    pub ast: Option<Vec<SectionV42>>,
    pub parsing_issue: Option<NeoErrorV42>,
    // TODO: Deprecate .href field in favor of .href() method
    pub href: Option<PathBuf>,
    pub id: Option<String>,
    pub issues: Vec<NeoErrorV42>,
    pub language: Option<String>,
    pub output: Option<String>,
    pub rel_output_path: Option<PathBuf>,
    pub sections: Option<Value>,
    pub source: String,
    pub source_absolute_path: PathBuf,
    pub source_display_path: PathBuf,
}

impl PageV42b {
    pub fn generate_ast(&mut self, site_config: &SiteConfigV42) {
        match parse_ast_v42(&self.source, site_config, false) {
            Ok(ast) => {
                self.sections = Some(Value::from_serialize(&ast));
                self.ast = Some(ast);
            }
            Err(e) => {
                match e {
                    GenericErrorTree::Base { location, kind } => {
                        let details = Location::recreate_context(self.source.as_str(), location);
                        self.parsing_issue = Some(NeoErrorV42::ParsingIssue {
                            column: details.column,
                            line: details.line,
                            source: self.source.as_str().to_string(),
                            completed: self.source.trim_end_matches(location).to_string(),
                            remainder: location.to_string(),
                            message: kind.to_string(),
                            template_list: vec![
                                "issues/parsing-issue/default.neoj".to_string(),
                                "built-in/issues/parsing-issue/default.neoj".to_string(),
                            ],
                        });
                    }
                    _ => {
                        // NOTE: The rest of the errors are for the
                        // tree. I'm not using them right now because it's a lot
                        // of info that mostly is overwhelming. The top level
                        // BASE error above has been what I've need every
                        // time so far. So, sticking with just it.
                    }
                };
            }
        }
    }

    // // TODO: Update to be rel_output_path
    // pub fn generate_fallback_href(&mut self) {
    //     // TODO: Don't create the href if there's no ID
    //     // otherwise you won't be able to link to the page properly
    //     if let None = &self.href {
    //         match get_value_from_raw_page_section_parsing(&self.source, "href") {
    //             Ok(v) => {
    //                 self.href = Some(v.1.join("").trim().to_string().into());
    //                 let _ = &self
    //                     .issues
    //                     .push(NeoErrorV42::new_page_issue(&self, vec![
    //                         "Warning: This pages is using a fallback href because one couldn't be found properly in the source file.".to_string(),
    //                         "If this page is showing up in an expected location that's probably why. Fixing whatever issues are happening in the source file should fix this as well.".to_string(),
    //                     ]));
    //             }
    //             Err(e) => {
    //                 let _ = &self.issues.push(NeoErrorV42::new_page_issue(
    //                     &self,
    //                     vec!["Error: Could not get fallback href from page".to_string()],
    //                 ));
    //             }
    //         }
    //     }
    // }

    // pub fn generate_fallback_id(&mut self) {
    //     if let None = &self.id {
    //         match get_value_from_raw_page_section_parsing(&self.source, "id") {
    //             Ok(v) => {
    //                 self.id = Some(v.1.join("").trim().to_string());
    //                 let _ = &self
    //                     .issues
    //                     .push(NeoErrorV42::new_page_issue(&self, vec![
    //                         "Warning: This pages is using a fallback ID because one couldn't be found properly in the source file.".to_string(),
    //                         "If this page is showing up in an expected location that's probably why. Fixing whatever issues are happening in the source file should fix this as well.".to_string(),
    //                     ]));
    //             }
    //             Err(e) => {
    //                 dbg!("TODO: Add this error to the page stack");
    //                 dbg!(e);
    //                 ()
    //             }
    //         }
    //     }
    // }

    // // TODO: Update to be rel_output_path
    // pub fn generate_href_from_id(&mut self) {
    //     if let Some(id) = &self.id {
    //         self.href = Some(PathBuf::from(format!("/en/{}/index.html", id)));
    //     }
    // }

    // // TODO: Update to be rel_output_path
    // pub fn generate_href_from_ast(&mut self) {
    //     if let Ok(attr) = self.get_page_attr_from_ast("href") {
    //         self.href = Some(PathBuf::from(&attr.text));
    //     }
    // }

    // pub fn generate_rel_output_path(&mut self) {
    //     if let Some(id) = &self.id {
    //         self.rel_output_path = Some(PathBuf::from(format!("en/{}/index.html", id)));
    //     }
    // }

    // pub fn generate_id_from_ast(&mut self) {
    //     match self.get_page_attr_from_ast("id") {
    //         Ok(attr) => {
    //             self.id = Some(attr.text.to_string());
    //         }
    //         Err(e) => {
    //             let _ = &self
    //                     .issues
    //                     .push(NeoErrorV42::new_page_issue(&self, vec![
    //                         "Could not get id from parsed filed".to_string(),
    //                         "This could mean that there is no '-- id: ...' in the '-- page' section of the file.".to_string(),
    //                         "It could also mean that no '-- page' section was found in the file.".to_string()
    //                     ]));
    //         }
    //     }
    // }

    pub fn get_page_attr_from_ast(
        &self,
        key: &str,
    ) -> Result<&SectionKeyValueAttrPayloadV42, NeoErrorV42> {
        match &self.ast {
            Some(ast) => {
                match ast
                    .iter()
                    .filter_map(|section| match section {
                        SectionV42::Json5(sec) => Some(sec),
                        _ => None,
                    })
                    .last()
                {
                    Some(s) => match s.attrs.get(key) {
                        Some(attr) => Ok(attr),
                        None => Err(NeoErrorV42::NoParsedPageDataForKey(key.to_string())),
                    },
                    None => Err(NeoErrorV42::NoParsedPageDataForKey(key.to_string())),
                }
            }
            None => Err(NeoErrorV42::NoParsedPageDataForKey(key.to_string())),
        }
    }

    pub fn set_id(&mut self) {
        match self.get_page_attr_from_ast("id") {
            Ok(attr) => self.id = Some(attr.text.to_string()),
            Err(_) => match get_value_from_raw_page_section_parsing(&self.source, "id") {
                Ok(v) => {
                    self.id = Some(v.1.join("").trim().to_string());
                    let _ = &self
                        .issues
                        .push(NeoErrorV42::new_page_issue(&self, vec![
                            "Warning: This pages is using a fallback ID because one couldn't be found properly in the source file.".to_string(),
                            "If this page is showing up in an expected location that's probably why. Fixing whatever issues are happening in the source file should fix this as well.".to_string(),
                        ]));
                }
                Err(e) => {
                    let _ = &self.issues.push(NeoErrorV42::new_page_issue(
                        &self,
                        vec!["Could not get an ID for the page".to_string()],
                    ));
                }
            },
        }
    }

    pub fn set_language_from_site_config(&mut self, site_config: &SiteConfigV42) {
        self.language = Some(site_config.default_language.clone());
    }

    pub fn set_rel_output_path(&mut self) {
        match &self.get_page_attr_from_ast("path") {
            Ok(attr) => {
                let path = PathBuf::from(attr.text.to_string());
                let path_stripped = match path.strip_prefix("/") {
                    Ok(p) => p.into(),
                    Err(_) => path.clone(),
                };
                match &path.extension() {
                    Some(_) => self.rel_output_path = Some(path_stripped),
                    None => self.rel_output_path = Some(path_stripped.join("index.html")),
                }
            }
            Err(_) => {
                if let (Some(language), Some(id)) = (&self.language, &self.id) {
                    self.rel_output_path =
                        Some(PathBuf::from(format!("{}/{}/index.html", language, id)));
                }
            }
        };
    }

    pub fn title(&self) -> Result<Value, Error> {
        // ORDER:
        // pagedata--title
        // -- title section first block
        // --section--title attr on first block if it has one
        // first number of n words from first block
        // pagedata--id
        if let Some(ast) = &self.ast.as_ref() {
            if let Some(page_title) = match &self
                .ast
                .as_ref()
                .unwrap()
                .iter()
                .filter_map(|section| match section {
                    SectionV42::Json5(s) => Some(s),
                    _ => None,
                })
                .find_map(|section| match section.kind.as_str() {
                    "page" => {
                        //dbg!(&section.attrs);
                        Some(&section.attrs)
                    }
                    _ => None,
                }) {
                Some(attrs) => match attrs.get("title") {
                    Some(attr) => Some(Ok(Value::from_serialize(attr.spans.clone()))),
                    None => None,
                },
                None => None,
            } {
                page_title
            } else if let Some(spans) =
                &self
                    .ast
                    .as_ref()
                    .unwrap()
                    .iter()
                    .find_map(|section| match section {
                        SectionV42::Basic(sec) => match sec.kind.as_str() {
                            "title" => sec.children.iter().find_map(|child| match child {
                                SectionV42::Block(c) => match c.kind.as_str() {
                                    "block-paragraph" => Some(c.spans.clone()),
                                    _ => None,
                                },
                                _ => None,
                            }),
                            _ => None,
                        },
                        _ => None,
                    })
            {
                //dbg!(spans);
                Ok(Value::from_serialize(spans))
            } else if let Some(id) = &self.id.clone() {
                Ok(Value::from_serialize(vec![SpanV42::PlainText(
                    PlainTextSpan { text: id.clone() },
                )]))
            } else {
                Ok(Value::from("[NEO_TODO_PAGE_TITLE]"))
            }
        } else {
            Err(Error::new(
                minijinja::ErrorKind::InvalidOperation,
                "no ast to build title from",
            ))
        }
    }

    //
}

impl Object for PageV42b {
    fn get_value(self: &Arc<Self>, key: &Value) -> Option<Value> {
        match key.as_str()? {
            "id" => Some(Value::from(self.id.clone().unwrap().clone())),
            "issues" => Some(Value::from_serialize(self.issues.clone())),
            "parsing_issue" => Some(Value::from_serialize(self.parsing_issue.clone())),
            "sections" => self.sections.clone(),
            "source" => Some(Value::from(self.source.clone())),
            "source_display_path" => {
                Some(Value::from(self.source_display_path.display().to_string()))
            }
            _ => None,
        }
    }

    fn call_method(
        self: &Arc<PageV42b>,
        state: &minijinja::State,
        name: &str,
        args: &[Value],
    ) -> Result<Value, Error> {
        match name {
            "title_flattened" => Ok(Value::from("title flattened")),
            // "sections" => self.ast(),
            // "get_pagedata" => self.make_value_from_string(self.get_pagedata(args)),
            // "sections" => self.sections(),
            //"title" => self.title(),
            _ => Ok(Value::from("")),
        }
    }
}

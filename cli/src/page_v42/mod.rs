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
    // TODO: Deprecate .href() method in favor of
    // of this .href field which should be prepopulated
    // TODO: Make .href a String
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
    pub title: Option<String>,
    pub title_as_spans: Option<Vec<SpanV42>>,
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

    // TODO: Check if this works
    pub fn get_attr_from_last_section_of_type(
        &self,
        section_kind: &str,
        key: &str,
    ) -> Result<&SectionKeyValueAttrPayloadV42, NeoErrorV42> {
        match &self.ast {
            Some(ast) => {
                match ast
                    .iter()
                    .filter_map(|section| match section {
                        SectionV42::Basic(sec) => {
                            if sec.kind == section_kind {
                                Some(sec)
                            } else {
                                None
                            }
                        }
                        _ => None,
                    })
                    .last()
                {
                    Some(s) => match s.attrs.get(key) {
                        Some(attr) => Ok(attr),
                        // TODO: Update this error to be generic for section
                        None => Err(NeoErrorV42::NoParsedPageDataForKey(key.to_string())),
                    },
                    // TODO: Update this error to be generic for section
                    None => Err(NeoErrorV42::NoParsedPageDataForKey(key.to_string())),
                }
            }
            // TODO: Update this error to be generic for section
            None => Err(NeoErrorV42::NoParsedPageDataForKey(key.to_string())),
        }
    }

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

    pub fn href(&self) -> Result<Value, Error> {
        match &self.rel_output_path {
            Some(path) => match &path.file_name() {
                Some(file_name) => {
                    if file_name.to_str() == Some("index.html") {
                        match &path.parent() {
                            Some(p) => {
                                if p.to_str() == Some("") {
                                    Ok(format!("/").into())
                                } else {
                                    Ok(format!("/{}/", path.parent().unwrap().display()).into())
                                }
                            }
                            None => Ok(format!("/{}/", path.parent().unwrap().display()).into()),
                        }
                    } else {
                        Ok(format!("/{}", path.display()).into())
                    }
                }
                None => Ok(format!("/{}", path.display()).into()),
            },
            None => Err(Error::new(
                minijinja::ErrorKind::InvalidOperation,
                "could not get href",
            )),
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
                Err(_e) => {
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

    pub fn set_titles(&mut self) {
        self.title_as_spans = match &self.get_page_attr_from_ast("title") {
            Ok(attr) => Some(attr.spans.clone()),
            Err(_) => {
                match &self.ast {
                    Some(ast) => {
                        if let Some(t) = ast.iter().find_map(|section| match section {
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
                        }) {
                            Some(t)
                        } else {
                            // TODO: Hoist Error
                            None
                        }
                    }
                    None => {
                        // TODO: Hoist Error
                        None
                    }
                }
            }
        };
        if let Some(spans) = &self.title_as_spans {
            self.title = Some(flatten_spans_v42(&spans));
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
            "title" => Some(Value::from(self.title.clone())),
            "title_as_spans" => Some(Value::from_serialize(self.title_as_spans.clone())),
            _ => None,
        }
    }

    fn call_method(
        self: &Arc<PageV42b>,
        _state: &minijinja::State,
        name: &str,
        _args: &[Value],
    ) -> Result<Value, Error> {
        match name {
            "href" => self.href(),
            // "sections" => self.ast(),
            // "get_pagedata" => self.make_value_from_string(self.get_pagedata(args)),
            // "sections" => self.sections(),
            _ => Ok(Value::from(format!("Could not get page.{}", name))),
        }
    }

    //
}

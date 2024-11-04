#![allow(unused)]
pub mod helpers;
pub mod inits;
pub mod mocks;

use crate::ast_v42::parse_ast_v42;
use crate::helpers::flatten_spans_v42;
// use crate::helpers::get_minijinja_error_text;
// use crate::neo_error::NeoError;
use crate::neo_error_v42::NeoErrorV42;
use crate::page_v42::helpers::*;
use crate::section_v42::SectionV42;
use crate::site_config::SiteConfigV42;
// use crate::span_v42::plaintext::PlainTextSpan;
use crate::span_v42::SpanV42;
use minijinja::value::Object;
// use minijinja::Error;
// use minijinja::ErrorKind;
use minijinja::Value;
// use nom_supreme::error::ErrorTree;
use nom_supreme::error::GenericErrorTree;
//use nom_supreme::final_parser::final_parser;
use nom_supreme::final_parser::Location;
use nom_supreme::final_parser::RecreateContext;
// use nom_supreme::parser_ext::ParserExt;
use serde::Deserialize;
use serde::Serialize;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PageV42b {
    pub ast: Option<Vec<SectionV42>>,
    // TODO: Deprecate parsing_issue and push it into general issue list
    // if it's not there already
    pub parsing_issue: Option<NeoErrorV42>,
    pub href: Option<PathBuf>,
    pub id: Option<String>,
    pub issues: Vec<NeoErrorV42>,
    pub language: Option<String>,
    pub output: Option<String>,
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
            // line: details.line,
            // column: details.column,
            // source: content.to_string(),
            // remainder: location.to_string(),
            // message: kind.to_string(),
            Err(e) => {
                dbg!("HERERER");
                dbg!(&self.source_display_path);
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
                    _ => self.issues.push(NeoErrorV42::ParsingIssue {
                        column: 0,
                        completed: "WORK_IN_PROGRESS".to_string(),
                        line: 0,
                        message: "WORK_IN_PROGRESS".to_string(),
                        remainder: "WIP".to_string(),
                        source: "WORK_IN_PROGRESS".to_string(),
                        template_list: vec![],
                    }),
                };
            }
        }
    }

    pub fn generate_fallback_href(&mut self) {
        // TODO: Don't create the href if there's no ID
        // otherwise you won't be able to link to the page properly
        if let None = &self.href {
            match get_value_from_raw_pagedata_parsing(&self.source, "href") {
                Ok(v) => {
                    self.href = Some(v.1.join("").trim().to_string().into());
                    let _ = &self
                        .issues
                        .push(NeoErrorV42::new_page_issue(&self, vec![
                            "Warning: This pages is using a fallback href because one couldn't be found properly in the source file.".to_string(),
                            "If this page is showing up in an expected location that's probably why. Fixing whatever issues are happening in the source file should fix this as well.".to_string(),
                        ]));
                }
                Err(e) => {
                    dbg!("TODO: Add this error to the page stack");
                    dbg!(e);
                    ()
                }
            }
        }
    }

    pub fn generate_fallback_id(&mut self) {
        if let None = &self.id {
            match get_value_from_raw_pagedata_parsing(&self.source, "id") {
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
                    dbg!("TODO: Add this error to the page stack");
                    dbg!(e);
                    ()
                }
            }
        }
    }

    pub fn generate_href_from_ast(&mut self) {
        if let Ok(spans) = self.get_pagedata_from_ast("href") {
            self.href = Some(flatten_spans_v42(&spans).into());
        }
    }

    pub fn generate_id_from_ast(&mut self) {
        match self.get_pagedata_from_ast("id") {
            Ok(spans) => {
                self.id = Some(flatten_spans_v42(&spans));
            }
            Err(e) => {
                let _ = &self
                        .issues
                        .push(NeoErrorV42::new_page_issue(&self, vec![
                            "Could not get id from parsed filed".to_string(),
                            "This could mean that there is no '-- id: ...' in the '-- pagedata' section of the file.".to_string(),
                            "It could also mean that no '-- pagedata' section was found in the file.".to_string()
                        ]));
            }
        }
    }

    pub fn get_pagedata_from_ast(&self, key: &str) -> Result<Vec<SpanV42>, NeoErrorV42> {
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
                    Some(s) => match &s.attrs.get(key) {
                        Some(attr) => Ok(attr.spans.clone()),
                        None => Err(NeoErrorV42::NoParsedPageDataForKey(key.to_string())),
                    },
                    None => Err(NeoErrorV42::NoParsedPageDataForKey(key.to_string())),
                }
            }
            None => Err(NeoErrorV42::NoParsedPageDataForKey(key.to_string())),
        }
    }

    pub fn set_language_from_site_config(&mut self, site_config: &SiteConfigV42) {
        self.language = Some(site_config.default_language.clone());
    }

    //
}

impl Object for PageV42b {
    fn get_value(self: &Arc<Self>, key: &Value) -> Option<Value> {
        match key.as_str()? {
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
    // fn call_method(
    //     self: &Arc<PageV42>,
    //     _state: &minijinja::State,
    //     name: &str,
    //     args: &[Value],
    // ) -> Result<Value, Error> {
    //     match name {
    //         // "sections" => self.ast(),
    //         "get_pagedata" => self.make_value_from_string(self.get_pagedata(args)),
    //         // "sections" => self.sections(),
    //         "title" => self.title(),
    //         _ => Ok(Value::from("")),
    //     }
    // }
    //
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::span_v42::{plaintext::PlainTextSpan, SpanV42};
    use pretty_assertions::assert_eq;

    #[test]
    fn id_basic_check() {
        let page = PageV42b::test_from_sample_file("001-pagedata-only.neo");
        let left = "001-pagedata-only.neo".to_string();
        let right = page.id.unwrap();
        assert_eq!(left, right);
    }

    // #[test]
    // fn id_from_invalid_page() {
    //     let page = PageV42::test_from_sample_file("006-unclosed-start-section.neo");
    //     let left = "006-unclosed-start-section.neo".to_string();
    //     let right = page.id.unwrap();
    //     assert_eq!(left, right);
    // }

    // #[test]
    // fn title_from_pagedata_id() {
    //     let page = PageV42::test_from_sample_file("001-pagedata-only.neo");
    //     let left = Value::from_serialize(vec![SpanV42::PlainText(PlainTextSpan {
    //         text: "001-pagedata-only.neo".to_string(),
    //     })]);
    //     let right = page.title().unwrap();
    //     assert_eq!(left, right);
    // }

    // #[test]
    // fn title_from_pagedata_title() {
    //     let page = PageV42::test_from_sample_file("005-title-in-pagedata.neo");
    //     let left = Value::from_serialize(vec![SpanV42::PlainText(PlainTextSpan {
    //         text: "Title In Pagedata".to_string(),
    //     })]);
    //     let right = page.title().unwrap();
    //     assert_eq!(left, right);
    // }

    // #[test]
    // #[ignore]
    // fn title_from_invalid_page() {
    //     let page = PageV42::test_from_sample_file("006-unclosed-start-section.neo");
    //     let left = Value::from_serialize(vec![SpanV42::PlainText(PlainTextSpan {
    //         text: "006-unclosed-start-section.neo".to_string(),
    //     })]);
    //     let right = page.title().unwrap();
    //     assert_eq!(left, right);
    // }

    // #[test]
    // fn title_from_title_section() {
    //     let page = PageV42::test_from_sample_file("002-basic-sample.neo");
    //     let left = Value::from_serialize(vec![SpanV42::PlainText(PlainTextSpan {
    //         text: "Basic Sample".to_string(),
    //     })]);
    //     let right = page.title().unwrap();
    //     assert_eq!(left, right);
    // }

    // #[test]
    // fn default_html_file_system_relative_path_from_id() {
    //     let page = PageV42::test_from_sample_file("001-pagedata-only.neo");
    //     let left = PathBuf::from("en/001-pagedata-only.neo/index.html");
    //     let right = page.html_output_path.unwrap();
    //     assert_eq!(left, right);
    // }

    // #[test]
    // fn custom_path_in_pagedata() {
    //     let page = PageV42::test_from_sample_file("003-custom-href-in-pagedata.neo");
    //     let left = PathBuf::from("index.html");
    //     let right = page.html_output_path.unwrap();
    //     assert_eq!(left, right);
    // }

    // #[test]
    // fn href_from_id_default() {
    //     let page = PageV42::test_from_sample_file("001-pagedata-only.neo");
    //     let left = Value::from("/en/001-pagedata-only.neo/".to_string());
    //     let right = page.href().unwrap();
    //     assert_eq!(left, right);
    // }

    // #[test]
    // fn href_from_pagedata() {
    //     let page = PageV42::test_from_sample_file("003-custom-href-in-pagedata.neo");
    //     let left = Value::from("/".to_string());
    //     let right = page.href().unwrap();
    //     assert_eq!(left, right);
    // }

    // #[test]
    // fn href_for_non_index_html_file() {
    //     let page = PageV42::test_from_sample_file("009-non-index-custom-href.neo");
    //     let left = Value::from("/somewhere/custom.html".to_string());
    //     let right = page.href().unwrap();
    //     assert_eq!(left, right);
    // }

    //
}

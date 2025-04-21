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
use chrono::DateTime;
use chrono::FixedOffset;
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
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PageV42b {
    pub ast: Option<Vec<SectionV42>>,
    pub cache_hash: Option<String>,
    // TODO: Deprecate .href() method in favor of
    // of this .href field which should be prepopulated
    pub href: Option<String>,
    pub id: Option<String>,
    pub issues: Vec<NeoErrorV42>,
    pub language: Option<String>,
    pub output: Option<String>,
    pub output_for_rss: Option<String>,
    pub parsing_issue: Option<NeoErrorV42>,
    pub rel_output_path: Option<PathBuf>,
    pub rss_date: Option<Value>,
    pub rss_id: Option<Value>,
    pub sections: Option<Value>,
    pub status: Option<Value>,
    pub source: String,
    pub source_absolute_path: PathBuf,
    pub source_display_path: PathBuf,
    pub title: Option<String>,
    pub title_as_spans: Option<Vec<SpanV42>>,
}

impl PageV42b {
    pub fn date_created(&self) -> Result<DateTime<FixedOffset>, NeoErrorV42> {
        match self.get_page_attr_from_ast("created") {
            Ok(date_string) => match DateTime::parse_from_rfc3339(&date_string[0].text) {
                Ok(date) => Ok(date),
                Err(_e) => Err(NeoErrorV42::GenericError {
                    message: "Problem with date".to_string(),
                }),
            },
            Err(e) => Err(e),
        }
    }

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

    // // TODO: Check if this works
    // pub fn get_attr_from_last_section_of_type(
    //     &self,
    //     section_kind: &str,
    //     key: &str,
    // ) -> Result<&SectionKeyValueAttrPayloadV42, NeoErrorV42> {
    //     match &self.ast {
    //         Some(ast) => {
    //             match ast
    //                 .iter()
    //                 .filter_map(|section| match section {
    //                     SectionV42::Basic(sec) => {
    //                         if sec.kind == section_kind {
    //                             Some(sec)
    //                         } else {
    //                             None
    //                         }
    //                     }
    //                     _ => None,
    //                 })
    //                 .last()
    //             {
    //                 Some(s) => match s.attrs.get(key) {
    //                     Some(attr) => Ok(attr),
    //                     // TODO: Update this error to be generic for section
    //                     None => Err(NeoErrorV42::NoParsedPageDataForKey(key.to_string())),
    //                 },
    //                 // TODO: Update this error to be generic for section
    //                 None => Err(NeoErrorV42::NoParsedPageDataForKey(key.to_string())),
    //             }
    //         }
    //         // TODO: Update this error to be generic for section
    //         None => Err(NeoErrorV42::NoParsedPageDataForKey(key.to_string())),
    //     }
    // }

    pub fn get_page_attr_from_ast(
        &self,
        key: &str,
    ) -> Result<&Vec<SectionKeyValueAttrPayloadV42>, NeoErrorV42> {
        match &self.ast {
            Some(ast) => {
                // TODO: Make this use ``-- page`` or
                // ``-- metadata`` explicitly and not just
                // grab the last section like it is now
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

    pub fn get_sections(&self, key: &String) -> Option<Vec<SectionV42>> {
        if let Some(ast) = &self.ast {
            let section_capture: Vec<SectionV42> = ast
                .iter()
                .filter_map(|section| match section {
                    SectionV42::Basic(payload) => {
                        if &payload.kind == key {
                            Some(section.clone())
                        } else {
                            None
                        }
                    }
                    // SectionV42::Json5(paylaod) => true,
                    _ => {
                        //dbg!(&section);
                        None
                    }
                })
                .collect();
            Some(section_capture)
        } else {
            None
        }
    }

    pub fn does_page_data_have_flag(&self, key: &str) -> bool {
        match &self.ast {
            Some(ast) => {
                // TODO: Make this actually match the
                // ``-- page`` (soon to be ``-- metadata``
                // section instead of just pulling the last one.
                match ast
                    .iter()
                    .filter_map(|section| match section {
                        SectionV42::Json5(sec) => Some(sec),
                        _ => None,
                    })
                    .last()
                {
                    Some(s) => s.flags.contains(&key.to_string()),
                    None => false,
                }
            }
            None => false,
        }

        //
    }

    pub fn rss_date(&self) -> Option<Value> {
        // TODO: Figure out what to do if date
        // doesn't work
        match self.date_created() {
            Ok(date) => Some(Value::from(date.to_rfc2822())),
            Err(_) => Some(Value::from("")),
        }
    }

    pub fn rss_id(&self) -> Option<Value> {
        match &self.id {
            Some(id) => {
                let uuid = Uuid::new_v5(&Uuid::NAMESPACE_DNS, id.as_bytes());
                Some(Value::from(format!("{}", uuid)))
            }
            None => None,
        }
    }

    pub fn set_href(&mut self) {
        self.href = match &self.rel_output_path {
            Some(path) => match &path.file_name() {
                Some(file_name) => {
                    if file_name.to_str() == Some("index.html") {
                        match &path.parent() {
                            Some(p) => {
                                if p.to_str() == Some("") {
                                    Some(format!("/").into())
                                } else {
                                    Some(format!("/{}/", path.parent().unwrap().display()).into())
                                }
                            }
                            None => Some(format!("/{}/", path.parent().unwrap().display()).into()),
                        }
                    } else {
                        Some(format!("/{}", path.display()).into())
                    }
                }
                None => Some(format!("/{}", path.display()).into()),
            },
            None => {
                // TODO: Hoist Error
                None
            }
        }
    }

    pub fn set_id(&mut self) {
        match self.get_page_attr_from_ast("id") {
            Ok(attr) => self.id = Some(attr.last().unwrap().text.to_string()),
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
        self.language = Some(site_config.language_default.clone());
    }

    pub fn set_rel_output_path(&mut self) {
        match &self.get_page_attr_from_ast("path") {
            Ok(attr) => {
                let path = PathBuf::from(attr.last().unwrap().text.to_string());
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
            Ok(attr) => {
                let the_spans = attr.iter().flat_map(|x| x.spans.clone()).collect();
                Some(the_spans)
            }
            Err(_) => {
                match &self.ast {
                    Some(ast) => {
                        if let Some(t) = ast.iter().find_map(|section| match section {
                            SectionV42::Basic(sec) => match sec.kind.as_str() {
                                "title" => sec.children.iter().find_map(|child| match child {
                                    SectionV42::Block(c) => match c.kind.as_str() {
                                        "paragraph" => Some(c.spans.clone()),
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

    pub fn status(&self) -> Option<Value> {
        match self.get_page_attr_from_ast("status") {
            Ok(v) => Some(Value::from(v[0].text.clone())),
            // TODO: do something better than call this
            // unknown
            Err(_) => Some(Value::from("unknown")),
        }
        //Ok(Value::from(self.get_page_attr_from_ast("status")[0]))
        //        Ok(Value::from("asdf"))
    }

    //
}

impl Object for PageV42b {
    fn get_value(self: &Arc<Self>, key: &Value) -> Option<Value> {
        match key.as_str()? {
            // "href" => Some(Value::from(self.href.as_ref())),
            // "id" => Some(Value::from(self.id.clone().unwrap().clone())),
            "issues" => Some(Value::from_serialize(self.issues.clone())),
            "sections" => self.sections.clone(),
            "source" => Some(Value::from(self.source.clone())),
            "source_display_path" => {
                Some(Value::from(self.source_display_path.display().to_string()))
            }
            //"title" => Some(Value::from(self.title.clone())),
            "title_as_spans" => Some(Value::from_serialize(self.title_as_spans.clone())),
            _ => Some(Value::from(format!(
                "ISSUE: No field for page.{}",
                key.to_string()
            ))),
        }
    }

    fn call_method(
        self: &Arc<PageV42b>,
        _state: &minijinja::State,
        name: &str,
        _args: &[Value],
    ) -> Result<Value, Error> {
        match name {
            "href" => Ok(Value::from(self.href.as_ref())),
            "id" => Ok(Value::from(self.id.as_ref())),
            "parsing_issue" => Ok(Value::from_serialize(self.parsing_issue.clone())),
            // "rss_date" => Ok(self.rss_date()?),
            // "status" => Ok(self.status()?),
            "title" => Ok(Value::from(self.title.as_ref())),
            // "href" => self.href(),
            // "sections" => self.ast(),
            // "get_pagedata" => self.make_value_from_string(self.get_pagedata(args)),
            // "sections" => self.sections(),
            _ => Ok(Value::from(format!("ISSUE: no method: page.{}", name))),
        }
    }

    //
}

//#[cfg(test)]
//mod test {
//    use super::*;
//    use pretty_assertions::assert_eq;
//    //#[test]
//    //fn solo_date_created() {
//    //    let page = PageV42b::mock_from_data_tests("012-date-created.neo");
//    //    dbg!(page.date_created());
//    //    //assert_eq!(1, 2);
//    //}
//}

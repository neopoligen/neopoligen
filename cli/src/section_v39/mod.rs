#![allow(unused_imports)]

pub mod basic;
pub mod block;
pub mod mocks;
pub mod object;
pub mod raw;
pub mod yaml;

use crate::section_attr_v39::SectionAttrV39;
use crate::section_attr_v39::SectionAttrV39Kind;
use crate::section_v39::basic::*;
// use crate::section_v39::checklist::*;
// use crate::section_v39::comment::*;
// use crate::section_v39::generic::*;
// use crate::section_v39::json::*;
// use crate::section_v39::list::*;
use crate::section_v39::raw::*;
use crate::section_v39::yaml::*;
use crate::sections::*;
use crate::span_v39::*;
use minijinja::Error;
use minijinja::Value;
use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::character::complete::multispace0;
use nom::character::complete::newline;
use nom::character::complete::not_line_ending;
use nom::character::complete::space0;
use nom::character::complete::space1;
use nom::combinator::eof;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;
use serde::Deserialize;
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SectionV39 {
    pub attrs: Vec<SectionAttrV39>,
    pub bounds: SectionV39Bounds,
    pub kind: SectionV39Kind,
    pub r#type: String,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum SectionV39Bounds {
    Full,
    Start,
    End,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum SectionV39Kind {
    Basic {
        children: Vec<SectionV39>,
    },
    Block {
        spans: Vec<SpanV39>,
    },
    Raw {
        children: Vec<SectionV39>,
        text: Option<String>,
    },
    Yaml {},
}

impl SectionV39 {
    pub fn bounds(&self) -> Option<String> {
        match self.bounds {
            SectionV39Bounds::Full => Some("full".to_string()),
            SectionV39Bounds::Start => Some("start".to_string()),
            SectionV39Bounds::End => Some("end".to_string()),
        }
    }

    pub fn children(&self) -> Result<Value, minijinja::Error> {
        if let Some(children) = match &self.kind {
            SectionV39Kind::Basic { children } => Some(children),
            SectionV39Kind::Raw { children, .. } => Some(children),
            _ => None,
        } {
            Ok(Value::make_object_iterable(children.clone(), |child| {
                Box::new(child.iter().cloned().map(Value::from_object))
            }))
        } else {
            Ok(Value::from_serialize::<Vec<Value>>(vec![]))
        }
    }

    pub fn get_attr(&self, target_key: &str) -> Option<String> {
        let tokens = self
            .attrs
            .iter()
            .filter_map(|attr| match &attr.kind {
                SectionAttrV39Kind::KeyValue { key, value } => {
                    if key == target_key {
                        Some(value.clone())
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .collect::<Vec<String>>();
        if tokens.len() > 0 {
            Some(tokens.join(" "))
        } else {
            None
        }
    }

    pub fn ping(&self) -> Option<String> {
        Some("PING-PING-PING".to_string())
    }

    pub fn spans(&self) -> Result<Value, minijinja::Error> {
        if let Some(spans) = match &self.kind {
            SectionV39Kind::Block { spans } => Some(spans),
            _ => None,
        } {
            Ok(Value::make_object_iterable(spans.clone(), |span| {
                Box::new(span.iter().cloned().map(Value::from_serialize))
            }))
        } else {
            Ok(Value::from_serialize::<Vec<Value>>(vec![]))
        }
    }

    pub fn template(&self) -> Option<String> {
        if let Some(template) = self.attrs.iter().find_map(|attr| match &attr.kind {
            SectionAttrV39Kind::KeyValue { key, value } => {
                if key == "template" {
                    Some(value.to_string())
                } else {
                    None
                }
            }
            _ => None,
        }) {
            Some(template)
        } else {
            Some("default".to_string())
        }
    }

    pub fn template_list(&self) -> Vec<String> {
        let mut templates = vec![];
        if self.template().unwrap().as_str() == "default" {
            templates.push(format!(
                "sections/{}/{}/default.neoj",
                self.r#type,
                self.bounds().unwrap()
            ));
        } else {
            templates.push(format!(
                "sections/{}/{}/{}.neoj",
                self.r#type,
                self.bounds().unwrap(),
                self.template().unwrap()
            ));
            templates.push(format!(
                "sections/{}/{}/default.neoj",
                self.r#type,
                self.bounds().unwrap()
            ));
        }
        templates.push(format!(
            "sections/generic/{}/default.neoj",
            self.bounds().unwrap()
        ));
        templates
    }

    pub fn text(&self) -> Option<String> {
        match &self.kind {
            SectionV39Kind::Raw { text, .. } => {
                if let Some(t) = text {
                    Some(t.to_string())
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    pub fn r#type(&self) -> &String {
        &self.r#type
    }
}

pub fn initial_error<'a>() -> IResult<&'a str, &'a str, ErrorTree<&'a str>> {
    // the purpose of this function is just to put an
    // error in the accumulator. There's a way to do that
    // with just making an error, but I haven't solved all
    // the parts to that yet.
    let (_, _) = tag("asdf").parse("this will never match so it throws an intentional error")?;
    Ok(("", ""))
}

pub fn start_or_full_section_v39<'a>(
    source: &'a str,
    sections: &'a Sections,
    spans: &'a Vec<String>,
) -> IResult<&'a str, SectionV39, ErrorTree<&'a str>> {
    let (source, results) = alt((
        |src| basic_section_full_v39(src, &sections, &spans),
        // |src| basic_section_start(src, &sections, &spans),
        // |src| checklist_section_full(src, &sections, &spans),
        // |src| checklist_section_start(src, &sections, &spans),
        // |src| comment_section_full(src, &sections, &spans),
        // |src| comment_section_start(src, &sections, &spans),
        // |src| json_section_full(src, &sections, &spans),
        // |src| json_section_start(src, &sections, &spans),
        // |src| list_section_full(src, &sections, &spans),
        // |src| list_section_start(src, &sections, &spans),
        |src| raw_section_full_v39(src, &sections, &spans),
        |src| raw_section_start_v39(src, &sections, &spans),
        |src| yaml_section_full_v39(src, &sections, &spans),
        // |src| yaml_section_start(src, &sections, &spans),
        // // make sure generic is last
        // |src| generic_section_full(src, &sections, &spans),
        // |src| generic_section_start(src, &sections, &spans),
    ))
    .context("")
    .parse(source)?;
    Ok((source, results))
}

pub fn tag_finder<'a>(
    source: &'a str,
    section: &Vec<String>,
) -> IResult<&'a str, &'a str, ErrorTree<&'a str>> {
    let (source, result) = section
        .iter()
        .fold(initial_error(), |acc, item| match acc {
            Ok(v) => Ok(v),
            _ => tag(item.as_str()).context("").parse(source),
        })?;
    Ok((source, result))
}

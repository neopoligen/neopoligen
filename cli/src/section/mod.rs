#![allow(unused_imports)]

//use crate::section_attr_v39::SectionAttrV39;
//use crate::section_attr_v39::SectionAttrV39Kind;
//use crate::section_v39::basic::*;
// use crate::section_v39::checklist::*;
// use crate::section_v39::comment::*;
// use crate::section_v39::generic::*;
// use crate::section_v39::json::*;
// use crate::section_v39::list::*;
// use crate::section_v39::raw::*;
// use crate::section_v39::yaml::*;
// use crate::span_v39::*;
use crate::section_attr::SectionAttr;
use crate::site_config::ConfigSections;
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
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Section {
    pub attrs: Vec<SectionAttr>,
    pub bounds: SectionBounds,
    pub kind: SectionKind,
    pub r#type: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum SectionBounds {
    Full,
    Start,
    End,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum SectionKind {
    Basic {
        children: Vec<Section>,
    },
    Block {
        // spans: Vec<Span>,
    },
    Raw {
        children: Vec<Section>,
        text: Option<String>,
    },
    Yaml {},
}

pub fn start_or_full_section_v39<'a>(
    source: &'a str,
    sections: &'a ConfigSections,
) -> IResult<&'a str, Section, ErrorTree<&'a str>> {
    Ok((
        source,
        Section {
            attrs: vec![],
            bounds: SectionBounds::Full,
            kind: SectionKind::Block {},
            r#type: "title".to_string(),
        },
    ))
}

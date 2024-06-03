#![allow(unused_imports)]

pub mod basic;
pub mod block;
pub mod yaml;

//use crate::section_attr::SectionAttrV39;
//use crate::section_attr::SectionAttrV39Kind;
//use crate::section::basic::*;
// use crate::section::checklist::*;
// use crate::section::comment::*;
// use crate::section::generic::*;
// use crate::section::json::*;
// use crate::section::list::*;
// use crate::section::raw::*;
// use crate::section::yaml::*;
// use crate::span::*;
use crate::section::basic::*;
use crate::section::yaml::*;
use crate::section_attr::SectionAttr;
use crate::site_config::ConfigSections;
use crate::span::*;
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
        spans: Vec<Span>,
    },
    Raw {
        children: Vec<Section>,
        text: Option<String>,
    },
    Yaml {},
}

pub fn start_or_full_section<'a>(
    source: &'a str,
    sections: &'a ConfigSections,
) -> IResult<&'a str, Section, ErrorTree<&'a str>> {
    let (source, section) = alt((
        |src| basic_section_full(src, &sections),
        |src| yaml_section_full(src, &sections),
    ))
    .context("")
    .parse(source)?;
    Ok((source, section))
}

pub fn initial_error<'a>() -> IResult<&'a str, &'a str, ErrorTree<&'a str>> {
    // the purpose of this function is just to put an
    // error in the accumulator. There's a way to do that
    // with just making an error, but I haven't solved all
    // the parts to that yet.
    let (_, _) = tag("asdf").parse("this will never match so it throws an intentional error")?;
    Ok(("", ""))
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

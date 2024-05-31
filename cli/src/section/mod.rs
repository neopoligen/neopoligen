pub mod basic;
pub mod checklist;
pub mod comment;
pub mod generic;
pub mod json;
pub mod list;
pub mod raw;
pub mod yaml;

use crate::section::basic::*;
use crate::section::checklist::*;
use crate::section::comment::*;
use crate::section::generic::*;
use crate::section::json::*;
use crate::section::list::*;
use crate::section::raw::*;
use crate::section::yaml::*;
use crate::sections::*;
use crate::span::*;
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

// #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
// #[serde(tag = "kind", rename_all = "lowercase")]
// pub struct SectionAttrForList {
//     pub key: String,
//     pub value: String,
// }

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum Section {
    Basic {
        attrs: BTreeMap<String, String>,
        attr_list: Vec<SectionAttr>,
        bounds: String,
        children: Vec<Section>,
        flags: Vec<String>,
        r#type: String,
    },
    Block {
        bounds: String,
        spans: Vec<Span>,
        r#type: String,
    },
    Checklist {
        attrs: BTreeMap<String, String>,
        // TODO
        // attr_list: Vec<SectionAttr>,
        r#type: String,
        children: Vec<Section>,
        flags: Vec<String>,
        bounds: String,
    },
    ChecklistItem {
        bounds: String,
        children: Vec<Section>,
        status: bool,
        status_value: Option<String>,
        r#type: String,
    },
    Comment {
        // TODO
        // attrs: BTreeMap<String, String>,
        // attr_list: Vec<SectionAttr>,
        bounds: String,
        r#type: String,
        text: Option<String>,
        children: Vec<Section>,
    },
    Generic {
        attrs: BTreeMap<String, String>,
        // TODO
        // attr_list: Vec<SectionAttr>,
        bounds: String,
        children: Vec<Section>,
        flags: Vec<String>,
        r#type: String,
    },
    Json {
        attrs: BTreeMap<String, String>,
        // TODO
        // attr_list: Vec<SectionAttr>,
        bounds: String,
        r#type: String,
        data: Option<String>,
        flags: Vec<String>,
        children: Vec<Section>,
    },
    List {
        attrs: BTreeMap<String, String>,
        // TODO
        // attr_list: Vec<SectionAttr>,
        bounds: String,
        children: Vec<Section>,
        flags: Vec<String>,
        r#type: String,
    },
    ListItem {
        bounds: String,
        children: Vec<Section>,
        r#type: String,
    },
    Raw {
        attrs: BTreeMap<String, String>,
        // TODO
        // attr_list: Vec<SectionAttr>,
        bounds: String,
        children: Vec<Section>,
        flags: Vec<String>,
        r#type: String,
        text: Option<String>,
    },
    TagFinderInit,
    Yaml {
        attrs: BTreeMap<String, String>,
        // TODO
        // attr_list: Vec<SectionAttr>,
        bounds: String,
        children: Vec<Section>,
        data: Option<String>,
        flags: Vec<String>,
        r#type: String,
    },
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum SectionAttr {
    KeyValue { key: String, value: String },
    Flag { key: String },
}

pub fn empty_until_newline_or_eof<'a>(
    source: &'a str,
) -> IResult<&'a str, &'a str, ErrorTree<&'a str>> {
    let (source, _) = alt((
        tuple((space0, newline.map(|_| ""))),
        tuple((multispace0, eof.map(|_| ""))),
    ))
    .context("")
    .parse(source)?;
    Ok((source, ""))
}

pub fn initial_error<'a>() -> IResult<&'a str, &'a str, ErrorTree<&'a str>> {
    // the purpose of this function is just to put an
    // error in the accumulator. There's a way to do that
    // with just making an error, but I haven't solved all
    // the parts to that yet.
    let (_, _) = tag("asdf").parse("fdsa")?;
    Ok(("", ""))
}

pub fn section_attr<'a>(source: &'a str) -> IResult<&'a str, SectionAttr, ErrorTree<&'a str>> {
    let (source, attr) = alt((section_key_value_attr, section_flag_attr))
        .context("")
        .parse(source)?;
    Ok((source, attr))
}

pub fn section_key_value_attr<'a>(
    source: &'a str,
) -> IResult<&'a str, SectionAttr, ErrorTree<&'a str>> {
    let (source, _) = tag("--").context("").parse(source)?;
    let (source, _) = space1.context("").parse(source)?;
    let (source, key) = is_not(": \n").context("").parse(source)?;
    let (source, _) = tag(":").context("").parse(source)?;
    let (source, value) = not_line_ending.context("").parse(source)?;
    let (source, _) = line_ending.context("").parse(source)?;
    Ok((
        source,
        SectionAttr::KeyValue {
            key: key.trim().to_string(),
            value: value.trim().to_string(),
        },
    ))
}

pub fn section_flag_attr<'a>(source: &'a str) -> IResult<&'a str, SectionAttr, ErrorTree<&'a str>> {
    let (source, _) = tag("--").context("").parse(source)?;
    let (source, _) = space1.context("").parse(source)?;
    let (source, key) = is_not(":\n").context("").parse(source)?;
    let (source, _) = line_ending.context("").parse(source)?;
    Ok((
        source,
        SectionAttr::Flag {
            key: key.trim().to_string(),
        },
    ))
}

pub fn start_or_full_section<'a>(
    source: &'a str,
    sections: &'a Sections,
    spans: &'a Vec<String>,
) -> IResult<&'a str, Section, ErrorTree<&'a str>> {
    let (source, results) = alt((
        |src| basic_section_full(src, &sections, &spans),
        |src| basic_section_start(src, &sections, &spans),
        |src| checklist_section_full(src, &sections, &spans),
        |src| checklist_section_start(src, &sections, &spans),
        |src| comment_section_full(src, &sections, &spans),
        |src| comment_section_start(src, &sections, &spans),
        |src| json_section_full(src, &sections, &spans),
        |src| json_section_start(src, &sections, &spans),
        |src| list_section_full(src, &sections, &spans),
        |src| list_section_start(src, &sections, &spans),
        |src| raw_section_full(src, &sections, &spans),
        |src| raw_section_start(src, &sections, &spans),
        |src| yaml_section_full(src, &sections, &spans),
        |src| yaml_section_start(src, &sections, &spans),
        // make sure generic is last
        |src| generic_section_full(src, &sections, &spans),
        |src| generic_section_start(src, &sections, &spans),
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

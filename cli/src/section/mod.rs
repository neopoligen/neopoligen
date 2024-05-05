pub mod basic;
pub mod json;
pub mod list;
pub mod list_item;
pub mod raw;

use crate::block::*;
use crate::section::basic::basic_section;
use crate::section::json::json_section;
use crate::section::list::list_section;
use crate::section::raw::raw_section;
use crate::section_attr::*;
use crate::span::empty_line;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::line_ending;
use nom::character::complete::multispace0;
use nom::character::complete::space0;
use nom::character::complete::space1;
use nom::combinator::eof;
// use nom::combinator::not;
use nom::combinator::rest;
use nom::multi::many0;
use std::collections::BTreeMap;
// use nom::multi::many1;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;
use serde::Serialize;
use serde_json::Value;

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum Section {
    Basic {
        attrs: Vec<SectionAttr>,
        bounds: SectionBounds,
        content: Vec<Block>,
        source: String,
        r#type: String,
    },
    Checklist {
        attrs: Vec<SectionAttr>,
        bounds: SectionBounds,
        items: Vec<Section>,
        source: String,
        r#type: String,
    },
    Json {
        attrs: Vec<SectionAttr>,
        bounds: SectionBounds,
        data: Option<Value>,
        source: String,
        r#type: String,
    },
    List {
        attrs: Vec<SectionAttr>,
        bounds: SectionBounds,
        items: Vec<Section>,
        source: String,
        r#type: String,
    },
    ListItem {
        bounds: SectionBounds,
        content: Vec<Block>,
        source: String,
        r#type: String,
    },
    Raw {
        attrs: Vec<SectionAttr>,
        bounds: SectionBounds,
        text: String,
        source: String,
        r#type: String,
    },
    Unknown {
        attrs: Vec<SectionAttr>,
        bounds: SectionBounds,
        content: Vec<Block>,
        source: String,
        r#type: String,
    },
    Initializer,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SectionBounds {
    End,
    Full,
    Start,
}

pub fn section<'a>(
    source: &'a str,
    sections: &'a BTreeMap<String, Vec<String>>,
) -> IResult<&'a str, Section, ErrorTree<&'a str>> {
    let (source, result) = alt((
        |src| basic_section(src, &sections.get("basic").unwrap()),
        |src| json_section(src, &sections.get("json").unwrap()),
        |src| list_section(src, &sections.get("list").unwrap()),
        |src| raw_section(src, &sections.get("raw").unwrap()),
    ))
    .context("section")
    .parse(source)?;
    Ok((source, result))
}
fn initial_error<'a>() -> IResult<&'a str, Section, ErrorTree<&'a str>> {
    // the purpose of this function is just to put an
    // error in the accumulator. There's a way to do that
    // with just making an error, but I haven't solved all
    // the parts to that yet.
    let (_, _) = tag("asdf").parse("fdsa")?;
    Ok(("", Section::Initializer))
}

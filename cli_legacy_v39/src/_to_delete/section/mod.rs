pub mod basic;
pub mod end;
pub mod generic;
pub mod json;
pub mod list;
pub mod list_item;
pub mod raw;

use crate::block::*;
use crate::section::basic::basic_section;
use crate::section::end::end_section;
use crate::section::generic::generic_section;
use crate::section::json::json_section;
use crate::section::list::list_section;
use crate::section::raw::raw_section;
use crate::section_attr::*;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::combinator::rest;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;
use serde::Serialize;
use serde_json::Value;
use std::collections::BTreeMap;

use self::end::end_section_alt_position;

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
    Generic {
        attrs: Vec<SectionAttr>,
        bounds: SectionBounds,
        content: Vec<Block>,
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
        end_section,
        end_section_alt_position,
        |src| basic_section(src, &sections.get("basic").unwrap()),
        |src| json_section(src, &sections.get("json").unwrap()),
        |src| list_section(src, &sections.get("list").unwrap()),
        |src| raw_section(src, &sections.get("raw").unwrap()),
        generic_section, // keep this last
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

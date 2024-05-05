use std::collections::BTreeMap;

use crate::block::*;
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
use nom::combinator::not;
use nom::combinator::rest;
use nom::multi::many0;
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
        |src| raw_section(src, &sections.get("raw").unwrap()),
    ))
    .context("section")
    .parse(source)?;
    Ok((source, result))
}

fn basic_section<'a>(
    source: &'a str,
    list: &'a Vec<String>,
) -> IResult<&'a str, Section, ErrorTree<&'a str>> {
    let (source, result) = list.iter().fold(initial_error(), |acc, item| match acc {
        Ok(v) => Ok(v),
        _ => basic_section_finder(source, item),
    })?;
    Ok((source, result))
}

pub fn basic_section_finder<'a>(
    source: &'a str,
    key: &'a str,
) -> IResult<&'a str, Section, ErrorTree<&'a str>> {
    let initial_source = source;
    let (source, _) = tag("--").context("basic_section_finder").parse(source)?;
    let (source, _) = space1.context("basic_section_finder").parse(source)?;
    let (source, r#type) = tag(key).context("basic_section_finder").parse(source)?;
    let (source, _) = not(tag("/"))
        .context("basic_section_finder")
        .parse(source)?;
    let (source, _) = alt((tuple((multispace0, eof)), tuple((space0, line_ending))))
        .context("basic_section_finder")
        .parse(source)?;
    let (source, attrs) = many0(section_attr)
        .context("basic_section_finder")
        .parse(source)?;
    let (source, _) = alt((empty_line.map(|_| ""), eof))
        .context("basic_section_finder")
        .parse(source)?;
    let (source, result) = many0(block).context("basic_section_finder").parse(source)?;
    let initial_source = &initial_source.replace(source, "");
    Ok((
        source,
        Section::Basic {
            attrs,
            bounds: SectionBounds::Full,
            content: result,
            source: initial_source.to_string(),
            r#type: r#type.to_string(),
        },
    ))
}

fn json_section<'a>(
    source: &'a str,
    list: &'a Vec<String>,
) -> IResult<&'a str, Section, ErrorTree<&'a str>> {
    let (source, result) = list.iter().fold(initial_error(), |acc, item| match acc {
        Ok(v) => Ok(v),
        _ => json_section_finder(source, item),
    })?;
    Ok((source, result))
}

pub fn json_section_finder<'a>(
    source: &'a str,
    key: &'a str,
) -> IResult<&'a str, Section, ErrorTree<&'a str>> {
    let initial_source = source;
    let (source, _) = tag("--").context("json_section").parse(source)?;
    let (source, _) = space1.context("json_section").parse(source)?;
    let (source, r#type) = tag(key).context("json_section").parse(source)?;
    let (source, _) = tuple((space0, line_ending))
        .context("json_section")
        .parse(source)?;
    let (source, attrs) = many0(section_attr).context("json_section").parse(source)?;
    let (source, _) = alt((empty_line.map(|_| ""), eof))
        .context("json_section")
        .parse(source)?;
    let (source, raw_string) = alt((take_until("\n--"), rest))
        .context("json_section")
        .parse(source)?;
    let data = match serde_json::from_str::<Value>(raw_string) {
        Ok(data) => Some(data),
        Err(_) => None,
    };
    let (source, _) = multispace0(source)?;
    let initial_source = &initial_source.replace(source, "");
    Ok((
        source,
        Section::Json {
            attrs,
            bounds: SectionBounds::Full,
            data,
            source: initial_source.to_string(),
            r#type: r#type.to_string(),
        },
    ))
}

fn raw_section<'a>(
    source: &'a str,
    list: &'a Vec<String>,
) -> IResult<&'a str, Section, ErrorTree<&'a str>> {
    let (source, result) = list.iter().fold(initial_error(), |acc, item| match acc {
        Ok(v) => Ok(v),
        _ => raw_section_finder(source, item),
    })?;
    Ok((source, result))
}

pub fn raw_section_finder<'a>(
    source: &'a str,
    key: &'a str,
) -> IResult<&'a str, Section, ErrorTree<&'a str>> {
    let initial_source = source;
    let (source, _) = tag("--").context("raw_section").parse(source)?;
    let (source, _) = space1.context("raw_section").parse(source)?;
    let (source, r#type) = tag(key).context("raw_section").parse(source)?;
    let (source, _) = tuple((space0, line_ending))
        .context("raw_section")
        .parse(source)?;
    let (source, attrs) = many0(section_attr).context("raw_section").parse(source)?;
    let (source, _) = alt((empty_line.map(|_| ""), eof))
        .context("raw_section")
        .parse(source)?;
    let (source, raw_string) = alt((take_until("\n--"), rest))
        .context("raw_section")
        .parse(source)?;
    let data = match serde_json::from_str::<Value>(raw_string) {
        Ok(data) => Some(data),
        Err(_) => None,
    };
    let (source, _) = multispace0(source)?;
    let initial_source = &initial_source.replace(source, "");
    Ok((
        source,
        Section::Json {
            attrs,
            bounds: SectionBounds::Full,
            data,
            source: initial_source.to_string(),
            r#type: r#type.to_string(),
        },
    ))
}

fn initial_error<'a>() -> IResult<&'a str, Section, ErrorTree<&'a str>> {
    // the purpose of this function is just to put an
    // error in the accumulator. There's a way to do that
    // with just making an error, but I haven't solved all
    // the parts to that yet.
    let (_, _) = tag("asdf").parse("fdsa")?;
    Ok(("", Section::Initializer))
}

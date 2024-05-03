use crate::block::*;
use crate::page_error::PageError;
use crate::section_attrs::SectionAttrs;
use crate::span::empty_line;
use serde::Serialize;
// use nom::branch::alt;
// use nom::bytes::complete::is_not;
// use nom::character::complete::line_ending;
// use nom::character::complete::space0;
// use nom::character::complete::space1;
// use nom::combinator::not;
use nom::multi::many1;
// use nom::sequence::tuple;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::character::complete::space0;
use nom::character::complete::space1;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum Section {
    Checklist {
        attrs: Vec<SectionAttrs>,
        items: Vec<Section>,
        r#type: String,
    },
    List {
        attrs: Vec<SectionAttrs>,
        items: Vec<Section>,
        r#type: String,
    },
    Raw {
        attrs: Vec<SectionAttrs>,
        text: String,
        r#type: String,
    },
    Standard {
        attrs: Vec<SectionAttrs>,
        content: Vec<Block>,
        r#type: String,
    },
    Unknown {
        attrs: Vec<SectionAttrs>,
        content: Vec<Block>,
        r#type: String,
    },
}

#[derive(Debug, Serialize, Clone)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum SectionBounds {
    End,
    Full,
    Start,
}

pub fn section(source: &str) -> IResult<&str, Section, ErrorTree<&str>> {
    let (source, _) = tag("--").context("section").parse(source)?;
    let (source, _) = space1.context("section").parse(source)?;
    let (source, r#type) = is_not(" \n\t").context("section").parse(source)?;
    let (source, _) = tuple((space0, line_ending))
        .context("section")
        .parse(source)?;
    let (source, _) = empty_line.context("section").parse(source)?;
    let (source, result) = many1(block).context("section").parse(source)?;
    dbg!(source);
    Ok((
        source,
        Section::Standard {
            attrs: vec![],
            content: result,
            r#type: r#type.to_string(),
        },
    ))
}

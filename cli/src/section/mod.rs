use crate::block::*;
use crate::section_attr::*;
use crate::span::empty_line;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::character::complete::space0;
use nom::character::complete::space1;
use nom::multi::many0;
use nom::multi::many1;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum Section {
    Basic {
        attrs: Vec<SectionAttr>,
        content: Vec<Block>,
        source: String,
        r#type: String,
    },
    Checklist {
        attrs: Vec<SectionAttr>,
        items: Vec<Section>,
        source: String,
        r#type: String,
    },
    Json {
        attrs: Vec<SectionAttr>,
        source: String,
        r#type: String,
    },
    List {
        attrs: Vec<SectionAttr>,
        items: Vec<Section>,
        source: String,
        r#type: String,
    },
    Raw {
        attrs: Vec<SectionAttr>,
        text: String,
        source: String,
        r#type: String,
    },
    Unknown {
        attrs: Vec<SectionAttr>,
        content: Vec<Block>,
        source: String,
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
    let initial_source = source.clone();
    let (source, _) = tag("--").context("section").parse(source)?;
    let (source, _) = space1.context("section").parse(source)?;
    let (source, r#type) = is_not(" \n\t").context("section").parse(source)?;
    let (source, _) = tuple((space0, line_ending))
        .context("section")
        .parse(source)?;
    let (source, attrs) = many0(section_attr).context("section").parse(source)?;
    let (source, _) = empty_line.context("section").parse(source)?;
    let (source, result) = many1(block).context("section").parse(source)?;
    let initial_source = &initial_source.replace(source, "");
    Ok((
        source,
        Section::Basic {
            attrs,
            content: result,
            source: initial_source.to_string(),
            r#type: r#type.to_string(),
        },
    ))
}

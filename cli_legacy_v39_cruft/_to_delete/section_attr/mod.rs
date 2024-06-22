use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::character::complete::not_line_ending;
use nom::character::complete::space0;
use nom::combinator::eof;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum SectionAttr {
    Flag { key: String },
    KeyValue { key: String, value: String },
}

pub fn section_attr(source: &str) -> IResult<&str, SectionAttr, ErrorTree<&str>> {
    let (source, attr) = alt((flag_attr, kv_attr))
        .context("section_attr")
        .parse(source)?;
    Ok((source, attr))
}

pub fn flag_attr(source: &str) -> IResult<&str, SectionAttr, ErrorTree<&str>> {
    let (source, _) = tag("--").context("section_attr").parse(source)?;
    let (source, key) = is_not(":\n").context("section_attr").parse(source)?;
    let (source, _) = alt((line_ending, eof))
        .context("section_attr")
        .parse(source)?;
    Ok((
        source,
        SectionAttr::Flag {
            key: key.trim().to_string(),
        },
    ))
}

pub fn kv_attr(source: &str) -> IResult<&str, SectionAttr, ErrorTree<&str>> {
    let (source, _) = tag("--").context("section_attr").parse(source)?;
    let (source, key) = is_not(":").context("section_attr").parse(source)?;
    let (source, _) = tag(":").context("section_attr").parse(source)?;
    let (source, _) = space0.context("section_attr").parse(source)?;
    let (source, value) = not_line_ending.context("section_attr").parse(source)?;
    let (source, _) = alt((line_ending, eof))
        .context("section_attr")
        .parse(source)?;
    Ok((
        source,
        SectionAttr::KeyValue {
            key: key.trim().to_string(),
            value: value.trim().to_string(),
        },
    ))
}

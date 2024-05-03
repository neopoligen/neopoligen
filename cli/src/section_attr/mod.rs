use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
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
    let (source, _) = tag("--").context("section_attr").parse(source)?;
    let (source, key) = is_not(":\n").context("section_attr").parse(source)?;
    let (source, _) = line_ending.context("section_attr").parse(source)?;
    Ok((
        source,
        SectionAttr::Flag {
            key: key.trim().to_string(),
        },
    ))
}

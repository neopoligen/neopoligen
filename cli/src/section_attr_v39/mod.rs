use crate::span_v39::*;
//use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
// use nom::character::complete::line_ending;
// use nom::character::complete::multispace0;
// use nom::character::complete::newline;
use nom::character::complete::not_line_ending;
// use nom::character::complete::space0;
use nom::character::complete::space1;
// use nom::combinator::eof;
// use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;

#[derive(Clone, Debug, PartialEq)]
pub struct SectionAttrV39 {
    pub kind: SectionAttrV39Kind,
}

#[derive(Clone, Debug, PartialEq)]
pub enum SectionAttrV39Kind {
    KeyValue { key: String, value: String },
    Flag { flag: String },
}

pub fn section_attr_v39<'a>(
    source: &'a str,
) -> IResult<&'a str, SectionAttrV39, ErrorTree<&'a str>> {
    let (source, attr) = section_key_value_attr_39(source)?;
    Ok((source, attr))
}

pub fn section_key_value_attr_39<'a>(
    source: &'a str,
) -> IResult<&'a str, SectionAttrV39, ErrorTree<&'a str>> {
    let (source, _) = tag("--").context("").parse(source)?;
    let (source, _) = space1.context("").parse(source)?;
    let (source, key) = is_not(": \n").context("").parse(source)?;
    let (source, _) = tag(":").context("").parse(source)?;
    let (source, value) = not_line_ending.context("").parse(source)?;
    let (source, _) = line_ending_or_eof.context("").parse(source)?;
    Ok((
        source,
        SectionAttrV39 {
            kind: SectionAttrV39Kind::KeyValue {
                key: key.trim().to_string(),
                value: value.trim().to_string(),
            },
        },
    ))
}

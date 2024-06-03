use crate::block::*;
use crate::section_attr_v39::*;
use crate::section_v39::block::*;
use crate::section_v39::*;
use crate::span_v39::*;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::multispace0;
use nom::combinator::rest;
use nom::multi::many0;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;
use std::collections::BTreeMap;

pub fn raw_section_full_v39<'a>(
    source: &'a str,
    sections: &'a Sections,
    _spans: &'a Vec<String>,
) -> IResult<&'a str, SectionV39, ErrorTree<&'a str>> {
    let (source, _) = tag("-- ").context("").parse(source)?;
    let (source, r#type) = (|src| tag_finder(src, &sections.raw))
        .context("")
        .parse(source)?;
    let (source, _) = structure_empty_until_newline_or_eof
        .context("")
        .parse(source)?;
    let (source, attrs) = many0(section_attr_v39).context("").parse(source)?;
    let (source, _) = structure_empty_until_newline_or_eof
        .context("")
        .parse(source)?;
    let (source, text) = alt((take_until("\n--"), rest)).context("").parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    let section = SectionV39 {
        attrs,
        bounds: SectionV39Bounds::Full,
        details: None,
        kind: SectionV39Kind::Raw {
            children: vec![],
            text: Some(text.to_string()),
        },
        r#type: r#type.to_string(),
    };
    Ok((source, section))
}

pub fn raw_section_start_v39<'a>(
    source: &'a str,
    sections: &'a Sections,
    spans: &'a Vec<String>,
) -> IResult<&'a str, SectionV39, ErrorTree<&'a str>> {
    let (source, _) = tag("-- ").context("").parse(source)?;
    let (source, r#type) = (|src| tag_finder(src, &sections.raw))
        .context("")
        .parse(source)?;
    let end_key = format!("-- /{}", r#type);
    let (source, _) = tag("/").context("").parse(source)?;
    let (source, _) = structure_empty_until_newline_or_eof
        .context("")
        .parse(source)?;
    let (source, attrs) = many0(section_attr_v39).context("").parse(source)?;
    let (source, _) = structure_empty_until_newline_or_eof
        .context("")
        .parse(source)?;
    let (source, text) = take_until(end_key.as_str()).context("").parse(source)?;
    let (source, end_section) = basic_section_end_v39(source, spans, r#type)?;
    let (source, _) = multispace0.context("").parse(source)?;
    let mut children = vec![];
    children.push(end_section);
    let section = SectionV39 {
        attrs,
        bounds: SectionV39Bounds::Start,
        details: None,
        kind: SectionV39Kind::Raw {
            children,
            text: Some(text.trim_end().to_string()),
        },
        r#type: r#type.to_string(),
    };
    Ok((source, section))
}

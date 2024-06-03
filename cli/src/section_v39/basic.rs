use crate::block::*;
use crate::section_attr_v39::*;
use crate::section_v39::block::*;
use crate::section_v39::*;
use crate::span_v39::*;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::multi::many0;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;
use std::collections::BTreeMap;

pub fn basic_section_end_v39<'a>(
    source: &'a str,
    spans: &'a Vec<String>,
    key: &'a str,
) -> IResult<&'a str, SectionV39, ErrorTree<&'a str>> {
    let (source, _) = tag("-- ").context("").parse(source)?;
    let (source, _) = tag("/").context("").parse(source)?;
    let (source, r#type) = tag(key).context("").parse(source)?;
    let (source, _) = structure_empty_until_newline_or_eof
        .context("")
        .parse(source)?;
    let (source, attrs) = many0(section_attr_v39).context("").parse(source)?;
    let (source, _) = structure_empty_until_newline_or_eof
        .context("")
        .parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    let (source, children) = many0(|src| block_of_end_content_v39(src, &spans))
        .context("")
        .parse(source)?;
    let section = SectionV39 {
        attrs,
        bounds: SectionV39Bounds::End,
        details: None,
        kind: SectionV39Kind::Basic { children },
        r#type: r#type.to_string(),
    };
    Ok((source, section))
}

pub fn basic_section_full_v39<'a>(
    source: &'a str,
    sections: &'a Sections,
    spans: &'a Vec<String>,
) -> IResult<&'a str, SectionV39, ErrorTree<&'a str>> {
    let (source, _) = tag("-- ").context("").parse(source)?;
    let (source, r#type) = (|src| tag_finder(src, &sections.basic))
        .context("")
        .parse(source)?;
    let (source, _) = structure_empty_until_newline_or_eof
        .context("")
        .parse(source)?;
    let (source, attrs) = many0(section_attr_v39).context("").parse(source)?;
    let (source, _) = structure_empty_until_newline_or_eof
        .context("")
        .parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    let (source, children) = many0(|src| block_of_anything_v39(src, &spans))
        .context("")
        .parse(source)?;
    let section = SectionV39 {
        attrs,
        bounds: SectionV39Bounds::Full,
        details: None,
        kind: SectionV39Kind::Basic { children },
        r#type: r#type.to_string(),
    };
    Ok((source, section))
}

pub fn basic_section_start_v39<'a>(
    source: &'a str,
    sections: &'a Sections,
    spans: &'a Vec<String>,
) -> IResult<&'a str, SectionV39, ErrorTree<&'a str>> {
    let (source, _) = tag("-- ").context("").parse(source)?;
    let (source, r#type) = (|src| tag_finder(src, &sections.basic))
        .context("")
        .parse(source)?;
    let (source, _) = tag("/").context("").parse(source)?;
    let (source, _) = structure_empty_until_newline_or_eof
        .context("")
        .parse(source)?;
    let (source, attrs) = many0(section_attr_v39).context("").parse(source)?;
    let (source, _) = structure_empty_until_newline_or_eof
        .context("")
        .parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    let (source, mut children) = many0(alt((
        |src| block_of_anything_v39(src, &spans),
        |src| start_or_full_section_v39(src, &sections, &spans),
    )))
    .context("")
    .parse(source)?;
    let (source, end_section) = basic_section_end_v39(source, &spans, r#type)?;
    children.push(end_section);
    let section = SectionV39 {
        attrs,
        bounds: SectionV39Bounds::Start,
        details: None,
        kind: SectionV39Kind::Basic { children },
        r#type: r#type.to_string(),
    };
    Ok((source, section))
}

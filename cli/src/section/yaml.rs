use crate::section::block::*;
use crate::section::*;
use crate::section_attr::*;
use crate::span::*;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::multispace0;
use nom::combinator::rest;
use nom::multi::many0;
use nom::multi::many1;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;
use std::collections::BTreeMap;

pub fn yaml_section_full<'a>(
    source: &'a str,
    sections: &'a ConfigSections,
) -> IResult<&'a str, Section, ErrorTree<&'a str>> {
    let (source, _) = tag("-- ").context("").parse(source)?;
    let (source, r#type) = (|src| tag_finder(src, &sections.yaml))
        .context("")
        .parse(source)?;
    let (source, _) = structure_empty_until_newline_or_eof
        .context("")
        .parse(source)?;
    let (source, attrs) = many0(section_attr).context("").parse(source)?;
    let (source, _) = structure_empty_until_newline_or_eof
        .context("")
        .parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    let (source, _text) = alt((take_until("\n--"), rest, eof))
        .context("")
        .parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    // TODO: convert text to data.
    Ok((
        source,
        Section {
            attrs,
            bounds: SectionBounds::Full,
            kind: SectionKind::Yaml {},
            r#type: r#type.to_string(),
        },
    ))
}

use crate::section::*;
use crate::section_attr::*;
use crate::span::empty_line;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::character::complete::multispace0;
use nom::character::complete::space0;
use nom::character::complete::space1;
use nom::combinator::eof;
use nom::combinator::not;
use nom::combinator::opt;
use nom::multi::many0;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;
use serde_json::Value;

pub fn raw_section<'a>(
    source: &'a str,
    list: &'a Vec<String>,
) -> IResult<&'a str, Section, ErrorTree<&'a str>> {
    let (source, result) = list.iter().fold(initial_error(), |acc, item| match acc {
        Ok(v) => Ok(v),
        _ => alt((
            |src| raw_full_section_finder(src, item),
            |src| raw_start_section_finder(src, item),
        ))
        .context("raw_sectio")
        .parse(source),
    })?;
    Ok((source, result))
}

fn raw_full_section_finder<'a>(
    source: &'a str,
    key: &'a str,
) -> IResult<&'a str, Section, ErrorTree<&'a str>> {
    let initial_source = source;
    let (source, _) = tag("--").context("raw_full_section").parse(source)?;
    let (source, _) = space1.context("raw_full_section").parse(source)?;
    let (source, r#type) = tag(key).context("raw_full_section").parse(source)?;
    let (source, _) = not(tag("/")).context("raw_full_section").parse(source)?;
    let (source, _) = tuple((space0, line_ending))
        .context("raw_full_section")
        .parse(source)?;
    let (source, attrs) = many0(section_attr)
        .context("raw_full_section")
        .parse(source)?;
    let (source, _) = alt((empty_line.map(|_| ""), eof))
        .context("raw_full_section")
        .parse(source)?;
    let (source, text) = alt((take_until("\n--"), rest))
        .context("raw_full_section")
        .parse(source)?;
    let (source, _) = multispace0(source)?;
    let initial_source = &initial_source.replace(source, "");
    Ok((
        source,
        Section::Raw {
            attrs,
            bounds: SectionBounds::Full,
            text: text.to_string(),
            source: initial_source.to_string(),
            r#type: r#type.to_string(),
        },
    ))
}

fn raw_start_section_finder<'a>(
    source: &'a str,
    key: &'a str,
) -> IResult<&'a str, Section, ErrorTree<&'a str>> {
    let initial_source = source;
    let (source, _) = tag("--").context("raw_full_section").parse(source)?;
    let (source, _) = space1.context("raw_full_section").parse(source)?;
    let (source, r#type) = tag(key).context("raw_full_section").parse(source)?;
    let (source, _) = tag("/").context("raw_full_section").parse(source)?;
    let (source, _) = tuple((space0, line_ending))
        .context("raw_full_section")
        .parse(source)?;
    let (source, attrs) = many0(section_attr)
        .context("raw_full_section")
        .parse(source)?;
    let (source, _) = alt((empty_line.map(|_| ""), eof))
        .context("raw_full_section")
        .parse(source)?;
    let (source, text) = take_until("\n-- /")
        .context("raw_full_section")
        .parse(source)?;
    let (source, _) = multispace0(source)?;
    let initial_source = &initial_source.replace(source, "");
    Ok((
        source,
        Section::Raw {
            attrs,
            bounds: SectionBounds::Start,
            text: text.trim().to_string(),
            source: initial_source.to_string(),
            r#type: r#type.to_string(),
        },
    ))
}

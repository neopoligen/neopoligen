use crate::block::*;
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

pub fn basic_section<'a>(
    source: &'a str,
    list: &'a Vec<String>,
) -> IResult<&'a str, Section, ErrorTree<&'a str>> {
    let (source, result) = list.iter().fold(initial_error(), |acc, item| match acc {
        Ok(v) => Ok(v),
        _ => basic_full_section_finder(source, item),
    })?;
    Ok((source, result))
}

fn basic_full_section_finder<'a>(
    source: &'a str,
    key: &'a str,
) -> IResult<&'a str, Section, ErrorTree<&'a str>> {
    let initial_source = source;
    let (source, _) = tag("--")
        .context("basic_full_section_finder")
        .parse(source)?;
    let (source, _) = space1.context("basic_full_section_finder").parse(source)?;
    let (source, end) = opt(tag("/"))
        .context("basic_full_section_finder")
        .parse(source)?;
    let (source, r#type) = tag(key)
        .context("basic_full_section_finder")
        .parse(source)?;
    let (source, start) = opt(tag("/"))
        .context("basic_full_section_finder")
        .parse(source)?;
    let (source, _) = alt((tuple((multispace0, eof)), tuple((space0, line_ending))))
        .context("basic_full_section_finder")
        .parse(source)?;
    let (source, attrs) = many0(section_attr)
        .context("basic_full_section_finder")
        .parse(source)?;
    let (source, _) = alt((empty_line.map(|_| ""), eof))
        .context("basic_full_section_finder")
        .parse(source)?;
    let (source, _) = multispace0
        .context("basic_full_section_finder")
        .parse(source)?;
    let (source, result) = many0(block)
        .context("basic_full_section_finder")
        .parse(source)?;
    let initial_source = &initial_source.replace(source, "");
    if start.is_some() {
        Ok((
            source,
            Section::Basic {
                attrs,
                bounds: SectionBounds::Start,
                content: result,
                source: initial_source.to_string(),
                r#type: r#type.to_string(),
            },
        ))
    } else if end.is_some() {
        Ok((
            source,
            Section::Basic {
                attrs,
                bounds: SectionBounds::End,
                content: result,
                source: initial_source.to_string(),
                r#type: r#type.to_string(),
            },
        ))
    } else {
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
}

fn basic_start_section_finder<'a>(
    source: &'a str,
    key: &'a str,
) -> IResult<&'a str, Section, ErrorTree<&'a str>> {
    let initial_source = source;
    let (source, _) = tag("--")
        .context("basic_start_section_finder")
        .parse(source)?;
    let (source, _) = space1.context("basic_start_section_finder").parse(source)?;
    let (source, r#type) = tag(key)
        .context("basic_start_section_finder")
        .parse(source)?;
    let (source, _) = tag("/")
        .context("basic_start_section_finder")
        .parse(source)?;
    let (source, _) = alt((tuple((multispace0, eof)), tuple((space0, line_ending))))
        .context("basic_start_section_finder")
        .parse(source)?;
    let (source, attrs) = many0(section_attr)
        .context("basic_start_section_finder")
        .parse(source)?;
    let (source, _) = alt((empty_line.map(|_| ""), eof))
        .context("basic_start_section_finder")
        .parse(source)?;
    let (source, result) = many0(block)
        .context("basic_start_section_finder")
        .parse(source)?;
    let initial_source = &initial_source.replace(source, "");
    Ok((
        source,
        Section::Basic {
            attrs,
            bounds: SectionBounds::Start,
            content: result,
            source: initial_source.to_string(),
            r#type: r#type.to_string(),
        },
    ))
}

fn basic_end_section_finder<'a>(
    source: &'a str,
    key: &'a str,
) -> IResult<&'a str, Section, ErrorTree<&'a str>> {
    let initial_source = source;
    let (source, _) = tag("--")
        .context("basic_start_section_finder")
        .parse(source)?;
    let (source, _) = space1.context("basic_start_section_finder").parse(source)?;
    let (source, _) = tag("/")
        .context("basic_start_section_finder")
        .parse(source)?;
    let (source, r#type) = tag(key)
        .context("basic_start_section_finder")
        .parse(source)?;
    let (source, _) = alt((tuple((multispace0, eof)), tuple((space0, line_ending))))
        .context("basic_start_section_finder")
        .parse(source)?;
    let (source, attrs) = many0(section_attr)
        .context("basic_start_section_finder")
        .parse(source)?;
    let (source, _) = alt((empty_line.map(|_| ""), eof))
        .context("basic_start_section_finder")
        .parse(source)?;
    let (source, result) = many0(block)
        .context("basic_start_section_finder")
        .parse(source)?;
    let initial_source = &initial_source.replace(source, "");
    Ok((
        source,
        Section::Basic {
            attrs,
            bounds: SectionBounds::End,
            content: result,
            source: initial_source.to_string(),
            r#type: r#type.to_string(),
        },
    ))
}

use crate::section::list_item::list_item_full_section;
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
use nom::multi::many0;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;

pub fn list_section<'a>(
    source: &'a str,
    list: &'a Vec<String>,
) -> IResult<&'a str, Section, ErrorTree<&'a str>> {
    let (source, result) = list.iter().fold(initial_error(), |acc, item| match acc {
        Ok(v) => Ok(v),
        _ => list_section_finder(source, item),
    })?;
    Ok((source, result))
}

fn list_section_finder<'a>(
    source: &'a str,
    key: &'a str,
) -> IResult<&'a str, Section, ErrorTree<&'a str>> {
    let initial_source = source;
    let (source, _) = tag("--").context("list_section_finder").parse(source)?;
    let (source, _) = space1.context("list_section_finder").parse(source)?;
    let (source, r#type) = tag(key).context("list_section_finder").parse(source)?;
    let (source, _) = not(tag("/")).context("list_section_finder").parse(source)?;
    let (source, _) = alt((tuple((multispace0, eof)), tuple((space0, line_ending))))
        .context("list_section_finder")
        .parse(source)?;
    let (source, attrs) = many0(section_attr)
        .context("list_section_finder")
        .parse(source)?;
    let (source, _) = alt((empty_line.map(|_| ""), eof))
        .context("list_section_finder")
        .parse(source)?;
    let (source, items) = many0(list_item_full_section)
        .context("list_section_finder")
        .parse(source)?;
    //let (source, result) = many0(block).context("list_section_finder").parse(source)?;
    let initial_source = &initial_source.replace(source, "");
    Ok((
        source,
        Section::List {
            attrs,
            bounds: SectionBounds::Full,
            items,
            source: initial_source.to_string(),
            r#type: r#type.to_string(),
        },
    ))
}

use crate::block::*;
use crate::section::*;
// use crate::section_attr::*;
// use crate::span::empty_line;
// use nom::branch::alt;
use nom::bytes::complete::tag;
// use nom::bytes::complete::take_until;
// use nom::character::complete::line_ending;
// use nom::character::complete::multispace0;
// use nom::character::complete::space0;
use nom::character::complete::space1;
// use nom::combinator::eof;
use nom::combinator::not;
// use nom::combinator::rest;
use nom::multi::many1;
// use std::collections::BTreeMap;
// use nom::multi::many1;
// use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;
// use serde::Serialize;
// use serde_json::Value;

pub fn list_item_block<'a>(source: &'a str) -> IResult<&'a str, Block, ErrorTree<&'a str>> {
    let (source, _) = not(tag("-")).context("list_item_block").parse(source)?;
    let (source, the_block) = block.context("list_item_block").parse(source)?;
    Ok((source, the_block))
}

pub fn list_item_full_section<'a>(
    source: &'a str,
) -> IResult<&'a str, Section, ErrorTree<&'a str>> {
    let initial_source = source;
    let (source, _) = tag("-")
        .context("list_item_full_section_finder")
        .parse(source)?;
    let (source, _) = not(tag("/"))
        .context("list_item_full_finder")
        .parse(source)?;
    let (source, _) = space1.context("list_item_full_finder").parse(source)?;
    let (source, result) = many1(list_item_block)
        .context("list_item_full_finder")
        .parse(source)?;
    let initial_source = &initial_source.replace(source, "");
    Ok((
        source,
        Section::ListItem {
            bounds: SectionBounds::Full,
            content: result,
            source: initial_source.to_string(),
            r#type: "list_item".to_string(),
        },
    ))
}

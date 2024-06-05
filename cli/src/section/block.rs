use crate::section::Section;
use crate::section::SectionBounds;
use crate::section::SectionKind;
use crate::span::colon::*;
use crate::span::escaped_backslash::*;
use crate::span::escaped_backtick::*;
use crate::span::escaped_greaterthan::*;
use crate::span::escaped_pipe::*;
use crate::span::named_span::*;
use crate::span::single_backtick::*;
use crate::span::single_greaterthan::*;
use crate::span::single_lessthan::*;
use crate::span::wordpart::*;
use crate::span::*;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::combinator::eof;
use nom::combinator::not;
use nom::multi::many1;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;

use self::code_shorthand::code_shorthand;

#[derive(Clone, Debug, PartialEq)]
pub struct Block {}

pub fn block_of_anything<'a>(source: &'a str) -> IResult<&'a str, Section, ErrorTree<&'a str>> {
    let (source, _) = not(eof).context("").parse(source)?;
    let (source, _) = not(tag("--")).context("").parse(source)?;
    let (source, spans) = many1(alt((
        wordpart,
        space,
        newline,
        code_shorthand,
        named_span,
        colon,
        single_lessthan,
        single_greaterthan,
        single_backtick,
        escaped_backtick,
        escaped_pipe,
        escaped_greaterthan,
        escaped_backslash,
    )))
    .context("")
    .parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    Ok((
        source,
        Section {
            attrs: vec![],
            bounds: SectionBounds::Full,
            kind: SectionKind::Block { spans },
            r#type: "block-of-text".to_string(),
        },
    ))
}

pub fn block_of_end_content<'a>(source: &'a str) -> IResult<&'a str, Section, ErrorTree<&'a str>> {
    let (source, _) = not(eof).context("").parse(source)?;
    let (source, _) = not(tag("-")).context("").parse(source)?;
    let (source, _) = not(tag("[")).context("").parse(source)?;
    let (source, spans) = many1(|src| span_for_body_text(src))
        .context("")
        .parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    Ok((
        source,
        Section {
            attrs: vec![],
            bounds: SectionBounds::Full,
            kind: SectionKind::Block { spans },
            r#type: "block-of-text".to_string(),
        },
    ))
}

pub fn block_of_list_content<'a>(source: &'a str) -> IResult<&'a str, Section, ErrorTree<&'a str>> {
    let (source, _) = not(eof).context("").parse(source)?;
    let (source, _) = not(tag("-")).context("").parse(source)?;
    let (source, spans) = many1(span_for_body_text).context("").parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    Ok((
        source,
        Section {
            attrs: vec![],
            bounds: SectionBounds::Full,
            kind: SectionKind::Block { spans },
            r#type: "block-of-text".to_string(),
        },
    ))
}

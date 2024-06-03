pub mod code_shorthand;
pub mod escaped_backslash;
pub mod escaped_backtick;
pub mod escaped_colon;
pub mod escaped_pipe;
pub mod single_backtick;

use crate::span::code_shorthand::*;
use crate::span::escaped_backslash::*;
use crate::span::escaped_backtick::*;
use crate::span::escaped_colon::*;
use crate::span::escaped_pipe::*;
use crate::span::single_backtick::*;
use crate::span_attr::*;
// use crate::span::link_shorthand::link_shorthand;
use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::character::complete::line_ending;
use nom::character::complete::multispace0;
use nom::character::complete::space0;
use nom::character::complete::space1;
use nom::combinator::eof;
use nom::combinator::not;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Span {
    pub attrs: Vec<SpanAttr>,
    pub kind: SpanKind,
    pub parsed_text: String,
    pub source_text: String,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "lowercase", tag = "type")]
pub enum SpanKind {
    SingleBacktick,
    CodeShorthand,
    EscapedBacktick,
    EscapedBackslash,
    EscapedColon,
    EscapedPipe,
    LinkShorthand,
    Newline,
    Space,
    WordPart,
}

// Reminder: This doesn't output a span for content
// it's only for the structure of the file
pub fn structure_empty_until_newline_or_eof<'a>(
    source: &'a str,
) -> IResult<&'a str, &'a str, ErrorTree<&'a str>> {
    let (source, _) = alt((
        tuple((space0, line_ending)),
        tuple((multispace0, eof.map(|_| ""))),
    ))
    .context("")
    .parse(source)?;
    Ok((source, ""))
}

pub fn span<'a>(
    source: &'a str,
    _spans: &'a Vec<String>,
) -> IResult<&'a str, Span, ErrorTree<&'a str>> {
    let (source, span) = alt((
        code_shorthand,
        // link_shorthand,
        escaped_backtick,
        escaped_backslash,
        escaped_colon,
        escaped_pipe,
        single_backtick,
        wordpart,
        space,
        newline,
    ))(source)?;
    Ok((source, span))
}

pub fn span_for_shorthand_text<'a>(source: &'a str) -> IResult<&'a str, Span, ErrorTree<&'a str>> {
    let (source, span) = alt((
        escaped_pipe,
        escaped_backtick,
        escaped_backslash,
        escaped_colon,
        single_backtick,
        wordpart,
        space,
        newline,
    ))(source)?;
    Ok((source, span))
}

pub fn span_without_shorthands_or_single_pipe<'a>(
    source: &'a str,
) -> IResult<&'a str, Span, ErrorTree<&'a str>> {
    let (source, span) = alt((
        escaped_pipe,
        escaped_backtick,
        escaped_backslash,
        escaped_colon,
        single_backtick,
        wordpart,
        space,
        newline,
    ))(source)?;
    Ok((source, span))
}

// TODO: Move to own file with tests
pub fn newline(source: &str) -> IResult<&str, Span, ErrorTree<&str>> {
    let initial_source = source;
    let (source, _) = tuple((space0, line_ending)).context("").parse(source)?;
    let (source, _) = not(tuple((space0, line_ending)))
        .context("")
        .parse(source)?;
    let source_text = initial_source.replace(source, "").to_string();
    Ok((
        source,
        Span {
            attrs: vec![],
            source_text,
            parsed_text: "\n".to_string(),
            kind: SpanKind::Newline,
        },
    ))
}

// TODO: Move to own file with tests
pub fn space(source: &str) -> IResult<&str, Span, ErrorTree<&str>> {
    let initial_source = source;
    let (source, _) = space1.context("").parse(source)?;
    let source_text = initial_source.replace(source, "").to_string();
    Ok((
        source,
        Span {
            attrs: vec![],
            source_text,
            parsed_text: " ".to_string(),
            kind: SpanKind::Space,
        },
    ))
}

// TODO: Move to own file with tests
pub fn wordpart(source: &str) -> IResult<&str, Span, ErrorTree<&str>> {
    let initial_source = source;
    let (source, text) = is_not(" \\`|\n\t").context("").parse(source)?;
    let source_text = initial_source.replace(source, "").to_string();
    Ok((
        source,
        Span {
            attrs: vec![],
            source_text,
            parsed_text: text.to_string(),
            kind: SpanKind::WordPart,
        },
    ))
}


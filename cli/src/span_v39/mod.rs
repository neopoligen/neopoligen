#![allow(unused_imports)]

use nom::branch::alt;
use nom::bytes::complete::is_not;
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
use serde::Deserialize;
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq)]
pub struct SpanV39 {
    pub kind: SpanV39Kind,
}

#[derive(Clone, Debug, PartialEq)]
pub enum SpanV39Kind {
    Space { text: String },
    WordPart { text: String },
}

// TODO: Needs test
pub fn empty_until_newline_or_eof<'a>(
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

pub fn line_ending_or_eof(source: &str) -> IResult<&str, &str, ErrorTree<&str>> {
    let (source, result) = alt((line_ending, eof))(source)?;
    Ok((source, result))
}

// TODO: Needs test
pub fn newline_v39(source: &str) -> IResult<&str, SpanV39, ErrorTree<&str>> {
    let (source, _text) = tuple((space0, line_ending)).context("").parse(source)?;
    let (source, _) = not(tuple((space0, line_ending)))
        .context("")
        .parse(source)?;
    Ok((
        source,
        SpanV39 {
            kind: SpanV39Kind::Space {
                text: " ".to_string(),
            },
        },
    ))
}

pub fn space(source: &str) -> IResult<&str, SpanV39, ErrorTree<&str>> {
    let (source, text) = space1.context("").parse(source)?;
    Ok((
        source,
        SpanV39 {
            kind: SpanV39Kind::Space {
                text: text.to_string(),
            },
        },
    ))
}

pub fn span_v39<'a>(
    source: &'a str,
    _spans: &'a Vec<String>,
) -> IResult<&'a str, SpanV39, ErrorTree<&'a str>> {
    let (source, span) = alt((word_part, space))(source)?;
    Ok((source, span))
}

pub fn word_part(source: &str) -> IResult<&str, SpanV39, ErrorTree<&str>> {
    let (source, text) = is_not(" ").context("").parse(source)?;
    Ok((
        source,
        SpanV39 {
            kind: SpanV39Kind::WordPart {
                text: text.to_string(),
            },
        },
    ))
}

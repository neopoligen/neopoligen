#![allow(unused_imports)]

use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::character::complete::multispace0;
use nom::character::complete::space0;
use nom::character::complete::space1;
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

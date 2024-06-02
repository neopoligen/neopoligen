use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
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

use crate::span_attr_v39::SpanAttrV39;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SpanV39 {
    pub kind: SpanV39Kind,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "lowercase", tag = "type")]
pub enum SpanV39Kind {
    Backtick {
        source_text: String,
    },
    CodeShorthand {
        attrs: Vec<SpanAttrV39>,
        source_text: String,
        parsed_text: String,
    },
    EscapedBacktick {
        source_text: String,
    },
    Newline {
        source_text: String,
    },
    Space {
        source_text: String,
    },
    WordPart {
        source_text: String,
    },
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

pub fn span_v39<'a>(
    source: &'a str,
    _spans: &'a Vec<String>,
) -> IResult<&'a str, SpanV39, ErrorTree<&'a str>> {
    let (source, span) = alt((
        escaped_backtick_v39,
        backtick_v39,
        word_part_v39,
        space_v39,
        newline_v39,
    ))(source)?;
    Ok((source, span))
}

pub fn backtick_v39(source: &str) -> IResult<&str, SpanV39, ErrorTree<&str>> {
    let (source, _) = tag("`").context("").parse(source)?;
    let (source, _) = not(tag("`")).context("").parse(source)?;
    Ok((
        source,
        SpanV39 {
            kind: SpanV39Kind::Backtick {
                source_text: "`".to_string(),
            },
        },
    ))
}

pub fn code_shorthand_v39(source: &str) -> IResult<&str, SpanV39, ErrorTree<&str>> {
    let (source, _) = tag("``").context("").parse(source)?;
    let (source, text) = is_not("`").context("").parse(source)?;
    let (source, _) = tag("``").context("").parse(source)?;
    let attrs = vec![];
    Ok((
        source,
        SpanV39 {
            kind: SpanV39Kind::CodeShorthand {
                attrs,
                source_text: "``code``".to_string(),
                parsed_text: text.to_string(),
            },
        },
    ))
}

pub fn escaped_backtick_v39(source: &str) -> IResult<&str, SpanV39, ErrorTree<&str>> {
    let (source, text) = tag("\\`").context("").parse(source)?;
    Ok((
        source,
        SpanV39 {
            kind: SpanV39Kind::EscapedBacktick {
                source_text: text.to_string(),
            },
        },
    ))
}
pub fn newline_v39(source: &str) -> IResult<&str, SpanV39, ErrorTree<&str>> {
    let (source, text) = tuple((space0, line_ending)).context("").parse(source)?;
    let (source, _) = not(tuple((space0, line_ending)))
        .context("")
        .parse(source)?;
    Ok((
        source,
        SpanV39 {
            kind: SpanV39Kind::Newline {
                source_text: format!("{}{}", text.0, text.1),
            },
        },
    ))
}

pub fn space_v39(source: &str) -> IResult<&str, SpanV39, ErrorTree<&str>> {
    let (source, text) = space1.context("").parse(source)?;
    Ok((
        source,
        SpanV39 {
            kind: SpanV39Kind::Space {
                source_text: text.to_string(),
            },
        },
    ))
}

pub fn word_part_v39(source: &str) -> IResult<&str, SpanV39, ErrorTree<&str>> {
    let (source, text) = is_not(" \n\t`").context("").parse(source)?;
    Ok((
        source,
        SpanV39 {
            kind: SpanV39Kind::WordPart {
                source_text: text.to_string(),
            },
        },
    ))
}

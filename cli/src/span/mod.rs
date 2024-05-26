pub mod backtic;
pub mod close_brace;
pub mod code;
pub mod em_shorthand;
pub mod escaped_pipe;
pub mod footnote;
pub mod greater_than;
pub mod html;
pub mod known_span;
pub mod less_than;
pub mod link;
pub mod open_brace;
pub mod strike;
pub mod strong;
pub mod underscore;

use crate::span::backtic::*;
use crate::span::close_brace::*;
use crate::span::code::*;
use crate::span::em_shorthand::*;
use crate::span::escaped_pipe::*;
use crate::span::footnote::*;
use crate::span::greater_than::*;
use crate::span::html::*;
use crate::span::known_span::*;
use crate::span::less_than::*;
use crate::span::link::*;
use crate::span::open_brace::*;
use crate::span::strike::*;
use crate::span::strong::*;
use crate::span::underscore::*;
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

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum Span {
    Code {
        attrs: BTreeMap<String, String>,
        flags: Vec<String>,
        text: String,
        r#type: String,
    },
    Em {
        attrs: BTreeMap<String, String>,
        flags: Vec<String>,
        text: String,
        r#type: String,
    },
    Footnote {
        attrs: BTreeMap<String, String>,
        flags: Vec<String>,
        text: String,
        r#type: String,
    },
    Html {
        text: String,
        r#type: String,
    },
    KnownSpan {
        attrs: BTreeMap<String, String>,
        flags: Vec<String>,
        spans: Vec<Span>,
        r#type: String,
    },
    /*
    Link {
        attrs: BTreeMap<String, String>,
        flags: Vec<String>,
        text: String,
        href: Option<String>,
        r#type: String,
    },
    */
    Newline {
        text: String,
        r#type: String,
    },
    RawText {
        text: String,
        r#type: String,
    },
    Space {
        text: String,
        r#type: String,
    },
    S {
        attrs: BTreeMap<String, String>,
        flags: Vec<String>,
        text: String,
        r#type: String,
    },
    Strong {
        attrs: BTreeMap<String, String>,
        flags: Vec<String>,
        text: String,
        r#type: String,
    },
    UnknownSpan {
        r#type: String,
        spans: Vec<Span>,
        flags: Vec<String>,
        attrs: BTreeMap<String, String>,
    },
    WordPart {
        text: String,
        r#type: String,
    },
}

pub enum SpanAttr {
    KeyValue { key: String, value: String },
    Flag { key: String },
}

pub fn span_finder<'a>(
    source: &'a str,
    spans: &'a Vec<String>,
) -> IResult<&'a str, Span, ErrorTree<&'a str>> {
    let (source, span) = alt((
        escaped_pipe,
        strike_shorthand,
        |src| code_shorthand(src, spans),
        |src| em_shorthand(src, spans),
        footnote_shorthand,
        |src| link_shorthand(src, spans),
        strong_shorthand,
        html_shorthand,
        |src| known_span(src, spans),
        newline,
        space,
        word_part,
        open_brace,
        close_brace,
        less_than,
        greater_than,
        backtic,
        underscore,
        |src| unknown_span(src, spans),
    ))
    .context("")
    .parse(source)?;
    Ok((source, span))
}

pub fn newline(source: &str) -> IResult<&str, Span, ErrorTree<&str>> {
    let (source, text) = tuple((space0, line_ending)).context("").parse(source)?;
    let (source, _) = not(tuple((space0, line_ending)))
        .context("")
        .parse(source)?;
    Ok((
        source,
        Span::Space {
            text: text.1.to_string(),
            r#type: "space".to_string(),
        },
    ))
}

pub fn space(source: &str) -> IResult<&str, Span, ErrorTree<&str>> {
    let (source, text) = space1.context("").parse(source)?;
    Ok((
        source,
        Span::Space {
            text: text.to_string(),
            r#type: "space".to_string(),
        },
    ))
}

pub fn span_key_value_attr(source: &str) -> IResult<&str, SpanAttr, ErrorTree<&str>> {
    let (source, _) = tag("|").context("").parse(source)?;
    let (source, key) = is_not(" |\n\t:").context("").parse(source)?;
    let (source, _) = tag(":").context("").parse(source)?;
    let (source, _) = space1.context("").parse(source)?;
    let (source, value) = is_not(">|").context("").parse(source)?;
    Ok((
        source,
        SpanAttr::KeyValue {
            key: key.trim().to_string(),
            value: value.trim().to_string(),
        },
    ))
}

pub fn span_flag_attr(source: &str) -> IResult<&str, SpanAttr, ErrorTree<&str>> {
    let (source, _) = tag("|").context("").parse(source)?;
    let (source, key) = is_not(" |\n\t>").context("").parse(source)?;
    Ok((
        source,
        SpanAttr::Flag {
            key: key.trim().to_string(),
        },
    ))
}

pub fn unknown_span<'a>(
    source: &'a str,
    spans: &'a Vec<String>,
) -> IResult<&'a str, Span, ErrorTree<&'a str>> {
    let (source, _) = tag("<<").context("").parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    let (source, r#type) = is_not(" |><").context("").parse(source)?;
    let (source, _) = tag("|").context("").parse(source)?;
    let (source, spans) = many0(|src| span_finder(src, spans))
        .context("")
        .parse(source)?;
    let (source, raw_attrs) = many0(alt((span_key_value_attr, span_flag_attr)))
        .context("")
        .parse(source)?;
    let (source, _) = tag(">>").context("").parse(source)?;
    let mut flags: Vec<String> = vec![];
    let mut attrs = BTreeMap::new();
    raw_attrs.iter().for_each(|attr| match attr {
        SpanAttr::KeyValue { key, value } => {
            attrs.insert(key.to_string(), value.to_string());
        }
        SpanAttr::Flag { key } => flags.push(key.to_string()),
    });
    Ok((
        source,
        Span::UnknownSpan {
            r#type: r#type.to_string(),
            spans,
            flags,
            attrs,
        },
    ))
}

pub fn word_part(source: &str) -> IResult<&str, Span, ErrorTree<&str>> {
    let (source, text) = is_not(" \n\t<>|^[]_`").context("").parse(source)?;
    Ok((
        source,
        Span::WordPart {
            text: text.to_string(),
            r#type: "wordpart".to_string(),
        },
    ))
}

pub fn known_span_type<'a>(
    source: &'a str,
    spans: &Vec<String>,
) -> IResult<&'a str, &'a str, ErrorTree<&'a str>> {
    let (source, result) = spans
        .iter()
        .fold(span_initial_error(), |acc, item| match acc {
            Ok(v) => Ok(v),
            _ => tag(item.as_str()).parse(source),
        })?;
    Ok((source, result))
}

pub fn span_initial_error<'a>() -> IResult<&'a str, &'a str, ErrorTree<&'a str>> {
    // the purpose of this function is just to put an
    // error in the accumulator. There's a way to do that
    // with just making an error, but I haven't solved all
    // the parts to that yet.
    let (_, _) = tag("asdf").parse("fdsa")?;
    Ok(("", ""))
}

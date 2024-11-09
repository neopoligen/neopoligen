
#![allow(unused)]
use super::PlainTextSpan;
use crate::helpers::flatten_spans_v42;
use crate::span_attrs_v42::*;
use crate::span_v42::string_parsers::*;
use crate::span_v42::SpanV42;
use nom::branch::alt;
use nom::bytes::complete::is_a;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::character::complete::alphanumeric1;
use nom::character::complete::multispace0;
use nom::character::complete::space0;
use nom::character::complete::space1;
use nom::combinator::not;
use nom::multi::many0;
use nom::multi::many1;
use nom::sequence::pair;
use nom::sequence::preceded;
use nom::sequence::terminated;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::ParserExt;
use serde::Deserialize;
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct CodeShorthandV42 {
    pub attrs: BTreeMap<String, Vec<SpanV42>>,
    pub flags: Vec<String>,
    pub kind: String,
    pub text: String,
}

pub fn code_shorthand_span(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, tokens) = preceded(pair(tag("``"), multispace0), many1(alt((
            plain_text_string_base,
            plain_text_space1_as_single_space,
            is_a("~*^![]{}<>_#:"),
            plain_text_single_newline_as_space,
            escaped_character,
        ))))
        .context("")
        .parse(source)?;
    let (source, (flags, attrs)) = code_shorthand_flags_and_attrs
        .context("")
        .parse(source)?;
    let (source, _) = tag("``").context("").parse(source)?;
    Ok((
        source,
        SpanV42::CodeShorthand(CodeShorthandV42 {
            kind: "code-shorthand".to_string(),
            attrs,
            flags,
            text: tokens.join(""),
        }),
    ))
}

pub fn code_shorthand_flags_and_attrs(
    source: &str,
) -> IResult<&str, (Vec<String>, BTreeMap<String, Vec<SpanV42>>), ErrorTree<&str>> {
    let (source, flags_and_attrs) = many0(alt((
        // code_shorthand_key_value_attr,
        code_shorthand_flag,
    )))
    .context("")
    .parse(source)?;
    let attrs = BTreeMap::new();
    let mut flags = vec![];
    flags_and_attrs.iter().for_each(|af| match &af {
        SpanBaseAttrV42::Flag(f) => flags.push(f.text.to_string()),
        _ => (),
    });
    Ok((source, (flags, attrs)))
}

pub fn code_shorthand_flag(source: &str) -> IResult<&str, SpanBaseAttrV42, ErrorTree<&str>> {
    let (source, _) = tag("|").context("").parse(source)?;
    let (source, parts) = many1(alt((
        plain_text_string_base,
        is_a("~*^![]{}<>_#:"),
        escaped_character,
    )))
    .context("")
    .parse(source)?;
    let (source, _) = not(tag(":")).context("").parse(source)?;
    Ok((
        source,
        SpanBaseAttrV42::Flag(SpanFlagAttrV42 {
            text: parts.join(""),
        }),
    ))
}


// TODO: Deprecate this
pub fn not_backtick_or_pipe_or_escape(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, result) = is_not("`\\|").context("").parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            kind: "plain-text".to_string(),
            text: result.to_string(),
        }),
    ))
}

pub fn code_shorthand_span_flag_attr(
    source: &str,
) -> IResult<&str, SpanBaseAttrV42, ErrorTree<&str>> {
    let (source, spans) = many1(alt((
        not_backtick_or_pipe_or_escape,
    )))
    .context("")
    .parse(source)?;
    let (source, _) = not(tag(":")).context("").parse(source)?;
    Ok((
        source,
        SpanBaseAttrV42::Flag(SpanFlagAttrV42 {
            text: flatten_spans_v42(&spans),
        }),
    ))
}

pub fn code_shorthand_key_value_attr(
    source: &str,
) -> IResult<&str, SpanBaseAttrV42, ErrorTree<&str>> {
    let (source, key_spans) = terminated(
        many1(alt((alphanumeric1, is_a("-_")))),
        pair(tag(":"), space1),
    )
    .context("")
    .parse(source)?;
    let (source, spans) = many1(alt((
        not_backtick_or_pipe_or_escape,
    )))
    .context("")
    .parse(source)?;
    Ok((
        source,
        SpanBaseAttrV42::KeyValue {
            key: key_spans.join(""),
            spans,
        },
    ))
}

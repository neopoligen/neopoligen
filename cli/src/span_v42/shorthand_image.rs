
#![allow(unused)]
use super::PlainTextSpan;
use crate::helpers::flatten_spans_v42;
use crate::span_attrs_v42::*;
use crate::span_v42::named_span::named_span;
use crate::span_v42::string_parsers::*;
use crate::span_v42::SpanV42;
use nom::branch::alt;
use nom::bytes::complete::is_a;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::character::complete::alphanumeric1;
use nom::character::complete::line_ending;
use nom::character::complete::multispace0;
use nom::character::complete::space0;
use nom::character::complete::space1;
use nom::combinator::not;
use nom::combinator::opt;
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
pub struct ImageShorthandV42 {
    pub attrs: BTreeMap<String, Vec<SpanV42>>,
    pub flags: Vec<String>,
    pub kind: String,
    pub text: String,
}

pub fn image_shorthand_span(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, tokens) = preceded(pair(tag("{{"), multispace0), many1(alt((
            plain_text_string_base,
            plain_text_space1_as_single_space,
            is_a("~*^![]{}<>_#:"),
            plain_text_single_line_ending_as_space,
            escaped_character,
        ))))
        .context("")
        .parse(source)?;
    let (source, (flags, attrs)) = image_shorthand_flags_and_attrs
        .context("")
        .parse(source)?;
    let (source, _) = tag("}}").context("").parse(source)?;
    Ok((
        source,
        SpanV42::ImageShorthand(ImageShorthandV42 {
            kind: "image-shorthand".to_string(),
            attrs,
            flags,
            text: tokens.join(""),
        }),
    ))
}

pub fn image_shorthand_flags_and_attrs(
    source: &str,
) -> IResult<&str, (Vec<String>, BTreeMap<String, Vec<SpanV42>>), ErrorTree<&str>> {
    // Reminder: attrs first otherwise things go wrong with this setup
    let (source, flags_and_attrs) = many0(alt((
        image_shorthand_attr,
        image_shorthand_flag,
    )))
    .context("")
    .parse(source)?;
    let mut attrs = BTreeMap::new();
    let mut flags = vec![];
    flags_and_attrs.iter().for_each(|af| match &af {
        SpanBaseAttrV42::Flag(f) => flags.push(f.text.to_string()),
        SpanBaseAttrV42::KeyValue{ key, spans } => {
            attrs.insert(key.to_string(), spans.clone());
            ()
        }
    });
    Ok((source, (flags, attrs)))
}

pub fn image_shorthand_flag(source: &str) -> IResult<&str, SpanBaseAttrV42, ErrorTree<&str>> {
    let (source, _) = tag("|").context("").parse(source)?;
    let (source, _) = space0.context("").parse(source)?;
    let (source, _) = opt(line_ending).context("").parse(source)?;
    let (source, _) = space0.context("").parse(source)?;
    let (source, parts) = many1(alt((
        plain_text_string_base,
        is_a("~*^![]{}<>_#:"),
        escaped_character,
    )))
    .context("")
    .parse(source)?;
    let (source, _) = space0.context("").parse(source)?;
    let (source, _) = opt(line_ending).context("").parse(source)?;
    let (source, _) = space0.context("").parse(source)?;
    Ok((
        source,
        SpanBaseAttrV42::Flag(SpanFlagAttrV42 {
            text: parts.join(""),
        }),
    ))
}


// TODO: Deprecate this
pub fn not_close_brace_or_pipe_or_escape(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, result) = is_not("}\\|").context("").parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            kind: "plain-text".to_string(),
            text: result.to_string(),
        }),
    ))
}

// TODO: Deprecate this
pub fn image_shorthand_span_flag_attr(
    source: &str,
) -> IResult<&str, SpanBaseAttrV42, ErrorTree<&str>> {
    let (source, spans) = many1(alt((
        not_close_brace_or_pipe_or_escape,
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

pub fn image_shorthand_attr(
    source: &str,
) -> IResult<&str, SpanBaseAttrV42, ErrorTree<&str>> {
    let (source, _) = tag("|").context("").parse(source)?;
    let (source, _) = space0.context("").parse(source)?;
    let (source, _) = opt(line_ending).context("").parse(source)?;
    let (source, _) = space0.context("").parse(source)?;
    let (source, key_spans) = terminated(
        many1(alt((alphanumeric1, is_a("-_")))),
        pair(tag(":"), space1),
    )
    .context("")
    .parse(source)?;
    let (source, spans) = many1(alt((
        span_of_plain_text_for_shorthand_attr_value, 
        named_span,
        span_of_escaped_character,
        image_span_of_extra_shorthand_attr_value,
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

pub fn image_span_of_extra_shorthand_attr_value (
    source: &str,
) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, result) = is_a("~*^![]{}<>_#:")
    .context("")
    .parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            kind: "plain-text".to_string(),
            text: result.to_string(),
        }),
    ))
}


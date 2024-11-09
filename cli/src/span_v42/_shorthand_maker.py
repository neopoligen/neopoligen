#!/usr/bin/env python3

from string import Template

# Reminder: don't do named_span or link-shorthand in this
# file as they work a little differently. 

shorthands = [
        ("~~", "~~", "strikethrough", "Strikethrough", "tilde", "~"),
        ("``", "``", "code", "Code", "backtick", "`"),
        ("__", "__", "emphasis", "Emphasis", "asterisk", "_"),
        ("^^", "^^", "footnote", "Footnote", "caret", "^"),
        ("**", "**", "strong", "Strong", "exclamation", "*"),
        ("[[", "]]", "html", "Html", "close_bracket", "]"),
        ("{{", "}}", "image", "Image", "close_brace", "}"),
        ("!!", "!!", "underline", "Underline", "underscore", "!"),
        ("@@", "@@", "mark", "Mark", "ampersand", "@"),
        ]

skeleton = """
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
pub struct ${NAME2}ShorthandV42 {
    pub attrs: BTreeMap<String, Vec<SpanV42>>,
    pub flags: Vec<String>,
    pub kind: String,
    pub text: String,
}

pub fn ${NAME1}_shorthand_span(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, tokens) = preceded(pair(tag("${OPEN}"), multispace0), many1(alt((
            plain_text_string_base,
            plain_text_space1_as_single_space,
            is_a("~*^![]{}<>_#:"),
            plain_text_single_newline_as_space,
            escaped_character,
        ))))
        .context("")
        .parse(source)?;
    let (source, (flags, attrs)) = ${NAME1}_shorthand_flags_and_attrs
        .context("")
        .parse(source)?;
    let (source, _) = tag("${CLOSE}").context("").parse(source)?;
    Ok((
        source,
        SpanV42::${NAME2}Shorthand(${NAME2}ShorthandV42 {
            kind: "${NAME1}-shorthand".to_string(),
            attrs,
            flags,
            text: tokens.join(""),
        }),
    ))
}

pub fn ${NAME1}_shorthand_flags_and_attrs(
    source: &str,
) -> IResult<&str, (Vec<String>, BTreeMap<String, Vec<SpanV42>>), ErrorTree<&str>> {
    // Reminder: check for attrs first otherwise things go wrong
    // with this setup
    let (source, flags_and_attrs) = many0(alt((
        ${NAME1}_shorthand_attr,
        ${NAME1}_shorthand_flag,
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

pub fn ${NAME1}_shorthand_flag(source: &str) -> IResult<&str, SpanBaseAttrV42, ErrorTree<&str>> {
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
pub fn not_${CHARNAME1}_or_pipe_or_escape(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, result) = is_not("${CHAR}\\\\|").context("").parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            kind: "plain-text".to_string(),
            text: result.to_string(),
        }),
    ))
}

// TODO: Deprecate this
pub fn ${NAME1}_shorthand_span_flag_attr(
    source: &str,
) -> IResult<&str, SpanBaseAttrV42, ErrorTree<&str>> {
    let (source, spans) = many1(alt((
        not_${CHARNAME1}_or_pipe_or_escape,
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

pub fn ${NAME1}_shorthand_attr(
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
        span_of_shorthand_attr_value,
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
"""


for shorthand in shorthands:
    with open(f"shorthand_{shorthand[2]}.rs", "w") as out:
        data = { 
                "OPEN": shorthand[0], 
                "CLOSE": shorthand[1], 
                "NAME1": shorthand[2], 
                "NAME2": shorthand[3],
                "CHARNAME1": shorthand[4],
                "CHAR": shorthand[5],
                }
        template = Template(skeleton)
        output = template.substitute(data)
        out.write(output)


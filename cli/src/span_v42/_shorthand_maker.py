#!/usr/bin/env python3

from string import Template

# Reminder: don't do named_span or link-shorthand in this
# file as they work a little differently. 

shorthands = [
        ("~~", "~~", "strikethrough", "Strikethrough", "tilde", "~"),
        ("``", "``", "code", "Code", "backtick", "`"),
        ("**", "**", "emphasis", "Emphasis", "asterisk", "*"),
        ("^^", "^^", "footnote", "Footnote", "caret", "^"),
        ("!!", "!!", "strong", "Strong", "exclamation", "!"),
        ("[[", "]]", "html", "Html", "close_bracket", "]"),
        ("{{", "}}", "image", "Image", "close_brace", "}"),
        ("__", "__", "underline", "Underline", "underscore", "_"),
        ("@@", "@@", "mark", "Mark", "ampersand", "@"),
        ]

skeleton = """use super::single_characters::*;
use super::PlainTextSpan;
use crate::helpers::flatten_spans_v42;
use crate::span_attrs_v42::*;
use crate::span_v42::escaped_characters::*;
use crate::span_v42::SpanV42;
use nom::branch::alt;
use nom::bytes::complete::is_a;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::character::complete::alphanumeric1;
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
pub struct ${NAME2}ShorthandV42 {
    attrs: BTreeMap<String, Vec<SpanV42>>,
    flags: Vec<String>,
    text: String,
}

pub fn ${NAME1}_shorthand_span(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, tokens) = preceded(pair(tag("${OPEN}"), space0), many1(${NAME1}_shorthand_content))
        .context("")
        .parse(source)?;
    let (source, (attrs, flags)) = get_${NAME1}_shorthand_attrs_and_flags
        .context("")
        .parse(source)?;
    let (source, _) = tag("${CLOSE}").context("").parse(source)?;
    Ok((
        source,
        SpanV42::${NAME2}Shorthand(${NAME2}ShorthandV42 {
            attrs,
            flags,
            text: flatten_spans_v42(&tokens),
        }),
    ))
}

pub fn get_${NAME1}_shorthand_attrs_and_flags(
    source: &str,
) -> IResult<&str, (BTreeMap<String, Vec<SpanV42>>, Vec<String>), ErrorTree<&str>> {
    let (source, attrs_and_flags) = many0(${NAME1}_shorthand_attr_or_flag)
        .context("")
        .parse(source)?;
    let mut attrs = BTreeMap::new();
    let flags = attrs_and_flags
        .iter()
        .filter_map(|af| match af {
            SpanBaseAttrV42::Flag(f) => Some(f.text.clone()),
            SpanBaseAttrV42::KeyValue { key, spans } => {
                attrs.insert(key.to_string(), spans.clone());
                None
            }
        })
        .collect::<Vec<String>>();
    Ok((source, (attrs, flags)))
}

pub fn ${NAME1}_shorthand_attr_or_flag(
    source: &str,
) -> IResult<&str, SpanBaseAttrV42, ErrorTree<&str>> {
    let (source, _) = tag("|").context("").parse(source)?;
    // Reminder: Make sure to look for key_value first so flags work properly
    let (source, attr_or_flag) = alt((
        ${NAME1}_shorthand_key_value_attr,
        ${NAME1}_shorthand_span_flag_attr,
    ))
    .context("")
    .parse(source)?;
    Ok((source, attr_or_flag))
}

pub fn ${NAME1}_shorthand_content(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, token) = alt((
        not_${CHARNAME1}_or_pipe_or_escape,
        single_${CHARNAME1},
        escaped_${CHARNAME1},
        escaped_pipe,
        escaped_backslash,
    ))
    .context("")
    .parse(source)?;
    Ok((source, token))
}

pub fn not_${CHARNAME1}_or_pipe_or_escape(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, result) = is_not("${CHAR}\\\\|").context("").parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: result.to_string(),
        }),
    ))
}

pub fn ${NAME1}_shorthand_span_flag_attr(
    source: &str,
) -> IResult<&str, SpanBaseAttrV42, ErrorTree<&str>> {
    let (source, spans) = many1(alt((
        not_${CHARNAME1}_or_pipe_or_escape,
        escaped_pipe,
        escaped_${CHARNAME1},
        single_${CHARNAME1},
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

pub fn ${NAME1}_shorthand_key_value_attr(
    source: &str,
) -> IResult<&str, SpanBaseAttrV42, ErrorTree<&str>> {
    let (source, key_spans) = terminated(
        many1(alt((alphanumeric1, is_a("-_")))),
        pair(tag(":"), space1),
    )
    .context("")
    .parse(source)?;
    let (source, spans) = many1(alt((
        not_${CHARNAME1}_or_pipe_or_escape,
        single_${CHARNAME1},
        escaped_${CHARNAME1},
        escaped_pipe,
        escaped_backslash,
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

#[cfg(test)]
mod test {
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


use crate::span::*;
use nom::branch::alt;
// use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
// use nom::character::complete::line_ending;
// use nom::character::complete::multispace0;
use nom::character::complete::space0;
// use nom::character::complete::space1;
// use nom::combinator::not;
use nom::multi::many1;
// use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;
// use serde::Deserialize;
// use serde::Serialize;
use std::collections::BTreeMap;

pub fn known_span<'a>(
    source: &'a str,
    spans: &'a Vec<String>,
) -> IResult<&'a str, Span, ErrorTree<&'a str>> {
    let (source, _) = tag("<<").context("").parse(source)?;
    let (source, _) = space0.context("").parse(source)?;
    let (source, r#type) = (|src| known_span_type(src, spans))
        .context("")
        .parse(source)?;
    let (source, _) = tag("|").context("").parse(source)?;
    let (source, spans) = many1(|src| span_finder(src, spans))
        .context("")
        .parse(source)?;
    // Make sure to look for key_value first so that ":" can
    // be used in URLs
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
        Span::KnownSpan {
            r#type: r#type.to_string(),
            spans,
            flags,
            attrs,
        },
    ))
}

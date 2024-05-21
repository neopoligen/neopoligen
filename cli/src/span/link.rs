use crate::span::*;
use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::multi::many0;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;
use std::collections::BTreeMap;
use std::collections::VecDeque;

// TODO: set this up to output to a regular link
pub fn link_shorthand(source: &str) -> IResult<&str, Span, ErrorTree<&str>> {
    let (source, _) = tag(">>").context("").parse(source)?;
    let (source, _text) = is_not(">|").context("").parse(source)?;
    let (source, raw_attrs) = many0(alt((
        link_shorthand_key_value_attr,
        link_shorthand_flag_attr,
    )))
    .context("")
    .parse(source)?;
    let (source, _) = tag(">>").context("").parse(source)?;
    let mut flags: VecDeque<String> = VecDeque::new();
    let mut attrs = BTreeMap::new();
    raw_attrs.iter().for_each(|attr| match attr {
        SpanAttr::KeyValue { key, value } => {
            attrs.insert(key.to_string(), value.to_string());
        }
        SpanAttr::Flag { key } => flags.push_back(key.to_string()),
    });

    // let href = if flags.len() > 0 {
    //     Some(flags.pop_front().unwrap().clone())
    // } else {
    //     None
    // };

    Ok((
        source,
        Span::KnownSpan {
            attrs,
            flags: flags.into(),
            spans: vec![],
            r#type: "link".to_string(),
        },
    ))
}

pub fn link_shorthand_key_value_attr(source: &str) -> IResult<&str, SpanAttr, ErrorTree<&str>> {
    let (source, _) = tag("|").context("").parse(source)?;
    let (source, key) = is_not(" |\n\t:>").context("").parse(source)?;
    let (source, _) = tag(":").context("").parse(source)?;
    let (source, value) = is_not("|>").context("").parse(source)?;
    Ok((
        source,
        SpanAttr::KeyValue {
            key: key.trim().to_string(),
            value: value.trim().to_string(),
        },
    ))
}

pub fn link_shorthand_flag_attr(source: &str) -> IResult<&str, SpanAttr, ErrorTree<&str>> {
    let (source, _) = tag("|").context("").parse(source)?;
    let (source, key) = is_not(" |\n\t:>").context("").parse(source)?;
    Ok((
        source,
        SpanAttr::Flag {
            key: key.trim().to_string(),
        },
    ))
}

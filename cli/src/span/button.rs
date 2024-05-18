use crate::span::*;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::is_not;
use nom::multi::many0;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;
use std::collections::BTreeMap;

pub fn button_shorthand(source: &str) -> IResult<&str, Span, ErrorTree<&str>> {
    let (source, _) = tag("~~").context("").parse(source)?;
    let (source, text) = is_not("~|").context("").parse(source)?;
    let (source, raw_attrs) = many0(alt((
        button_shorthand_key_value_attr,
        button_shorthand_flag_attr,
    )))
    .context("")
    .parse(source)?;
    let (source, _) = tag("~~").context("").parse(source)?;
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
        Span::Button {
            attrs,
            flags,
            text: text.to_string(),
            r#type: "button".to_string(),
        },
    ))
}

pub fn button_shorthand_key_value_attr(source: &str) -> IResult<&str, SpanAttr, ErrorTree<&str>> {
    let (source, _) = tag("|").context("").parse(source)?;
    let (source, key) = is_not(" |\n\t:~").context("").parse(source)?;
    let (source, _) = tag(":").context("").parse(source)?;
    let (source, value) = is_not("|~").context("").parse(source)?;
    Ok((
        source,
        SpanAttr::KeyValue {
            key: key.trim().to_string(),
            value: value.trim().to_string(),
        },
    ))
}

pub fn button_shorthand_flag_attr(source: &str) -> IResult<&str, SpanAttr, ErrorTree<&str>> {
    let (source, _) = tag("|").context("").parse(source)?;
    let (source, key) = is_not(" |\n\t:~").context("").parse(source)?;
    Ok((
        source,
        SpanAttr::Flag {
            key: key.trim().to_string(),
        },
    ))
}

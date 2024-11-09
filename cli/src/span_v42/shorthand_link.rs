use super::PlainTextSpan;
use crate::helpers::flatten_spans_v42;
use crate::span_attrs_v42::*;
use crate::span_v42::SpanV42;
use nom::branch::alt;
use nom::bytes::complete::is_a;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::character::complete::alphanumeric1;
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
pub struct LinkShorthandV42 {
    attrs: BTreeMap<String, Vec<SpanV42>>,
    flags: Vec<String>,
    text: String,
    url: String,
}

pub fn link_shorthand_span(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, tokens1) = preceded(pair(tag(">>"), space0), many1(link_shorthand_content))
        .context("")
        .parse(source)?;
    let (source, tokens2) = opt(preceded(
        pair(tag("|"), space0),
        many1(link_shorthand_content),
    ))
    .context("")
    .parse(source)?;
    let (source, (attrs, flags)) = get_link_shorthand_attrs_and_flags
        .context("")
        .parse(source)?;
    let (source, _) = tag(">>").context("").parse(source)?;

    match tokens2 {
        Some(url_tokens) => Ok((
            source,
            SpanV42::LinkShorthand(LinkShorthandV42 {
                attrs,
                flags,
                text: flatten_spans_v42(&tokens1),
                url: flatten_spans_v42(&url_tokens),
            }),
        )),
        None => Ok((
            source,
            SpanV42::LinkShorthand(LinkShorthandV42 {
                attrs,
                flags,
                text: flatten_spans_v42(&tokens1),
                url: flatten_spans_v42(&tokens1),
            }),
        )),
    }

    //
}

pub fn get_link_shorthand_attrs_and_flags(
    source: &str,
) -> IResult<&str, (BTreeMap<String, Vec<SpanV42>>, Vec<String>), ErrorTree<&str>> {
    let (source, attrs_and_flags) = many0(link_shorthand_attr_or_flag)
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

pub fn link_shorthand_attr_or_flag(
    source: &str,
) -> IResult<&str, SpanBaseAttrV42, ErrorTree<&str>> {
    let (source, _) = tag("|").context("").parse(source)?;
    // Reminder: Make sure to look for key_value first so flags work properly
    let (source, attr_or_flag) =
        alt((link_shorthand_key_value_attr, link_shorthand_span_flag_attr))
            .context("")
            .parse(source)?;
    Ok((source, attr_or_flag))
}

pub fn link_shorthand_content(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, token) = alt((not_greater_than_or_pipe_or_escape,))
        .context("")
        .parse(source)?;
    Ok((source, token))
}

pub fn not_greater_than_or_pipe_or_escape(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, result) = is_not(">\\|").context("").parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            kind: "plain-text".to_string(),
            text: result.to_string(),
        }),
    ))
}

pub fn link_shorthand_span_flag_attr(
    source: &str,
) -> IResult<&str, SpanBaseAttrV42, ErrorTree<&str>> {
    let (source, spans) = many1(alt((not_greater_than_or_pipe_or_escape,)))
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

pub fn link_shorthand_key_value_attr(
    source: &str,
) -> IResult<&str, SpanBaseAttrV42, ErrorTree<&str>> {
    let (source, key_spans) = terminated(
        many1(alt((alphanumeric1, is_a("-_")))),
        pair(tag(":"), space1),
    )
    .context("")
    .parse(source)?;
    let (source, spans) = many1(alt((not_greater_than_or_pipe_or_escape,)))
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
    use super::*;
    use crate::span_v42::SpanV42;
    use pretty_assertions::assert_eq;

    #[test]
    fn link_shorthand_basic_test() {
        let source = ">>text to link|https://www.example.com/>>";
        let left = SpanV42::LinkShorthand(LinkShorthandV42 {
            attrs: BTreeMap::new(),
            flags: vec![],
            text: "text to link".to_string(),
            url: "https://www.example.com/".to_string(),
        });
        let right = link_shorthand_span(source).unwrap().1;
        assert_eq!(left, right);
    }

    #[test]
    fn link_shorthand_basic_test_with_attrs_and_flags() {
        let source = ">>text to link|https://www.example.com/|class: alfa|bravo>>";
        let mut attrs: BTreeMap<String, Vec<SpanV42>> = BTreeMap::new();
        attrs.insert(
            "class".to_string(),
            vec![SpanV42::PlainText(PlainTextSpan {
                kind: "plain-text".to_string(),
                text: "alfa".to_string(),
            })],
        );
        let left = SpanV42::LinkShorthand(LinkShorthandV42 {
            attrs,
            flags: vec!["bravo".to_string()],
            text: "text to link".to_string(),
            url: "https://www.example.com/".to_string(),
        });
        let right = link_shorthand_span(source).unwrap().1;
        assert_eq!(left, right);
    }

    #[test]
    fn link_shorthand_url_only() {
        let source = ">>https://www.example.com/>>";
        let left = SpanV42::LinkShorthand(LinkShorthandV42 {
            attrs: BTreeMap::new(),
            flags: vec![],
            text: "https://www.example.com/".to_string(),
            url: "https://www.example.com/".to_string(),
        });
        let right = link_shorthand_span(source).unwrap().1;
        assert_eq!(left, right);
    }
}

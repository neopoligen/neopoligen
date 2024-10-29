use super::not_pipe_or_greater_than::not_pipe_or_greater_than;
use super::plaintext_without_spaces::plaintext_without_spaces;
use crate::helpers::flatten_spans_v42;
use crate::span_v42::any_colons::any_colons;
use crate::span_v42::any_hyphens::any_hyphens;
use crate::span_v42::any_underscores::any_underscores;
use crate::span_v42::plaintext::plaintext;
use crate::span_v42::single_line_ending::single_line_ending;
use crate::span_v42::SpanV42;
use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::is_a;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::character::complete::alphanumeric1;
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
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

// TODO: move this to its own file
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct KeyValueSpanAttrV42 {
    key: String,
    value: Vec<SpanV42>,
    text: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct NamedSpanKeyValueAttrPayloadV42 {
    value: Vec<SpanV42>,
    text: String,
}

// TODO: move this to its own file
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct FlagSpanAttrV42 {
    value: String,
}

// TODO: move this to its own file
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum NamedSpanAttrV42 {
    KeyValue(KeyValueSpanAttrV42),
    Flag(FlagSpanAttrV42),
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct NamedSpanV42 {
    attrs: BTreeMap<String, NamedSpanKeyValueAttrPayloadV42>,
    flags: Vec<String>,
    kind: String,
    spans: Option<Vec<SpanV42>>,
    text: Option<String>,
}

pub fn named_span(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, tokens) = preceded(
        pair(tag("<<"), multispace0),
        many1(alt((alphanumeric1, is_a("-_")))),
    )
    .context("")
    .parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    let (source, _) = not(tuple((tag("|"), multispace0, tag(">"))))
        .context("")
        .parse(source)?;
    let (source, spans) = opt(preceded(tag("|"), many0(named_span_content_span)))
        .context("")
        .parse(source)?;
    let (source, (attrs, flags)) = named_span_attrs_and_flags.context("").parse(source)?;
    let (source, _) = tag(">>").context("").parse(source)?;
    let kind = tokens.iter().join("");
    let text = match &spans {
        Some(spns) => Some(flatten_spans_v42(spns)),
        None => None,
    };
    Ok((
        source,
        SpanV42::NamedSpan(NamedSpanV42 {
            attrs,
            flags,
            kind,
            spans,
            text,
        }),
    ))
}

pub fn named_span_attrs_and_flags(
    source: &str,
) -> IResult<
    &str,
    (
        BTreeMap<String, NamedSpanKeyValueAttrPayloadV42>,
        Vec<String>,
    ),
    ErrorTree<&str>,
> {
    let (source, attr_flags) = many0(named_span_attr_flag).context("").parse(source)?;
    let mut attrs = BTreeMap::new();
    let flags = attr_flags
        .iter()
        .filter_map(|af| match af {
            NamedSpanAttrV42::Flag(f) => Some(f.value.clone()),
            NamedSpanAttrV42::KeyValue(f) => {
                attrs.insert(
                    f.key.to_string(),
                    NamedSpanKeyValueAttrPayloadV42 {
                        value: f.value.clone(),
                        text: f.text.clone(),
                    },
                );
                None
            }
        })
        .collect::<Vec<String>>();
    Ok((source, (attrs, flags)))
}

pub fn named_span_attr_flag(source: &str) -> IResult<&str, NamedSpanAttrV42, ErrorTree<&str>> {
    let (source, _) = tag("|").context("").parse(source)?;
    let (source, attr) = alt((named_span_key_value_attr, named_span_flag_attr))
        .context("")
        .parse(source)?;
    Ok((source, attr))
}

pub fn named_span_key_value_attr(source: &str) -> IResult<&str, NamedSpanAttrV42, ErrorTree<&str>> {
    let (source, key_spans) = terminated(many1(alt((plaintext,))), pair(tag(":"), space1))
        .context("")
        .parse(source)?;
    let (source, spans) = many1(alt((plaintext,))).context("").parse(source)?;
    Ok((
        source,
        NamedSpanAttrV42::KeyValue(KeyValueSpanAttrV42 {
            key: flatten_spans_v42(&key_spans),
            value: spans,
            text: "".to_string(),
        }),
    ))
}

pub fn named_span_flag_attr(source: &str) -> IResult<&str, NamedSpanAttrV42, ErrorTree<&str>> {
    let (source, spans) = many1(alt((
        plaintext_without_spaces,
        any_hyphens,
        any_underscores,
        any_colons,
        single_line_ending,
    )))
    .context("")
    .parse(source)?;
    let (source, _) = not(tag(":")).context("").parse(source)?;
    Ok((
        source,
        NamedSpanAttrV42::Flag(FlagSpanAttrV42 {
            value: flatten_spans_v42(&spans),
        }),
    ))
}

pub fn named_span_content_span(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, span) = alt((not_pipe_or_greater_than,)).context("").parse(source)?;
    //let (source, span) = alt((is_not("|>"),)).context("").parse(source)?;
    Ok((source, span))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::span_v42::PlainTextSpan;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[test]
    fn basic_test() {
        let source = "<<sierra|tango>>";
        let left = (
            "",
            SpanV42::NamedSpan(NamedSpanV42 {
                attrs: BTreeMap::new(),
                flags: vec![],
                kind: "sierra".to_string(),
                spans: Some(vec![SpanV42::PlainText(PlainTextSpan {
                    text: "tango".to_string(),
                })]),
                text: Some("tango".to_string()),
            }),
        );
        let right = named_span(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn with_flag() {
        let source = "<<delta|echo|foxtrot>>";
        let left = (
            "",
            SpanV42::NamedSpan(NamedSpanV42 {
                attrs: BTreeMap::new(),
                flags: vec!["foxtrot".to_string()],
                kind: "delta".to_string(),
                spans: Some(vec![SpanV42::PlainText(PlainTextSpan {
                    text: "echo".to_string(),
                })]),
                text: Some("echo".to_string()),
            }),
        );
        let right = named_span(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn with_key_value_attr() {
        let source = "<<delta|echo|class: red>>";
        let mut attrs = BTreeMap::new();
        attrs.insert(
            "class".to_string(),
            NamedSpanKeyValueAttrPayloadV42 {
                value: vec![SpanV42::PlainText(PlainTextSpan {
                    text: "red".to_string(),
                })],
                text: "".to_string(),
            },
        );
        let left = (
            "",
            SpanV42::NamedSpan(NamedSpanV42 {
                attrs,
                flags: vec![],
                kind: "delta".to_string(),
                spans: Some(vec![SpanV42::PlainText(PlainTextSpan {
                    text: "echo".to_string(),
                })]),
                text: Some("echo".to_string()),
            }),
        );
        let right = named_span(source).unwrap();
        assert_eq!(left, right);
    }

    #[rstest]
    #[case("<<hr>>etc...", "etc...")]
    #[case("<<alfa-bravo>>etc...", "etc...")]
    #[case("<<em|alfa>>etc...", "etc...")]
    #[case("<<em|alfa-bravo>>etc...", "etc...")]
    #[case("<<em|charlie -- delta>>etc...", "etc...")]
    #[case("<<alfa|bravo|charlie>>etc...", "etc...")]
    #[case("<<alfa|bravo|charlie-delta>>etc...", "etc...")]
    #[case("<<alfa|bravo|charlie--delta>>etc...", "etc...")]
    #[case("<<alfa|bravo|charlie_delta>>etc...", "etc...")]
    #[case("<<alfa|bravo|charlie__delta>>etc...", "etc...")]
    #[case("<<link|text|https://www.example.com/?ping=pong>>etc...", "etc...")]
    #[case("<<link|text|\nhttps://www.example.com/?ping=pong>>etc...", "etc...")]
    fn valid_cases_with_remainders(#[case] source: &str, #[case] remainder: &str) {
        match named_span(source) {
            Ok(data) => {
                if data.0 == remainder {
                    assert!(true);
                } else {
                    dbg!(data);
                    assert!(false);
                }
            }
            Err(e) => {
                dbg!(e);
                assert!(false)
            }
        }
    }

    // these are just quick tests, not exhaustive, they are
    // just here to make sure something that shouldn't
    // sneak through is being properly filtered. The
    // only thing that's allowed is hyphens which matches
    // the way HTML works.
    #[rstest]
    #[case(
        "<<hr:>>",
        "colon not allowed to prevent issues wtih file paths for templates"
    )]
    #[case(
        "<<hr/>>",
        "forward slaches not allowed to prevent issues wtih file paths for templates"
    )]
    #[case(
        "<<hr\\>>",
        "back slaches not allowed to prevent issues wtih file paths for templates"
    )]
    fn invalid_cases(#[case] source: &str, #[case] _note: &str) {
        match named_span(source) {
            Ok(data) => {
                dbg!(data);
                assert!(false);
            }
            Err(_e) => {
                assert!(true)
            }
        }
    }
}

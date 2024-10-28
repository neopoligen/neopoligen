use super::plaintext_without_spaces::plaintext_without_spaces;
use crate::helpers::flatten_spans_v42;
use crate::span_v42::any_hyphens::any_hyphens;
use crate::span_v42::any_underscores::any_underscores;
use crate::span_v42::plaintext::plaintext;
use crate::span_v42::SpanV42;
use crate::spans_v42;
use itertools::Itertools;
use minijinja::value::Object;
use minijinja::Error;
use minijinja::Value;
use nom::branch::alt;
use nom::bytes::complete::is_a;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::character::complete::space0;
use nom::combinator::opt;
use nom::multi::many0;
use nom::multi::many1;
use nom::sequence::pair;
use nom::sequence::preceded;
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
    attrs: BTreeMap<String, NamedSpanAttrV42>,
    flags: Vec<String>,
    name: String,
    spans: Option<Vec<SpanV42>>,
}

pub fn named_span(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, tokens) = preceded(pair(tag("<<"), space0), many1(alt((alpha1, is_a("-")))))
        .context("")
        .parse(source)?;
    let (source, spans) = opt(preceded(tag("|"), many0(named_span_content_span)))
        .context("")
        .parse(source)?;
    let (source, (attrs, flags)) = named_span_attrs_and_flags.context("").parse(source)?;
    let (source, _) = tag(">>").context("").parse(source)?;
    let name = tokens.iter().join("");
    Ok((
        source,
        SpanV42::NamedSpan(NamedSpanV42 {
            attrs: BTreeMap::new(),
            flags,
            name,
            spans,
        }),
    ))
}

pub fn named_span_attrs_and_flags(
    source: &str,
) -> IResult<&str, (Vec<NamedSpanAttrV42>, Vec<String>), ErrorTree<&str>> {
    let (source, attr_flags) = many0(named_span_attr_flag).context("").parse(source)?;
    let flags = attr_flags
        .iter()
        .filter_map(|af| match af {
            NamedSpanAttrV42::Flag(f) => Some(f.value.clone()),
            _ => None,
        })
        .collect::<Vec<String>>();

    Ok((source, (vec![], flags)))
}

pub fn named_span_attr_flag(source: &str) -> IResult<&str, NamedSpanAttrV42, ErrorTree<&str>> {
    let (source, _) = tag("|").context("").parse(source)?;
    let (source, attr) = alt((named_span_key_value_attr, named_span_flag_attr))
        .context("")
        .parse(source)?;
    Ok((source, attr))
}

pub fn named_span_key_value_attr(source: &str) -> IResult<&str, NamedSpanAttrV42, ErrorTree<&str>> {
    let (source, attr) = tag("asdf").context("").parse(source)?;
    Ok((
        source,
        NamedSpanAttrV42::KeyValue(KeyValueSpanAttrV42 {
            key: "asdf".to_string(),
            value: vec![],
        }),
    ))
}

pub fn named_span_flag_attr(source: &str) -> IResult<&str, NamedSpanAttrV42, ErrorTree<&str>> {
    let (source, spans) = many1(alt((
        plaintext_without_spaces,
        any_hyphens,
        any_underscores,
    )))
    .context("")
    .parse(source)?;
    Ok((
        source,
        NamedSpanAttrV42::Flag(FlagSpanAttrV42 {
            value: flatten_spans_v42(&spans),
        }),
    ))
}

pub fn named_span_content_span(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, span) = alt((plaintext, any_hyphens)).context("").parse(source)?;
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
                name: "sierra".to_string(),
                spans: Some(vec![SpanV42::PlainText(PlainTextSpan {
                    text: "tango".to_string(),
                })]),
            }),
        );
        let right = named_span(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn solo_with_flag() {
        let source = "<<delta|echo|foxtrot>>";
        let left = (
            "",
            SpanV42::NamedSpan(NamedSpanV42 {
                attrs: BTreeMap::new(),
                flags: vec!["foxtrot".to_string()],
                name: "delta".to_string(),
                spans: Some(vec![SpanV42::PlainText(PlainTextSpan {
                    text: "echo".to_string(),
                })]),
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
            Err(e) => {
                assert!(true)
            }
        }
    }
}

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
pub struct SpanAttrV42 {
    key: String,
    value: Vec<SpanV42>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct NamedSpanV42 {
    attrs: BTreeMap<String, SpanAttrV42>,
    flags: Vec<String>,
    name: String,
    spans: Vec<SpanV42>,
}

pub fn named_span(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, tokens) = preceded(pair(tag("<<"), space0), many1(alt((alpha1, is_a("-")))))
        .context("")
        .parse(source)?;
    let (source, named_span_spans) = many0(named_span_content_span).context("").parse(source)?;
    let (source, _) = tag(">>").context("").parse(source)?;
    let name = tokens.iter().join("");
    Ok((
        source,
        SpanV42::NamedSpan(NamedSpanV42 {
            attrs: BTreeMap::new(),
            flags: vec![],
            name,
            spans: vec![],
        }),
    ))
}

pub fn named_span_content_span(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, _) = tag("|").context("").parse(source)?;
    let (source, span) = plaintext.context("").parse(source)?;
    Ok((source, span))
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[test]
    fn solo_basic_test() {
        let source = "<<sierra|tango>>";
        let left = (
            "",
            SpanV42::NamedSpan(NamedSpanV42 {
                attrs: BTreeMap::new(),
                flags: vec![],
                name: "sierra".to_string(),
                spans: vec![],
            }),
        );
        let right = named_span(source).unwrap();
        assert_eq!(left, right);
    }

    #[rstest]
    #[case("<<hr>>etc...", "etc...")]
    #[case("<<alfa-bravo>>etc...", "etc...")]
    // #[case("<<em|alfa>>etc...", "etc...")]
    fn solo_valid_cases_with_remainders(#[case] source: &str, #[case] remainder: &str) {
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
    fn solo_invalid_cases(#[case] source: &str, #[case] _note: &str) {
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

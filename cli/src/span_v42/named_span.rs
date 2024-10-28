use crate::span_v42::SpanV42;
use minijinja::value::Object;
use minijinja::Error;
use minijinja::Value;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::character::complete::space0;
use nom::sequence::pair;
use nom::sequence::preceded;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct NamedSpan {
    name: String,
    children: Vec<SpanV42>,
}

pub fn named_span(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, name) = preceded(pair(tag("<<"), space0), is_not("> \t\r\n:/\\"))
        .context("")
        .parse(source)?;
    dbg!(source);
    let (source, _) = tag(">>").context("").parse(source)?;
    Ok((
        source,
        SpanV42::NamedSpan(NamedSpan {
            name: name.to_string(),
            children: vec![],
        }),
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case("<<hr>>etc...", "etc...")]
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

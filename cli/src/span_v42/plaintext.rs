use crate::span_v42::SpanV42;
use minijinja::value::Object;
use minijinja::Error;
use minijinja::Value;
use nom::bytes::complete::is_not;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PlainTextSpan {
    pub output: String,
}

pub fn plaintext<'a>(source: &'a str) -> IResult<&'a str, SpanV42, ErrorTree<&'a str>> {
    let (source, output) = is_not("\r\n<>`-|:\\").context("").parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            output: output.to_string(),
        }),
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case("alfa-bravo", "-bravo")]
    fn valid_cases_with_remainders(#[case] source: &str, #[case] remainder: &str) {
        assert_eq!(remainder, plaintext(source).unwrap().0);
    }
}

#![allow(unused_variables)]
#![allow(unused_imports)]

pub mod any_colon;
pub mod any_hyphens;
pub mod block_line_ending;
pub mod named_span;
pub mod plaintext;
pub mod single_exclamation;
pub mod single_hyphen;
pub mod single_line_ending;
pub mod single_underscore;
pub mod space0_line_ending_or_eof;

use self::named_span::NamedSpanV42;
use crate::span_v42::plaintext::*;
use minijinja::value::Object;
use minijinja::Error;
use minijinja::Value;
use nom::branch::alt;
use nom::character::complete::line_ending;
use nom::character::complete::multispace0;
use nom::character::complete::space0;
use nom::character::complete::space1;
use nom::combinator::eof;
use nom::combinator::not;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum SpanV42 {
    PlainText(PlainTextSpan),
    NamedSpan(NamedSpanV42),
}

pub fn span_v42<'a>(source: &'a str) -> IResult<&'a str, SpanV42, ErrorTree<&'a str>> {
    let (source, span) = alt((plaintext,)).context("").parse(source)?;
    Ok((source, span))
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

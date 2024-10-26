#![allow(unused_variables)]
#![allow(unused_imports)]

pub mod any_hyphen;
pub mod plaintext;
pub mod single_line_ending;
pub mod space0_line_ending_or_eof;

// use crate::helpers::flatten_spans;

// use crate::span::code_shorthand::*;
// use crate::span::code_shorthand_single_pipe::*;
// use crate::span::colon::*;
// use crate::span::colon_not_followed_by_space::*;
// use crate::span::em_shorthand::*;
// use crate::span::escaped_backslash::*;
// use crate::span::escaped_backtick::*;
// use crate::span::escaped_caret::*;
// use crate::span::escaped_colon::*;
// use crate::span::escaped_greaterthan::*;
// use crate::span::escaped_hyphen::*;
// use crate::span::escaped_lessthan::*;
// use crate::span::escaped_pipe::*;
// use crate::span::escaped_underscore::*;
// use crate::span::footnote_shorthand::*;
// use crate::span::hyphen::*;
// use crate::span::hyphen_with_trailing_non_hyphen::*;
// use crate::span::more_than_two_carets::*;
// use crate::span::more_than_two_underscores::*;
// use crate::span::named_span::*;
// use crate::span::single_backtick::*;
// use crate::span::single_caret::*;
// use crate::span::single_greaterthan::*;
//  use crate::span::single_hyphen::*;
// use crate::span::single_lessthan::*;
// use crate::span::single_underscore::*;
//  use crate::span::wordpart::*;

// use crate::span_attr::*;
use minijinja::value::Object;
use minijinja::Error;
use minijinja::Value;
use nom::branch::alt;
//use nom::bytes::complete::is_not;
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

use crate::span_v42::plaintext::*;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum SpanV42 {
    PlainText(PlainTextSpan),
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

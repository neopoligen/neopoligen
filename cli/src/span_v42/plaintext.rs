// use crate::span_attr::*;
use minijinja::value::Object;
use minijinja::Error;
use minijinja::Value;
use nom::branch::alt;
//use nom::bytes::complete::is_not;
use crate::span_v42::SpanV42;
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

pub struct PlainTextSpan {
    pub text: String,
}

pub fn plaintext<'a>(source: &'a str) -> IResult<&'a str, SpanV42, ErrorTree<&'a str>> {
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: "asdf".to_string(),
        }),
    ))
}

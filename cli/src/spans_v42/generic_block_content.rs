#![allow(unused_imports)]
use crate::span_v42::plaintext::plaintext;
use crate::span_v42::SpanV42;
use nom::branch::alt;
use nom::character::complete::multispace0;
use nom::multi::many1;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::ParserExt;

pub fn spans_of_generic_block_content<'a>(
    source: &'a str,
) -> IResult<&'a str, Vec<SpanV42>, ErrorTree<&'a str>> {
    let (source, output) = many1(alt((plaintext,))).context("").parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    Ok((source, output))
}

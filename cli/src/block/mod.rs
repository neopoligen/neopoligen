use crate::span::*;
// use nom::branch::alt;
// use nom::bytes::complete::is_not;
// use nom::character::complete::space1;
use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::combinator::not;
use nom::multi::many1;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Block {
    Paragraph { spans: Vec<Span> },
}

pub fn block(source: &str) -> IResult<&str, Block, ErrorTree<&str>> {
    let (source, _) = not(tag("--")).context("block").parse(source)?;
    let (source, result) = many1(span).context("block").parse(source)?;
    let (source, _) = multispace0.context("block").parse(source)?;
    Ok((source, Block::Paragraph { spans: result }))
}

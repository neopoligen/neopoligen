use crate::section::*;
use nom::multi::many1;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;

pub fn ast(source: &str) -> IResult<&str, Vec<Section>, ErrorTree<&str>> {
    let (source, result) = many1(section).context("page").parse(source)?;
    Ok((source, result))
}

use crate::page_error::PageError;
use crate::section::*;
// use nom::branch::alt;
// use nom::bytes::complete::is_not;
// use nom::character::complete::line_ending;
// use nom::character::complete::space0;
// use nom::character::complete::space1;
// use nom::combinator::not;
use nom::multi::many1;
// use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;
use serde::Serialize;
use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub struct Page {
    content: Vec<Section>,
    id: String,
    raw_source: String,
    errors: Vec<PageError>,
    tags: Vec<String>,
    source_path: PathBuf,
    output_path: PathBuf,
    folders: Vec<String>,
}

pub fn page(source: &str) -> IResult<&str, Vec<Section>, ErrorTree<&str>> {
    let (source, result) = many1(section).context("page").parse(source)?;
    Ok((source, result))
}

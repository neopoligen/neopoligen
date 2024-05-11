use crate::section::Section;
use crate::span::*;
use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::combinator::eof;
use nom::combinator::not;
use nom::multi::many0;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;

pub fn block_of_anything<'a>(
    source: &'a str,
    spans: &'a Vec<String>,
) -> IResult<&'a str, Section, ErrorTree<&'a str>> {
    let (source, _) = not(eof).context("").parse(source)?;
    let (source, _) = not(tag("--")).context("").parse(source)?;
    let (source, spans) = many0(|src| span_finder(src, spans))
        .context("")
        .parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    Ok((source, Section::Block { spans }))
}

pub fn block_of_end_content<'a>(
    source: &'a str,
    spans: &'a Vec<String>,
) -> IResult<&'a str, Section, ErrorTree<&'a str>> {
    let (source, _) = not(eof).context("").parse(source)?;
    let (source, _) = not(tag("-")).context("").parse(source)?;
    let (source, _) = not(tag("[")).context("").parse(source)?;
    let (source, spans) = many0(|src| span_finder(src, spans))
        .context("")
        .parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    Ok((source, Section::Block { spans }))
}

use crate::section::*;
use crate::site_sections::SiteSections;
use nom::multi::many1;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;

pub fn ast<'a>(
    source: &'a str,
    sections: &'a SiteSections,
) -> IResult<&'a str, Vec<Section>, ErrorTree<&'a str>> {
    let (source, result) = many1(|src| section(src, sections))
        .context("page")
        .parse(source)?;
    Ok((source, result))
}

use crate::span::*;
use nom::bytes::complete::take_until;
use nom::bytes::complete::tag;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;

pub fn html_shorthand(source: &str) -> IResult<&str, Span, ErrorTree<&str>> {
    let (source, _) = tag("((").context("").parse(source)?;
    let (source, text) = take_until("))").context("").parse(source)?;
    let (source, _) = tag("))").context("").parse(source)?;
    Ok((
        source,
        Span::Html {
            text: text.to_string(),
            r#type: "html".to_string(),
        },
    ))
}


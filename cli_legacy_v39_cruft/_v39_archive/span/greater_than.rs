use crate::span::Span;
use nom::bytes::complete::tag;
use nom::combinator::not;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;

pub fn greater_than(source: &str) -> IResult<&str, Span, ErrorTree<&str>> {
    let (source, text) = tag(">").context("").parse(source)?;
    let (source, _) = not(tag(">")).context("").parse(source)?;
    Ok((
        source,
        Span::RawText {
            text: text.to_string(),
            r#type: "greater-than".to_string(),
        },
    ))
}

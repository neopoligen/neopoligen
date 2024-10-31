use crate::span::*;
use nom::bytes::complete::is_not;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;

pub fn wordpart(source: &str) -> IResult<&str, Span, ErrorTree<&str>> {
    let (source, text) = is_not("\n|\\`:<>-_^").context("").parse(source)?;
    Ok((
        source,
        Span {
            attrs: vec![],
            parsed_text: text.to_string(),
            kind: SpanKind::WordPart,
        },
    ))
}

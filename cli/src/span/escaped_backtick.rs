use crate::span::*;
use nom::bytes::complete::tag;
use nom::combinator::not;
use nom::sequence::pair;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;

pub fn escaped_backtick(source: &str) -> IResult<&str, Span, ErrorTree<&str>> {
    // Reminder: not doing the source/replace here because the
    // escape is captured by the slash. There's certainly
    // a way around that, but this is fine
    let (source, _) = pair(tag("\\"), tag("`")).context("").parse(source)?;
    Ok((
        source,
        Span {
            attrs: vec![],
            source_text: "\\`".to_string(),
            parsed_text: "`".to_string(),
            kind: SpanKind::EscapedBacktick,
        },
    ))
}

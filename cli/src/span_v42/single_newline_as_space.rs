use crate::span_v42::*;
use nom::bytes::complete::tag;
use nom::sequence::terminated;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;

pub fn single_newline_as_space(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, _) = terminated(tag("\n"), not(tag("\n")))
        .context("")
        .parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: " ".to_string(),
        }),
    ))
}

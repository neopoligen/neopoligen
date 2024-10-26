use crate::span_v42::plaintext::plaintext;
use crate::span_v42::SpanV42;
use nom::multi::many1;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::ParserExt;

pub fn spans_of_basic_content<'a>(
    source: &'a str,
) -> IResult<&'a str, Vec<SpanV42>, ErrorTree<&'a str>> {
    let (source, output) = many1(plaintext).context("").parse(source)?;
    Ok((source, output))
}

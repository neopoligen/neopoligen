use crate::span_v42::any_colon::any_colon;
use crate::span_v42::any_hyphen::any_hyphens;
use crate::span_v42::plaintext::plaintext;
use crate::span_v42::SpanV42;
use nom::branch::alt;
use nom::multi::many1;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::ParserExt;

pub fn spans_of_section_key_value_attr_value<'a>(
    source: &'a str,
) -> IResult<&'a str, Vec<SpanV42>, ErrorTree<&'a str>> {
    let (source, output) = many1(alt((plaintext, any_colon, any_hyphens)))
        .context("")
        .parse(source)?;
    Ok((source, output))
}

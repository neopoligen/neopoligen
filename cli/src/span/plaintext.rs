use crate::span::Span;
use crate::span::SpanKind;
use nom::bytes::complete::is_not;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;

// TODO: Move stuff to this one and any siblings that need
// to be made with it. This includes single hyphens, double
// hyphens, and [ and ] characters since those things will
// be parsed out at the start of blocks.
pub fn plaintext<'a>(source: &'a str) -> IResult<&'a str, Span, ErrorTree<&'a str>> {
    // NOTE: pipe needs to stay here since plaintext is used in shorthands and
    // inside spans.
    let (source, parsed_text) = is_not("\n<^*`_-|\\").context("").parse(source)?;
    let span = Span {
        attrs: vec![],
        parsed_text: parsed_text.to_string(),
        kind: SpanKind::PlainText,
    };
    Ok((source, span))
}

// TODO: set up other things so they can caputre pipe

use crate::span::Span;
use crate::span::SpanKind;
use nom::bytes::complete::is_not;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;

pub fn plaintext<'a>(source: &'a str) -> IResult<&'a str, Span, ErrorTree<&'a str>> {
    let (source, parsed_text) = is_not("\n<>`-|:\\").context("").parse(source)?;
    // TODO: This should be what you end up with, but starting with
    // some things turned off above to get parsing to work (e.g. no caret so
    // you can push the footnote thing back a bit.)s
    // let (source, parsed_text) = is_not("\n<^>*`_-|:\\").context("").parse(source)?;
    let span = Span {
        attrs: vec![],
        parsed_text: parsed_text.to_string(),
        kind: SpanKind::PlainText,
    };
    Ok((source, span))
}

// pub fn plaintext_dev<'a>(source: &'a str) -> IResult<&'a str, Span, ErrorTree<&'a str>> {
//     let (source, parsed_text) = is_not("\n<^*`_-|:\\").context("").parse(source)?;
//     let span = Span {
//         attrs: vec![],
//         parsed_text: parsed_text.to_string(),
//         kind: SpanKind::PlainText,
//     };
//     Ok((source, span))
// }

// A bunch of other things use this which exercise it
// in terms of testing for now

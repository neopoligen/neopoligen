use crate::span::*;
use nom::bytes::complete::is_not;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;

// // TODO: Move to own file with tests
// pub fn newline(source: &str) -> IResult<&str, Span, ErrorTree<&str>> {
//     let initial_source = source;
//     let (source, _) = tuple((space0, line_ending)).context("").parse(source)?;
//     let (source, _) = not(tuple((space0, line_ending)))
//         .context("")
//         .parse(source)?;
//     let source_text = initial_source.replace(source, "").to_string();
//     Ok((
//         source,
//         Span {
//             attrs: vec![],
//             source_text,
//             parsed_text: "\n".to_string(),
//             kind: SpanKind::Newline,
//         },
//     ))
// }

// // TODO: Move to own file with tests
// pub fn space(source: &str) -> IResult<&str, Span, ErrorTree<&str>> {
//     let initial_source = source;
//     let (source, _) = space1.context("").parse(source)?;
//     let source_text = initial_source.replace(source, "").to_string();
//     Ok((
//         source,
//         Span {
//             attrs: vec![],
//             source_text,
//             parsed_text: " ".to_string(),
//             kind: SpanKind::Space,
//         },
//     ))
// }
//

// TODO: Move to own file with tests
pub fn wordpart(source: &str) -> IResult<&str, Span, ErrorTree<&str>> {
    let initial_source = source;
    let (source, text) = is_not(" \\`|:<>\n\t").context("").parse(source)?;
    let source_text = initial_source.replace(source, "").to_string();
    Ok((
        source,
        Span {
            attrs: vec![],
            source_text,
            parsed_text: text.to_string(),
            kind: SpanKind::WordPart,
        },
    ))
}

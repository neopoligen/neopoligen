use crate::span::*;
use nom::bytes::complete::tag;
use nom::multi::many1;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;

pub fn more_than_two_carets(source: &str) -> IResult<&str, Span, ErrorTree<&str>> {
    let initial_source = source;
    let (source, _) = tag("^^").context("").parse(source)?;
    let (source, _) = many1(tag("^")).context("").parse(source)?;
    let parsed_text = initial_source.replace(source, "");

    Ok((
        source,
        Span {
            attrs: vec![],
            parsed_text: parsed_text.to_string(),
            kind: SpanKind::MoreThanTwoCarets,
        },
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    #[test]
    fn basic_check() {
        let source = "^^^";
        let left = (
            "",
            Span {
                attrs: vec![],
                parsed_text: "^^^".to_string(),
                kind: SpanKind::MoreThanTwoCarets,
            },
        );
        let right = more_than_two_carets(source).unwrap();
        assert_eq!(left, right);
    }
}

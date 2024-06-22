use crate::span::*;
use nom::bytes::complete::tag;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;

pub fn hyphen(source: &str) -> IResult<&str, Span, ErrorTree<&str>> {
    let (source, _) = tag("-").context("").parse(source)?;
    Ok((
        source,
        Span {
            attrs: vec![],
            parsed_text: "-".to_string(),
            kind: SpanKind::Hyphen,
        },
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    #[test]
    fn basci_check() {
        let source = "-";
        let left = (
            "",
            Span {
                attrs: vec![],
                parsed_text: "-".to_string(),
                kind: SpanKind::Hyphen,
            },
        );
        let right = hyphen(source).unwrap();
        assert_eq!(left, right);
    }
}

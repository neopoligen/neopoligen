use crate::span::*;
use nom::bytes::complete::tag;
use nom::sequence::pair;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;

pub fn double_underscore(source: &str) -> IResult<&str, Span, ErrorTree<&str>> {
    let (source, _) = pair(tag("_"), tag("_")).context("").parse(source)?;
    Ok((
        source,
        Span {
            attrs: vec![],
            parsed_text: "__".to_string(),
            kind: SpanKind::DoubleUnderscore,
        },
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    #[test]
    fn at_eof() {
        let source = "__";
        let left = (
            "",
            Span {
                attrs: vec![],
                parsed_text: "__".to_string(),
                kind: SpanKind::DoubleUnderscore,
            },
        );
        let right = double_underscore(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn in_front_of_another_character() {
        let source = "__x";
        let left = (
            "x",
            Span {
                attrs: vec![],
                parsed_text: "__".to_string(),
                kind: SpanKind::DoubleUnderscore {},
            },
        );
        let right = double_underscore(source).unwrap();
        assert_eq!(left, right);
    }
}

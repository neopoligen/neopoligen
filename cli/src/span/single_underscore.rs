use crate::span::*;
use nom::bytes::complete::tag;
use nom::combinator::not;
use nom::sequence::pair;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;

pub fn single_underscore(source: &str) -> IResult<&str, Span, ErrorTree<&str>> {
    let (source, _) = pair(tag("_"), not(tag("_"))).context("").parse(source)?;
    Ok((
        source,
        Span {
            attrs: vec![],
            parsed_text: "_".to_string(),
            kind: SpanKind::SingleUnderscore,
        },
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    #[test]
    fn at_eof() {
        let source = "_";
        let left = (
            "",
            Span {
                attrs: vec![],
                parsed_text: "_".to_string(),
                kind: SpanKind::SingleUnderscore,
            },
        );
        let right = single_underscore(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn in_front_of_another_character() {
        let source = "_x";
        let left = (
            "x",
            Span {
                attrs: vec![],
                parsed_text: "_".to_string(),
                kind: SpanKind::SingleUnderscore {},
            },
        );
        let right = single_underscore(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn dont_capture_two() {
        let source = "__";
        assert!(single_underscore(source).is_err());
    }
}

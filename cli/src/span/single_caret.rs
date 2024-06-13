use crate::span::*;
use nom::bytes::complete::tag;
use nom::combinator::not;
use nom::sequence::pair;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;

pub fn single_caret(source: &str) -> IResult<&str, Span, ErrorTree<&str>> {
    let (source, _) = pair(tag("^"), not(tag("^"))).context("").parse(source)?;
    Ok((
        source,
        Span {
            attrs: vec![],
            parsed_text: "^".to_string(),
            kind: SpanKind::SingleCaret,
        },
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    #[test]
    fn single_at_eof() {
        let source = "^";
        let left = (
            "",
            Span {
                attrs: vec![],
                parsed_text: "^".to_string(),
                kind: SpanKind::SingleCaret,
            },
        );
        let right = single_caret(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn single_in_front_of_another_character() {
        let source = "^x";
        let left = (
            "x",
            Span {
                attrs: vec![],
                parsed_text: "^".to_string(),
                kind: SpanKind::SingleCaret {},
            },
        );
        let right = single_caret(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn dont_capture_two() {
        let source = "^^";
        assert!(single_caret(source).is_err());
    }
}

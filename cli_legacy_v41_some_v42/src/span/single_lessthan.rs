use crate::span::*;
use nom::bytes::complete::tag;
use nom::combinator::not;
use nom::sequence::pair;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;

pub fn single_lessthan(source: &str) -> IResult<&str, Span, ErrorTree<&str>> {
    let (source, _) = pair(tag("<"), not(tag("<"))).context("").parse(source)?;
    Ok((
        source,
        Span {
            attrs: vec![],
            parsed_text: "<".to_string(),
            kind: SpanKind::SingleLessThan,
        },
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    #[test]
    fn backtick_at_eof() {
        let source = "<";
        let left = (
            "",
            Span {
                attrs: vec![],
                parsed_text: "<".to_string(),
                kind: SpanKind::SingleLessThan,
            },
        );
        let right = single_lessthan(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn backtick_at_infront_of_another_character() {
        let source = "<x";
        let left = (
            "x",
            Span {
                attrs: vec![],
                parsed_text: "<".to_string(),
                kind: SpanKind::SingleLessThan {},
            },
        );
        let right = single_lessthan(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn dont_capture_two_greaterthans() {
        let source = "<<";
        assert!(single_lessthan(source).is_err());
    }
}

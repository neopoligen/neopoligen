use crate::span::*;
use nom::bytes::complete::tag;
use nom::combinator::not;
use nom::sequence::pair;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;

pub fn single_greaterthan(source: &str) -> IResult<&str, Span, ErrorTree<&str>> {
    let initial_source = source;
    let (source, _) = pair(tag(">"), not(tag(">"))).context("").parse(source)?;
    let source_text = initial_source.replace(source, "").to_string();
    Ok((
        source,
        Span {
            attrs: vec![],
            source_text,
            parsed_text: ">".to_string(),
            kind: SpanKind::SingleGreaterThan,
        },
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    #[test]
    fn backtick_at_eof() {
        let source = ">";
        let left = (
            "",
            Span {
                attrs: vec![],
                source_text: ">".to_string(),
                parsed_text: ">".to_string(),
                kind: SpanKind::SingleGreaterThan,
            },
        );
        let right = single_greaterthan(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn backtick_at_infront_of_another_character() {
        let source = ">x";
        let left = (
            "x",
            Span {
                attrs: vec![],
                source_text: ">".to_string(),
                parsed_text: ">".to_string(),
                kind: SpanKind::SingleGreaterThan {},
            },
        );
        let right = single_greaterthan(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn dont_capture_two_greaterthans() {
        let source = ">>";
        assert!(single_greaterthan(source).is_err());
    }
}

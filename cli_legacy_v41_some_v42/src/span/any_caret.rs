use crate::span::*;
use nom::bytes::complete::tag;
// use nom::combinator::not;
// use nom::sequence::pair;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;

// This is for things like inside code blocks where you don't
// want (or have to) escape less than signs since they
// aren't control characters in the parsing.

pub fn any_caret(source: &str) -> IResult<&str, Span, ErrorTree<&str>> {
    let (source, _) = tag("^").context("").parse(source)?;
    Ok((
        source,
        Span {
            attrs: vec![],
            parsed_text: "^".to_string(),
            kind: SpanKind::AnyCaret,
        },
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn single_character() {
        let source = "^";
        let left = (
            "",
            Span {
                attrs: vec![],
                parsed_text: "^".to_string(),
                kind: SpanKind::AnyCaret,
            },
        );
        let right = any_caret(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn allowing_trailing() {
        let source = "^^";
        let left = (
            "^",
            Span {
                attrs: vec![],
                parsed_text: "^".to_string(),
                kind: SpanKind::AnyCaret,
            },
        );
        let right = any_caret(source).unwrap();
        assert_eq!(left, right);
    }
}

use crate::span::*;
use nom::bytes::complete::tag;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;

// This is for things like inside code blocks where you don't
// want (or have to) escape less than signs since they
// aren't control characters in the parsing.

pub fn any_asterisk(source: &str) -> IResult<&str, Span, ErrorTree<&str>> {
    let (source, _) = tag("*").context("").parse(source)?;
    Ok((
        source,
        Span {
            attrs: vec![],
            parsed_text: "*".to_string(),
            kind: SpanKind::AnyAsterisk,
        },
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn single_character() {
        let source = "*";
        let left = (
            "",
            Span {
                attrs: vec![],
                parsed_text: "*".to_string(),
                kind: SpanKind::AnyAsterisk,
            },
        );
        let right = any_asterisk(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn allowing_trailing() {
        let source = "**";
        let left = (
            "*",
            Span {
                attrs: vec![],
                parsed_text: "*".to_string(),
                kind: SpanKind::AnyAsterisk,
            },
        );
        let right = any_asterisk(source).unwrap();
        assert_eq!(left, right);
    }
}

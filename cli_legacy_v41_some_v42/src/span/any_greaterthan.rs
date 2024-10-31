use crate::span::*;
use nom::bytes::complete::tag;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;

pub fn any_greaterthan(source: &str) -> IResult<&str, Span, ErrorTree<&str>> {
    let (source, _) = tag(">").context("").parse(source)?;
    Ok((
        source,
        Span {
            attrs: vec![],
            parsed_text: ">".to_string(),
            kind: SpanKind::AnyGreaterThan,
        },
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn basic_test() {
        let source = ">";
        let left = (
            "",
            Span {
                attrs: vec![],
                parsed_text: ">".to_string(),
                kind: SpanKind::AnyGreaterThan,
            },
        );
        let right = any_greaterthan(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn allowing_trailinggreaterthan() {
        let source = ">>";
        let left = (
            ">",
            Span {
                attrs: vec![],
                parsed_text: ">".to_string(),
                kind: SpanKind::AnyGreaterThan,
            },
        );
        let right = any_greaterthan(source).unwrap();
        assert_eq!(left, right);
    }
}

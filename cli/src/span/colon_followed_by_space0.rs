use crate::span::*;
use nom::bytes::complete::tag;
use nom::sequence::pair;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;

pub fn colon_followed_by_space0(source: &str) -> IResult<&str, Span, ErrorTree<&str>> {
    let (source, result) = pair(tag(":"), space0).context("").parse(source)?;
    let space_value = if result.1.len() == 0 { "" } else { " " };
    Ok((
        source,
        Span {
            attrs: vec![],
            parsed_text: format!(":{}", space_value),
            kind: SpanKind::ColonFollowedBySpace0,
        },
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn basic_check() {
        let source = ": ";
        let left = (
            "",
            Span {
                attrs: vec![],
                parsed_text: ": ".to_string(),
                kind: SpanKind::ColonFollowedBySpace0,
            },
        );
        let right = colon_followed_by_space0(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn basic_check_without_space() {
        let source = ":";
        let left = (
            "",
            Span {
                attrs: vec![],
                parsed_text: ":".to_string(),
                kind: SpanKind::ColonFollowedBySpace0,
            },
        );
        let right = colon_followed_by_space0(source).unwrap();
        assert_eq!(left, right);
    }
}

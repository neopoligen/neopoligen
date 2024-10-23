use crate::span::*;
use nom::bytes::complete::tag;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;

pub fn hyphen_with_trailing_non_hyphen(source: &str) -> IResult<&str, Span, ErrorTree<&str>> {
    // Reminder: not doing the source/replace here because the
    // escape is captured by the slash. There's certainly
    // a way around that, but this is fine
    let (source, _) = tuple((tag("-"), not(tag("-")))).context("").parse(source)?;
    Ok((
        source,
        Span {
            attrs: vec![],
            parsed_text: "-".to_string(),
            kind: SpanKind::HyphenWithTrailingNonHyphen,
        },
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    #[test]
    fn solo_basic_check() {
        let source = "- ";
        let left = (
            " ",
            Span {
                attrs: vec![],
                parsed_text: "-".to_string(),
                kind: SpanKind::HyphenWithTrailingNonHyphen,
            },
        );
        let right = hyphen_with_trailing_non_hyphen(source).unwrap();
        assert_eq!(left, right);
    }
}

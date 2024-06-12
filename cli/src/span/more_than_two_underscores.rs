use crate::span::*;
use nom::bytes::complete::tag;
use nom::multi::many1;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;

pub fn more_than_two_underscores(source: &str) -> IResult<&str, Span, ErrorTree<&str>> {
    // Reminder: not doing the source/replace here because the
    // escape is captured by the slash. There's certainly
    // a way around that, but this is fine
    let initial_source = source;
    let (source, _) = tag("__").context("").parse(source)?;
    let (source, _) = many1(tag("_")).context("").parse(source)?;
    let parsed_text = initial_source.replace(source, "");

    Ok((
        source,
        Span {
            attrs: vec![],
            source_text: parsed_text.to_string(),
            parsed_text: parsed_text.to_string(),
            kind: SpanKind::MoreThanTwoUnderscores,
        },
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    #[test]
    fn basic_check() {
        let source = "___";
        let left = (
            "",
            Span {
                attrs: vec![],
                source_text: "___".to_string(),
                parsed_text: "___".to_string(),
                kind: SpanKind::MoreThanTwoUnderscores,
            },
        );
        let right = more_than_two_underscores(source).unwrap();
        assert_eq!(left, right);
    }
}

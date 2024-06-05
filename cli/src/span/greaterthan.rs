use crate::span::*;
use nom::bytes::complete::tag;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;

pub fn greaterthan(source: &str) -> IResult<&str, Span, ErrorTree<&str>> {
    let initial_source = source;
    let (source, _) = tag(">").context("").parse(source)?;
    let source_text = initial_source.replace(source, "").to_string();
    Ok((
        source,
        Span {
            attrs: vec![],
            source_text,
            parsed_text: ">".to_string(),
            kind: SpanKind::GreaterThan,
        },
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    #[test]
    fn basci_check() {
        let source = ">";
        let left = (
            "",
            Span {
                attrs: vec![],
                source_text: ">".to_string(),
                parsed_text: ">".to_string(),
                kind: SpanKind::GreaterThan,
            },
        );
        let right = greaterthan(source).unwrap();
        assert_eq!(left, right);
    }
}

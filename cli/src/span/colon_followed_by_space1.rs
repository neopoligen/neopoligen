use crate::span::*;
use nom::bytes::complete::tag;
use nom::sequence::pair;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;

pub fn colon_followed_by_space1(source: &str) -> IResult<&str, Span, ErrorTree<&str>> {
    let (source, _) = pair(tag(":"), space1).context("").parse(source)?;
    Ok((
        source,
        Span {
            attrs: vec![],
            parsed_text: ": ".to_string(),
            kind: SpanKind::ColonFollowedBySpace1,
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
                kind: SpanKind::ColonFollowedBySpace1,
            },
        );
        let right = colon_followed_by_space1(source).unwrap();
        assert_eq!(left, right);
    }
}

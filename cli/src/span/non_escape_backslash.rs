use crate::span::*;
use nom::bytes::complete::tag;
use nom::combinator::not;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;

pub fn non_escape_backslash(source: &str) -> IResult<&str, Span, ErrorTree<&str>> {
    let initial_source = source;
    let (source, _) = tag("\\").context("").parse(source)?;
    let (source, _) = not(tag("<")).context("").parse(source)?;
    let (source, _) = not(tag("|")).context("").parse(source)?;
    let (source, _) = not(tag(">")).context("").parse(source)?;
    let (source, _) = not(tag("`")).context("").parse(source)?;
    let (source, _) = not(tag(":")).context("").parse(source)?;
    let source_text = initial_source.replace(source, "").to_string();
    Ok((
        source,
        Span {
            attrs: vec![],
            source_text,
            parsed_text: "\\".to_string(),
            kind: SpanKind::NonEscapeBackslash,
        },
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;
    #[rstest]
    #[case("\\:", "")]
    #[case("\\>", "")]
    #[case("\\|", "")]
    #[case("\\<", "")]
    #[case("\\`", "")]
    fn run_test(#[case] input: &str, #[case] _description: &str) {
        assert!(non_escape_backslash(input).is_err());
    }
}

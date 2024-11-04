use crate::span_v42::plaintext::PlainTextSpan;
use crate::span_v42::SpanV42;
use nom::bytes::complete::is_not;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;

pub fn plaintext_without_spaces(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    // Remember: some of these characters aren't used in
    // shortcodes but are here to prevent them being used in
    // things that might touch file paths.
    let (source, output) = is_not(" \t\r\n~`*^![]{}<>|:_\\")
        .context("")
        .parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: output.to_string(),
        }),
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case("alfa-bravo", "-bravo")]
    fn valid_cases_with_remainders(#[case] source: &str, #[case] remainder: &str) {
        assert_eq!(remainder, plaintext_without_spaces(source).unwrap().0);
    }
}

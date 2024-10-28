use super::PlainTextSpan;
use super::SpanV42;
use nom::bytes::complete::is_a;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;

pub fn any_underscores(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, result) = is_a("_").context("").parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: result.to_string(),
        }),
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn single_character() {
        let source = "_";
        let left = (
            "",
            SpanV42::PlainText(PlainTextSpan {
                text: "_".to_string(),
            }),
        );
        let right = any_underscores(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn allow_multiple() {
        let source = "__";
        let left = (
            "",
            SpanV42::PlainText(PlainTextSpan {
                text: "__".to_string(),
            }),
        );
        let right = any_underscores(source).unwrap();
        assert_eq!(left, right);
    }

    //
}

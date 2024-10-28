use super::PlainTextSpan;
use super::SpanV42;
use nom::bytes::complete::is_a;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;

pub fn any_colon(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, result) = is_a(":").context("").parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            output: result.to_string(),
        }),
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn single_character() {
        let source = ":";
        let left = (
            "",
            SpanV42::PlainText(PlainTextSpan {
                output: ":".to_string(),
            }),
        );
        let right = any_colon(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn allow_multiple() {
        let source = "::";
        let left = (
            "",
            SpanV42::PlainText(PlainTextSpan {
                output: "::".to_string(),
            }),
        );
        let right = any_colon(source).unwrap();
        assert_eq!(left, right);
    }

    //
}

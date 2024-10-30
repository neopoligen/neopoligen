use super::PlainTextSpan;
use super::SpanV42;
use nom::bytes::complete::is_a;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;

pub fn any_colons(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, result) = is_a(":").context("").parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: result.to_string(),
            kind: "plain_text".to_string(),
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
                text: ":".to_string(),
                kind: "plain_text".to_string(),
            }),
        );
        let right = any_colons(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn allow_multiple() {
        let source = "::";
        let left = (
            "",
            SpanV42::PlainText(PlainTextSpan {
                text: "::".to_string(),
                kind: "plain_text".to_string(),
            }),
        );
        let right = any_colons(source).unwrap();
        assert_eq!(left, right);
    }

    //
}

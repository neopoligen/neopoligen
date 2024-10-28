use crate::span_v42::*;
use nom::bytes::complete::tag;
use nom::sequence::terminated;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;

pub fn single_hyphen(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, output) = terminated(tag("-"), not(tag("-")))
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

    #[test]
    fn single_at_eof() {
        let source = "-";
        let left = (
            "",
            SpanV42::PlainText(PlainTextSpan {
                text: "-".to_string(),
            }),
        );
        let right = single_hyphen(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn single_with_more_content() {
        let source = "-alfa";
        let left = (
            "alfa",
            SpanV42::PlainText(PlainTextSpan {
                text: "-".to_string(),
            }),
        );
        let right = single_hyphen(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn prevent_mutiple() {
        let source = "--";
        assert!(single_hyphen(source).is_err());
    }
}

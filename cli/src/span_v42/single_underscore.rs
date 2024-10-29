use crate::span_v42::*;
use nom::bytes::complete::tag;
use nom::sequence::terminated;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;

pub fn single_underscore(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, output) = terminated(tag("_"), not(tag("_")))
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
        let source = "_";
        let left = (
            "",
            SpanV42::PlainText(PlainTextSpan {
                text: "_".to_string(),
            }),
        );
        let right = single_underscore(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn single_with_more_content() {
        let source = "_alfa";
        let left = (
            "alfa",
            SpanV42::PlainText(PlainTextSpan {
                text: "_".to_string(),
            }),
        );
        let right = single_underscore(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn prevent_mutiple() {
        let source = "__";
        assert!(single_underscore(source).is_err());
    }
}

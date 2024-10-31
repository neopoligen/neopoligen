use super::PlainTextSpan;
use super::SpanV42;
use nom::character::complete::alphanumeric1;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;

pub fn any_alphanumeric(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, result) = alphanumeric1.context("").parse(source)?;
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
    fn basic_check() {
        let source = "alfa1:";
        let left = (
            ":",
            SpanV42::PlainText(PlainTextSpan {
                text: "alfa1".to_string(),
            }),
        );
        let right = any_alphanumeric(source).unwrap();
        assert_eq!(left, right);
    }

    //
}

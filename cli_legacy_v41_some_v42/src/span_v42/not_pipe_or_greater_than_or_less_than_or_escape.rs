use super::PlainTextSpan;
use super::SpanV42;
use nom::bytes::complete::is_not;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;

pub fn not_pipe_or_greater_than_or_less_than_or_escape(
    source: &str,
) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, result) = is_not("\\|<>").context("").parse(source)?;
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
    fn check_to_escape() {
        let source = "alfa\\>";
        let left = (
            "\\>",
            SpanV42::PlainText(PlainTextSpan {
                text: "alfa".to_string(),
            }),
        );
        let right = not_pipe_or_greater_than_or_less_than_or_escape(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn check_to_pipe() {
        let source = "!@#$|";
        let left = (
            "|",
            SpanV42::PlainText(PlainTextSpan {
                text: "!@#$".to_string(),
            }),
        );
        let right = not_pipe_or_greater_than_or_less_than_or_escape(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn check_to_greater_than() {
        let source = "alfa bravo\ncharlie >";
        let left = (
            ">",
            SpanV42::PlainText(PlainTextSpan {
                text: "alfa bravo\ncharlie ".to_string(),
            }),
        );
        let right = not_pipe_or_greater_than_or_less_than_or_escape(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn check_to_less_than() {
        let source = "alfa<bravo";
        let left = (
            "<bravo",
            SpanV42::PlainText(PlainTextSpan {
                text: "alfa".to_string(),
            }),
        );
        let right = not_pipe_or_greater_than_or_less_than_or_escape(source).unwrap();
        assert_eq!(left, right);
    }

    //
}

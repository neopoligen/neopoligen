use crate::span_v42::PlainTextSpan;
use crate::span_v42::SpanV42;
use nom::character::complete::line_ending;
use nom::character::complete::space0;
use nom::combinator::eof;
use nom::combinator::not;
use nom::sequence::pair;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;

pub fn block_line_ending(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, _) = tuple((line_ending, not(eof), not(pair(space0, line_ending))))
        .context("")
        .parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: " ".to_string(),
            kind: "plain_text".to_string(),
        }),
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn single_instance() {
        let source = "\nx";
        let left = (
            "x",
            SpanV42::PlainText(PlainTextSpan {
                text: " ".to_string(),
                kind: "plain_text".to_string(),
            }),
        );
        let right = block_line_ending(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn single_instance_at_eof() {
        let source = "\nx";
        let left = (
            "x",
            SpanV42::PlainText(PlainTextSpan {
                text: " ".to_string(),
                kind: "plain_text".to_string(),
            }),
        );
        let right = block_line_ending(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn do_not_caputre_two() {
        let source = "\n\n";
        assert!(block_line_ending(source).is_err());
    }

    #[test]
    fn avoid_empty_lines() {
        let source = "\n        \n";
        assert!(block_line_ending(source).is_err());
    }

    #[test]
    fn avoid_eof() {
        let source = "";
        assert!(block_line_ending(source).is_err());
    }

    #[test]
    fn avoid_new_line_before_eof() {
        let source = "\n";
        assert!(block_line_ending(source).is_err());
    }
}

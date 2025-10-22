use crate::span_v42::PlainTextSpan;
use crate::span_v42::SpanV42;
use nom::character::complete::line_ending;
use nom::combinator::not;
use nom::sequence::pair;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;

pub fn single_line_ending<'a>(source: &'a str) -> IResult<&'a str, SpanV42, ErrorTree<&'a str>> {
    let (source, _) = pair(line_ending, not(line_ending))
        .context("")
        .parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: " ".to_string(),
            
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
                
            }),
        );
        let right = single_line_ending(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn do_not_caputre_two() {
        let source = "\n\nx";
        assert!(single_line_ending(source).is_err());
    }
}

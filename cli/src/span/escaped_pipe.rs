use crate::span::Span;
use nom::bytes::complete::tag;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;

pub fn escaped_pipe(source: &str) -> IResult<&str, Span, ErrorTree<&str>> {
    let (source, text) = tag("\\|").context("").parse(source)?;
    Ok((
        source,
        Span::RawText {
            text: text.to_string(),
            r#type: "escaped-pipe".to_string(),
        },
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn basic_test() {
        let source = "\\|";
        let left = Span::RawText {
            text: "\\|".to_string(),
            r#type: "escaped-pipe".to_string(),
        };
        let right = (|src| escaped_pipe(src))(source).unwrap().1;
        assert_eq!(left, right);
    }

    //
}

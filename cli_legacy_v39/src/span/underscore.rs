use crate::span::Span;
use nom::bytes::complete::tag;
use nom::combinator::not;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;

pub fn underscore(source: &str) -> IResult<&str, Span, ErrorTree<&str>> {
    let (source, text) = tag("_").context("").parse(source)?;
    let (source, _) = not(tag("_")).context("").parse(source)?;
    Ok((
        source,
        Span::RawText {
            text: text.to_string(),
            r#type: "underscore".to_string(),
        },
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn basic_test() {
        let source = "_x";
        let left = Span::RawText {
            text: "_".to_string(),
            r#type: "underscore".to_string(),
        };
        let right = (|src| underscore(src))(source).unwrap().1;
        assert_eq!(left, right);
    }

    #[test]
    fn skip_second() {
        let source = "__";
        assert!((|src| underscore(src))(source).is_err())
    }

    //
}

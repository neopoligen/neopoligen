use crate::span::Span;
use nom::bytes::complete::tag;
use nom::combinator::not;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;

pub fn backtic(source: &str) -> IResult<&str, Span, ErrorTree<&str>> {
    let (source, text) = tag("`").context("").parse(source)?;
    let (source, _) = not(tag("`")).context("").parse(source)?;
    Ok((
        source,
        Span::RawText {
            text: text.to_string(),
            r#type: "backtic".to_string(),
        },
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn basic_test() {
        let source = "`x";
        let left = Span::RawText {
            text: "`".to_string(),
            r#type: "backtic".to_string(),
        };
        let right = (|src| backtic(src))(source).unwrap().1;
        assert_eq!(left, right);
    }

    #[test]
    fn skip_second() {
        let source = "``";
        assert!((|src| backtic(src))(source).is_err())
    }

    //
}

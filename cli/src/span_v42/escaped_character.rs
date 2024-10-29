use super::SpanV42;
use nom::bytes::complete::tag;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;
use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct EscapedCharacterSpanV42 {
    pub text: String,
}

pub fn escaped_greater_than(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, _) = tag("\\>").context("").parse(source)?;
    Ok((
        source,
        SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
            text: ">".to_string(),
        }),
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn check_escaped_greater_than() {
        let source = "\\>";
        let left = (
            "",
            SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
                text: ">".to_string(),
            }),
        );
        let right = escaped_greater_than(source).unwrap();
        assert_eq!(left, right);
    }

    //
}

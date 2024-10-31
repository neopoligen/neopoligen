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

pub fn escaped_asterisk(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, _) = tag("\\*").context("").parse(source)?;
    Ok((
        source,
        SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
            text: "*".to_string(),
        }),
    ))
}

pub fn escaped_caret(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, _) = tag("\\^").context("").parse(source)?;
    Ok((
        source,
        SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
            text: "^".to_string(),
        }),
    ))
}

pub fn escaped_less_than(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, _) = tag("\\<").context("").parse(source)?;
    Ok((
        source,
        SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
            text: "<".to_string(),
        }),
    ))
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

pub fn escaped_pipe(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, _) = tag("\\|").context("").parse(source)?;
    Ok((
        source,
        SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
            text: "|".to_string(),
        }),
    ))
}

pub fn escaped_backslash(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, _) = tag("\\\\").context("").parse(source)?;
    Ok((
        source,
        SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
            text: "\\".to_string(),
        }),
    ))
}

pub fn escaped_backtick(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, _) = tag("\\`").context("").parse(source)?;
    Ok((
        source,
        SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
            text: "`".to_string(),
        }),
    ))
}

pub fn escaped_underscore(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, _) = tag("\\_").context("").parse(source)?;
    Ok((
        source,
        SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
            text: "_".to_string(),
        }),
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn check_escaped_less_than() {
        let source = "\\<";
        let left = (
            "",
            SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
                text: "<".to_string(),
            }),
        );
        let right = escaped_less_than(source).unwrap();
        assert_eq!(left, right);
    }

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

    #[test]
    fn check_escaped_pipe() {
        let source = "\\|";
        let left = (
            "",
            SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
                text: "|".to_string(),
            }),
        );
        let right = escaped_pipe(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn check_escaped_backslash() {
        let source = "\\\\";
        let left = (
            "",
            SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
                text: "\\".to_string(),
            }),
        );
        let right = escaped_backslash(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn check_escaped_backtick() {
        let source = "\\`";
        let left = (
            "",
            SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
                text: "`".to_string(),
            }),
        );
        let right = escaped_backtick(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn check_escaped_caret() {
        let source = "\\^";
        let left = (
            "",
            SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
                text: "^".to_string(),
            }),
        );
        let right = escaped_caret(source).unwrap();
        assert_eq!(left, right);
    }

    //
}

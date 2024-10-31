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

pub fn escaped_tilde(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, _) = tag("\\~").context("").parse(source)?;
    Ok((
        source,
        SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
            text: "~".to_string(),
        }),
    ))
}

pub fn escaped_exclamation(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, _) = tag("\\!").context("").parse(source)?;
    Ok((
        source,
        SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
            text: "!".to_string(),
        }),
    ))
}

pub fn escaped_at(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, _) = tag("\\@").context("").parse(source)?;
    Ok((
        source,
        SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
            text: "@".to_string(),
        }),
    ))
}

pub fn escaped_pound_sign(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, _) = tag("\\#").context("").parse(source)?;
    Ok((
        source,
        SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
            text: "#".to_string(),
        }),
    ))
}

pub fn escaped_dollar_sign(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, _) = tag("\\$").context("").parse(source)?;
    Ok((
        source,
        SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
            text: "$".to_string(),
        }),
    ))
}

pub fn escaped_percent_sign(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, _) = tag("\\%").context("").parse(source)?;
    Ok((
        source,
        SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
            text: "%".to_string(),
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

pub fn escaped_ampersand(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, _) = tag("\\&").context("").parse(source)?;
    Ok((
        source,
        SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
            text: "&".to_string(),
        }),
    ))
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

pub fn escaped_open_paren(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, _) = tag("\\(").context("").parse(source)?;
    Ok((
        source,
        SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
            text: "(".to_string(),
        }),
    ))
}

pub fn escaped_close_paren(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, _) = tag("\\)").context("").parse(source)?;
    Ok((
        source,
        SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
            text: ")".to_string(),
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

pub fn escaped_hyphen(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, _) = tag("\\-").context("").parse(source)?;
    Ok((
        source,
        SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
            text: "-".to_string(),
        }),
    ))
}

pub fn escaped_plus_sign(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, _) = tag("\\+").context("").parse(source)?;
    Ok((
        source,
        SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
            text: "+".to_string(),
        }),
    ))
}

pub fn escaped_equal_sign(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, _) = tag("\\=").context("").parse(source)?;
    Ok((
        source,
        SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
            text: "=".to_string(),
        }),
    ))
}

pub fn escaped_open_brace(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, _) = tag("\\{").context("").parse(source)?;
    Ok((
        source,
        SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
            text: "{".to_string(),
        }),
    ))
}

pub fn escaped_close_brace(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, _) = tag("\\}").context("").parse(source)?;
    Ok((
        source,
        SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
            text: "}".to_string(),
        }),
    ))
}

pub fn escaped_open_bracket(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, _) = tag("\\[").context("").parse(source)?;
    Ok((
        source,
        SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
            text: "[".to_string(),
        }),
    ))
}

pub fn escaped_close_bracket(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, _) = tag("\\]").context("").parse(source)?;
    Ok((
        source,
        SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
            text: "]".to_string(),
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

pub fn escaped_back_slash(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, _) = tag("\\\\").context("").parse(source)?;
    Ok((
        source,
        SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
            text: "\\".to_string(),
        }),
    ))
}

pub fn escaped_colon(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, _) = tag("\\:").context("").parse(source)?;
    Ok((
        source,
        SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
            text: ":".to_string(),
        }),
    ))
}

pub fn escaped_semicolon(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, _) = tag("\\;").context("").parse(source)?;
    Ok((
        source,
        SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
            text: ";".to_string(),
        }),
    ))
}

pub fn escaped_double_quote(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, _) = tag("\\\"").context("").parse(source)?;
    Ok((
        source,
        SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
            text: "\"".to_string(),
        }),
    ))
}

pub fn escaped_single_quote(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, _) = tag("\\'").context("").parse(source)?;
    Ok((
        source,
        SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
            text: "'".to_string(),
        }),
    ))
}

pub fn escaped_comma(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, _) = tag("\\,").context("").parse(source)?;
    Ok((
        source,
        SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
            text: ",".to_string(),
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

pub fn escaped_period(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, _) = tag("\\.").context("").parse(source)?;
    Ok((
        source,
        SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
            text: ".".to_string(),
        }),
    ))
}

pub fn escaped_forward_slash(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, _) = tag("\\/").context("").parse(source)?;
    Ok((
        source,
        SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
            text: "/".to_string(),
        }),
    ))
}

pub fn escaped_question_mark(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, _) = tag("\\?").context("").parse(source)?;
    Ok((
        source,
        SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
            text: "?".to_string(),
        }),
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
}


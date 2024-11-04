use super::SpanV42;
use html_escape::*;
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
            text: encode_quoted_attribute("~").to_string(),
        }),
    ))
}

pub fn escaped_exclamation(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, _) = tag("\\!").context("").parse(source)?;
    Ok((
        source,
        SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
            text: encode_quoted_attribute("!").to_string(),
        }),
    ))
}

pub fn escaped_at(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, _) = tag("\\@").context("").parse(source)?;
    Ok((
        source,
        SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
            text: encode_quoted_attribute("@").to_string(),
        }),
    ))
}

pub fn escaped_pound_sign(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, _) = tag("\\#").context("").parse(source)?;
    Ok((
        source,
        SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
            text: encode_quoted_attribute("#").to_string(),
        }),
    ))
}

pub fn escaped_dollar_sign(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, _) = tag("\\$").context("").parse(source)?;
    Ok((
        source,
        SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
            text: encode_quoted_attribute("$").to_string(),
        }),
    ))
}

pub fn escaped_percent_sign(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, _) = tag("\\%").context("").parse(source)?;
    Ok((
        source,
        SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
            text: encode_quoted_attribute("%").to_string(),
        }),
    ))
}

pub fn escaped_caret(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, _) = tag("\\^").context("").parse(source)?;
    Ok((
        source,
        SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
            text: encode_quoted_attribute("^").to_string(),
        }),
    ))
}

pub fn escaped_ampersand(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, _) = tag("\\&").context("").parse(source)?;
    Ok((
        source,
        SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
            text: encode_quoted_attribute("&").to_string(),
        }),
    ))
}

pub fn escaped_asterisk(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, _) = tag("\\*").context("").parse(source)?;
    Ok((
        source,
        SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
            text: encode_quoted_attribute("*").to_string(),
        }),
    ))
}

pub fn escaped_open_paren(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, _) = tag("\\(").context("").parse(source)?;
    Ok((
        source,
        SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
            text: encode_quoted_attribute("(").to_string(),
        }),
    ))
}

pub fn escaped_close_paren(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, _) = tag("\\)").context("").parse(source)?;
    Ok((
        source,
        SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
            text: encode_quoted_attribute(")").to_string(),
        }),
    ))
}

pub fn escaped_underscore(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, _) = tag("\\_").context("").parse(source)?;
    Ok((
        source,
        SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
            text: encode_quoted_attribute("_").to_string(),
        }),
    ))
}

pub fn escaped_hyphen(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, _) = tag("\\-").context("").parse(source)?;
    Ok((
        source,
        SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
            text: encode_quoted_attribute("-").to_string(),
        }),
    ))
}

pub fn escaped_plus_sign(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, _) = tag("\\+").context("").parse(source)?;
    Ok((
        source,
        SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
            text: encode_quoted_attribute("+").to_string(),
        }),
    ))
}

pub fn escaped_equal_sign(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, _) = tag("\\=").context("").parse(source)?;
    Ok((
        source,
        SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
            text: encode_quoted_attribute("=").to_string(),
        }),
    ))
}

pub fn escaped_open_brace(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, _) = tag("\\{").context("").parse(source)?;
    Ok((
        source,
        SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
            text: encode_quoted_attribute("{").to_string(),
        }),
    ))
}

pub fn escaped_close_brace(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, _) = tag("\\}").context("").parse(source)?;
    Ok((
        source,
        SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
            text: encode_quoted_attribute("}").to_string(),
        }),
    ))
}

pub fn escaped_open_bracket(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, _) = tag("\\[").context("").parse(source)?;
    Ok((
        source,
        SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
            text: encode_quoted_attribute("[").to_string(),
        }),
    ))
}

pub fn escaped_close_bracket(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, _) = tag("\\]").context("").parse(source)?;
    Ok((
        source,
        SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
            text: encode_quoted_attribute("]").to_string(),
        }),
    ))
}

pub fn escaped_pipe(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, _) = tag("\\|").context("").parse(source)?;
    Ok((
        source,
        SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
            text: encode_quoted_attribute("|").to_string(),
        }),
    ))
}

pub fn escaped_backslash(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, _) = tag("\\\\").context("").parse(source)?;
    Ok((
        source,
        SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
            text: encode_quoted_attribute("\\").to_string(),
        }),
    ))
}

pub fn escaped_colon(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, _) = tag("\\:").context("").parse(source)?;
    Ok((
        source,
        SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
            text: encode_quoted_attribute(":").to_string(),
        }),
    ))
}

pub fn escaped_semicolon(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, _) = tag("\\;").context("").parse(source)?;
    Ok((
        source,
        SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
            text: encode_quoted_attribute(";").to_string(),
        }),
    ))
}

pub fn escaped_double_quote(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, _) = tag("\\\"").context("").parse(source)?;
    Ok((
        source,
        SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
            text: encode_quoted_attribute("\"").to_string(),
        }),
    ))
}

pub fn escaped_single_quote(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, _) = tag("\\'").context("").parse(source)?;
    Ok((
        source,
        SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
            text: encode_quoted_attribute("'").to_string(),
        }),
    ))
}

pub fn escaped_comma(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, _) = tag("\\,").context("").parse(source)?;
    Ok((
        source,
        SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
            text: encode_quoted_attribute(",").to_string(),
        }),
    ))
}

pub fn escaped_less_than(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, _) = tag("\\<").context("").parse(source)?;
    Ok((
        source,
        SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
            text: encode_quoted_attribute("<").to_string(),
        }),
    ))
}

pub fn escaped_greater_than(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, _) = tag("\\>").context("").parse(source)?;
    Ok((
        source,
        SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
            text: encode_quoted_attribute(">").to_string(),
        }),
    ))
}

pub fn escaped_period(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, _) = tag("\\.").context("").parse(source)?;
    Ok((
        source,
        SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
            text: encode_quoted_attribute(".").to_string(),
        }),
    ))
}

pub fn escaped_forward_slash(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, _) = tag("\\/").context("").parse(source)?;
    Ok((
        source,
        SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
            text: encode_quoted_attribute("/").to_string(),
        }),
    ))
}

pub fn escaped_question_mark(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, _) = tag("\\?").context("").parse(source)?;
    Ok((
        source,
        SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
            text: encode_quoted_attribute("?").to_string(),
        }),
    ))
}

pub fn escaped_backtick(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, _) = tag("\\`").context("").parse(source)?;
    Ok((
        source,
        SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
            text: encode_quoted_attribute("`").to_string(),
        }),
    ))
}

pub fn escaped_space(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, _) = tag("\\ ").context("").parse(source)?;
    Ok((
        source,
        SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
            text: encode_quoted_attribute(" ").to_string(),
        }),
    ))
}

pub fn escaped_tag(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, _) = tag("\\\t").context("").parse(source)?;
    Ok((
        source,
        SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
            text: encode_quoted_attribute("\t").to_string(),
        }),
    ))
}


#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn check_escaped_tilde() {
        let source = "\\~ alfa";
        let left = (
            " alfa",
            SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
                text: encode_quoted_attribute("~").to_string(),
            }),
        );
        let right = escaped_tilde(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn check_escaped_exclamation() {
        let source = "\\! alfa";
        let left = (
            " alfa",
            SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
                text: encode_quoted_attribute("!").to_string(),
            }),
        );
        let right = escaped_exclamation(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn check_escaped_at() {
        let source = "\\@ alfa";
        let left = (
            " alfa",
            SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
                text: encode_quoted_attribute("@").to_string(),
            }),
        );
        let right = escaped_at(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn check_escaped_pound_sign() {
        let source = "\\# alfa";
        let left = (
            " alfa",
            SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
                text: encode_quoted_attribute("#").to_string(),
            }),
        );
        let right = escaped_pound_sign(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn check_escaped_dollar_sign() {
        let source = "\\$ alfa";
        let left = (
            " alfa",
            SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
                text: encode_quoted_attribute("$").to_string(),
            }),
        );
        let right = escaped_dollar_sign(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn check_escaped_percent_sign() {
        let source = "\\% alfa";
        let left = (
            " alfa",
            SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
                text: encode_quoted_attribute("%").to_string(),
            }),
        );
        let right = escaped_percent_sign(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn check_escaped_caret() {
        let source = "\\^ alfa";
        let left = (
            " alfa",
            SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
                text: encode_quoted_attribute("^").to_string(),
            }),
        );
        let right = escaped_caret(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn check_escaped_ampersand() {
        let source = "\\& alfa";
        let left = (
            " alfa",
            SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
                text: encode_quoted_attribute("&").to_string(),
            }),
        );
        let right = escaped_ampersand(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn check_escaped_asterisk() {
        let source = "\\* alfa";
        let left = (
            " alfa",
            SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
                text: encode_quoted_attribute("*").to_string(),
            }),
        );
        let right = escaped_asterisk(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn check_escaped_open_paren() {
        let source = "\\( alfa";
        let left = (
            " alfa",
            SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
                text: encode_quoted_attribute("(").to_string(),
            }),
        );
        let right = escaped_open_paren(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn check_escaped_close_paren() {
        let source = "\\) alfa";
        let left = (
            " alfa",
            SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
                text: encode_quoted_attribute(")").to_string(),
            }),
        );
        let right = escaped_close_paren(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn check_escaped_underscore() {
        let source = "\\_ alfa";
        let left = (
            " alfa",
            SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
                text: encode_quoted_attribute("_").to_string(),
            }),
        );
        let right = escaped_underscore(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn check_escaped_hyphen() {
        let source = "\\- alfa";
        let left = (
            " alfa",
            SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
                text: encode_quoted_attribute("-").to_string(),
            }),
        );
        let right = escaped_hyphen(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn check_escaped_plus_sign() {
        let source = "\\+ alfa";
        let left = (
            " alfa",
            SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
                text: encode_quoted_attribute("+").to_string(),
            }),
        );
        let right = escaped_plus_sign(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn check_escaped_equal_sign() {
        let source = "\\= alfa";
        let left = (
            " alfa",
            SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
                text: encode_quoted_attribute("=").to_string(),
            }),
        );
        let right = escaped_equal_sign(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn check_escaped_open_brace() {
        let source = "\\{ alfa";
        let left = (
            " alfa",
            SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
                text: encode_quoted_attribute("{").to_string(),
            }),
        );
        let right = escaped_open_brace(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn check_escaped_close_brace() {
        let source = "\\} alfa";
        let left = (
            " alfa",
            SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
                text: encode_quoted_attribute("}").to_string(),
            }),
        );
        let right = escaped_close_brace(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn check_escaped_open_bracket() {
        let source = "\\[ alfa";
        let left = (
            " alfa",
            SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
                text: encode_quoted_attribute("[").to_string(),
            }),
        );
        let right = escaped_open_bracket(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn check_escaped_close_bracket() {
        let source = "\\] alfa";
        let left = (
            " alfa",
            SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
                text: encode_quoted_attribute("]").to_string(),
            }),
        );
        let right = escaped_close_bracket(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn check_escaped_pipe() {
        let source = "\\| alfa";
        let left = (
            " alfa",
            SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
                text: encode_quoted_attribute("|").to_string(),
            }),
        );
        let right = escaped_pipe(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn check_escaped_backslash() {
        let source = "\\\\ alfa";
        let left = (
            " alfa",
            SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
                text: encode_quoted_attribute("\\").to_string(),
            }),
        );
        let right = escaped_backslash(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn check_escaped_colon() {
        let source = "\\: alfa";
        let left = (
            " alfa",
            SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
                text: encode_quoted_attribute(":").to_string(),
            }),
        );
        let right = escaped_colon(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn check_escaped_semicolon() {
        let source = "\\; alfa";
        let left = (
            " alfa",
            SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
                text: encode_quoted_attribute(";").to_string(),
            }),
        );
        let right = escaped_semicolon(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn check_escaped_double_quote() {
        let source = "\\\" alfa";
        let left = (
            " alfa",
            SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
                text: encode_quoted_attribute("\"").to_string(),
            }),
        );
        let right = escaped_double_quote(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn check_escaped_single_quote() {
        let source = "\\' alfa";
        let left = (
            " alfa",
            SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
                text: encode_quoted_attribute("'").to_string(),
            }),
        );
        let right = escaped_single_quote(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn check_escaped_comma() {
        let source = "\\, alfa";
        let left = (
            " alfa",
            SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
                text: encode_quoted_attribute(",").to_string(),
            }),
        );
        let right = escaped_comma(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn check_escaped_less_than() {
        let source = "\\< alfa";
        let left = (
            " alfa",
            SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
                text: encode_quoted_attribute("<").to_string(),
            }),
        );
        let right = escaped_less_than(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn check_escaped_greater_than() {
        let source = "\\> alfa";
        let left = (
            " alfa",
            SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
                text: encode_quoted_attribute(">").to_string(),
            }),
        );
        let right = escaped_greater_than(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn check_escaped_period() {
        let source = "\\. alfa";
        let left = (
            " alfa",
            SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
                text: encode_quoted_attribute(".").to_string(),
            }),
        );
        let right = escaped_period(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn check_escaped_forward_slash() {
        let source = "\\/ alfa";
        let left = (
            " alfa",
            SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
                text: encode_quoted_attribute("/").to_string(),
            }),
        );
        let right = escaped_forward_slash(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn check_escaped_question_mark() {
        let source = "\\? alfa";
        let left = (
            " alfa",
            SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
                text: encode_quoted_attribute("?").to_string(),
            }),
        );
        let right = escaped_question_mark(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn check_escaped_backtick() {
        let source = "\\` alfa";
        let left = (
            " alfa",
            SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
                text: encode_quoted_attribute("`").to_string(),
            }),
        );
        let right = escaped_backtick(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn check_escaped_space() {
        let source = "\\  alfa";
        let left = (
            " alfa",
            SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
                text: encode_quoted_attribute(" ").to_string(),
            }),
        );
        let right = escaped_space(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn check_escaped_tag() {
        let source = "\\\t alfa";
        let left = (
            " alfa",
            SpanV42::EscapedCharacter(EscapedCharacterSpanV42 {
                text: encode_quoted_attribute("\t").to_string(),
            }),
        );
        let right = escaped_tag(source).unwrap();
        assert_eq!(left, right);
    }


}
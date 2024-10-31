use super::PlainTextSpan;
use super::SpanV42;
use nom::bytes::complete::is_a;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;


pub fn any_tildies(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, result) = is_a("~").context("").parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: result.to_string(),
        }),
    ))
}

pub fn any_exclamations(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, result) = is_a("!").context("").parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: result.to_string(),
        }),
    ))
}

pub fn any_ats(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, result) = is_a("@").context("").parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: result.to_string(),
        }),
    ))
}

pub fn any_pound_signs(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, result) = is_a("#").context("").parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: result.to_string(),
        }),
    ))
}

pub fn any_dollar_signs(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, result) = is_a("$").context("").parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: result.to_string(),
        }),
    ))
}

pub fn any_percent_signs(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, result) = is_a("%").context("").parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: result.to_string(),
        }),
    ))
}

pub fn any_carets(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, result) = is_a("^").context("").parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: result.to_string(),
        }),
    ))
}

pub fn any_ampersands(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, result) = is_a("&").context("").parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: result.to_string(),
        }),
    ))
}

pub fn any_asterisks(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, result) = is_a("*").context("").parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: result.to_string(),
        }),
    ))
}

pub fn any_open_parens(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, result) = is_a("(").context("").parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: result.to_string(),
        }),
    ))
}

pub fn any_close_parens(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, result) = is_a(")").context("").parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: result.to_string(),
        }),
    ))
}

pub fn any_underscores(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, result) = is_a("_").context("").parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: result.to_string(),
        }),
    ))
}

pub fn any_hyphens(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, result) = is_a("-").context("").parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: result.to_string(),
        }),
    ))
}

pub fn any_plus_signs(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, result) = is_a("+").context("").parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: result.to_string(),
        }),
    ))
}

pub fn any_equal_signs(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, result) = is_a("=").context("").parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: result.to_string(),
        }),
    ))
}

pub fn any_open_braces(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, result) = is_a("{").context("").parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: result.to_string(),
        }),
    ))
}

pub fn any_close_braces(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, result) = is_a("}").context("").parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: result.to_string(),
        }),
    ))
}

pub fn any_open_brackets(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, result) = is_a("[").context("").parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: result.to_string(),
        }),
    ))
}

pub fn any_close_brackets(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, result) = is_a("]").context("").parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: result.to_string(),
        }),
    ))
}

pub fn any_pipes(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, result) = is_a("|").context("").parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: result.to_string(),
        }),
    ))
}

pub fn any_back_slashes(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, result) = is_a("\\").context("").parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: result.to_string(),
        }),
    ))
}

pub fn any_colons(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, result) = is_a(":").context("").parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: result.to_string(),
        }),
    ))
}

pub fn any_semicolons(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, result) = is_a(";").context("").parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: result.to_string(),
        }),
    ))
}

pub fn any_double_quotes(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, result) = is_a("\"").context("").parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: result.to_string(),
        }),
    ))
}

pub fn any_single_quotes(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, result) = is_a("'").context("").parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: result.to_string(),
        }),
    ))
}

pub fn any_commas(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, result) = is_a(",").context("").parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: result.to_string(),
        }),
    ))
}

pub fn any_less_thans(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, result) = is_a("<").context("").parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: result.to_string(),
        }),
    ))
}

pub fn any_greater_thans(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, result) = is_a(">").context("").parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: result.to_string(),
        }),
    ))
}

pub fn any_periods(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, result) = is_a(".").context("").parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: result.to_string(),
        }),
    ))
}

pub fn any_forward_slashes(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, result) = is_a("/").context("").parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: result.to_string(),
        }),
    ))
}

pub fn any_question_marks(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, result) = is_a("?").context("").parse(source)?;
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
    fn single_character_tilde() {
        let source = "~ alfa";
        let left = (
            " alfa",
            SpanV42::PlainText(PlainTextSpan {
                text: "~".to_string(),
            }),
        );
        let right = any_tildies(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn allow_multiple_tilde() {
        let source = "~~ bravo";
        let left = (
            " bravo",
            SpanV42::PlainText(PlainTextSpan {
                text: "~~".to_string(),
            }),
        );
        let right = any_tildies(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn single_character_exclamation() {
        let source = "! alfa";
        let left = (
            " alfa",
            SpanV42::PlainText(PlainTextSpan {
                text: "!".to_string(),
            }),
        );
        let right = any_exclamations(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn allow_multiple_exclamation() {
        let source = "!! bravo";
        let left = (
            " bravo",
            SpanV42::PlainText(PlainTextSpan {
                text: "!!".to_string(),
            }),
        );
        let right = any_exclamations(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn single_character_at() {
        let source = "@ alfa";
        let left = (
            " alfa",
            SpanV42::PlainText(PlainTextSpan {
                text: "@".to_string(),
            }),
        );
        let right = any_ats(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn allow_multiple_at() {
        let source = "@@ bravo";
        let left = (
            " bravo",
            SpanV42::PlainText(PlainTextSpan {
                text: "@@".to_string(),
            }),
        );
        let right = any_ats(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn single_character_pound_sign() {
        let source = "# alfa";
        let left = (
            " alfa",
            SpanV42::PlainText(PlainTextSpan {
                text: "#".to_string(),
            }),
        );
        let right = any_pound_signs(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn allow_multiple_pound_sign() {
        let source = "## bravo";
        let left = (
            " bravo",
            SpanV42::PlainText(PlainTextSpan {
                text: "##".to_string(),
            }),
        );
        let right = any_pound_signs(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn single_character_dollar_sign() {
        let source = "$ alfa";
        let left = (
            " alfa",
            SpanV42::PlainText(PlainTextSpan {
                text: "$".to_string(),
            }),
        );
        let right = any_dollar_signs(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn allow_multiple_dollar_sign() {
        let source = "$$ bravo";
        let left = (
            " bravo",
            SpanV42::PlainText(PlainTextSpan {
                text: "$$".to_string(),
            }),
        );
        let right = any_dollar_signs(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn single_character_percent_sign() {
        let source = "% alfa";
        let left = (
            " alfa",
            SpanV42::PlainText(PlainTextSpan {
                text: "%".to_string(),
            }),
        );
        let right = any_percent_signs(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn allow_multiple_percent_sign() {
        let source = "%% bravo";
        let left = (
            " bravo",
            SpanV42::PlainText(PlainTextSpan {
                text: "%%".to_string(),
            }),
        );
        let right = any_percent_signs(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn single_character_caret() {
        let source = "^ alfa";
        let left = (
            " alfa",
            SpanV42::PlainText(PlainTextSpan {
                text: "^".to_string(),
            }),
        );
        let right = any_carets(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn allow_multiple_caret() {
        let source = "^^ bravo";
        let left = (
            " bravo",
            SpanV42::PlainText(PlainTextSpan {
                text: "^^".to_string(),
            }),
        );
        let right = any_carets(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn single_character_ampersand() {
        let source = "& alfa";
        let left = (
            " alfa",
            SpanV42::PlainText(PlainTextSpan {
                text: "&".to_string(),
            }),
        );
        let right = any_ampersands(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn allow_multiple_ampersand() {
        let source = "&& bravo";
        let left = (
            " bravo",
            SpanV42::PlainText(PlainTextSpan {
                text: "&&".to_string(),
            }),
        );
        let right = any_ampersands(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn single_character_asterisk() {
        let source = "* alfa";
        let left = (
            " alfa",
            SpanV42::PlainText(PlainTextSpan {
                text: "*".to_string(),
            }),
        );
        let right = any_asterisks(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn allow_multiple_asterisk() {
        let source = "** bravo";
        let left = (
            " bravo",
            SpanV42::PlainText(PlainTextSpan {
                text: "**".to_string(),
            }),
        );
        let right = any_asterisks(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn single_character_open_paren() {
        let source = "( alfa";
        let left = (
            " alfa",
            SpanV42::PlainText(PlainTextSpan {
                text: "(".to_string(),
            }),
        );
        let right = any_open_parens(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn allow_multiple_open_paren() {
        let source = "(( bravo";
        let left = (
            " bravo",
            SpanV42::PlainText(PlainTextSpan {
                text: "((".to_string(),
            }),
        );
        let right = any_open_parens(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn single_character_close_paren() {
        let source = ") alfa";
        let left = (
            " alfa",
            SpanV42::PlainText(PlainTextSpan {
                text: ")".to_string(),
            }),
        );
        let right = any_close_parens(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn allow_multiple_close_paren() {
        let source = ")) bravo";
        let left = (
            " bravo",
            SpanV42::PlainText(PlainTextSpan {
                text: "))".to_string(),
            }),
        );
        let right = any_close_parens(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn single_character_underscore() {
        let source = "_ alfa";
        let left = (
            " alfa",
            SpanV42::PlainText(PlainTextSpan {
                text: "_".to_string(),
            }),
        );
        let right = any_underscores(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn allow_multiple_underscore() {
        let source = "__ bravo";
        let left = (
            " bravo",
            SpanV42::PlainText(PlainTextSpan {
                text: "__".to_string(),
            }),
        );
        let right = any_underscores(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn single_character_hyphen() {
        let source = "- alfa";
        let left = (
            " alfa",
            SpanV42::PlainText(PlainTextSpan {
                text: "-".to_string(),
            }),
        );
        let right = any_hyphens(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn allow_multiple_hyphen() {
        let source = "-- bravo";
        let left = (
            " bravo",
            SpanV42::PlainText(PlainTextSpan {
                text: "--".to_string(),
            }),
        );
        let right = any_hyphens(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn single_character_plus_sign() {
        let source = "+ alfa";
        let left = (
            " alfa",
            SpanV42::PlainText(PlainTextSpan {
                text: "+".to_string(),
            }),
        );
        let right = any_plus_signs(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn allow_multiple_plus_sign() {
        let source = "++ bravo";
        let left = (
            " bravo",
            SpanV42::PlainText(PlainTextSpan {
                text: "++".to_string(),
            }),
        );
        let right = any_plus_signs(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn single_character_equal_sign() {
        let source = "= alfa";
        let left = (
            " alfa",
            SpanV42::PlainText(PlainTextSpan {
                text: "=".to_string(),
            }),
        );
        let right = any_equal_signs(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn allow_multiple_equal_sign() {
        let source = "== bravo";
        let left = (
            " bravo",
            SpanV42::PlainText(PlainTextSpan {
                text: "==".to_string(),
            }),
        );
        let right = any_equal_signs(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn single_character_open_brace() {
        let source = "{ alfa";
        let left = (
            " alfa",
            SpanV42::PlainText(PlainTextSpan {
                text: "{".to_string(),
            }),
        );
        let right = any_open_braces(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn allow_multiple_open_brace() {
        let source = "{{ bravo";
        let left = (
            " bravo",
            SpanV42::PlainText(PlainTextSpan {
                text: "{{".to_string(),
            }),
        );
        let right = any_open_braces(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn single_character_close_brace() {
        let source = "} alfa";
        let left = (
            " alfa",
            SpanV42::PlainText(PlainTextSpan {
                text: "}".to_string(),
            }),
        );
        let right = any_close_braces(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn allow_multiple_close_brace() {
        let source = "}} bravo";
        let left = (
            " bravo",
            SpanV42::PlainText(PlainTextSpan {
                text: "}}".to_string(),
            }),
        );
        let right = any_close_braces(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn single_character_open_bracket() {
        let source = "[ alfa";
        let left = (
            " alfa",
            SpanV42::PlainText(PlainTextSpan {
                text: "[".to_string(),
            }),
        );
        let right = any_open_brackets(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn allow_multiple_open_bracket() {
        let source = "[[ bravo";
        let left = (
            " bravo",
            SpanV42::PlainText(PlainTextSpan {
                text: "[[".to_string(),
            }),
        );
        let right = any_open_brackets(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn single_character_close_bracket() {
        let source = "] alfa";
        let left = (
            " alfa",
            SpanV42::PlainText(PlainTextSpan {
                text: "]".to_string(),
            }),
        );
        let right = any_close_brackets(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn allow_multiple_close_bracket() {
        let source = "]] bravo";
        let left = (
            " bravo",
            SpanV42::PlainText(PlainTextSpan {
                text: "]]".to_string(),
            }),
        );
        let right = any_close_brackets(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn single_character_pipe() {
        let source = "| alfa";
        let left = (
            " alfa",
            SpanV42::PlainText(PlainTextSpan {
                text: "|".to_string(),
            }),
        );
        let right = any_pipes(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn allow_multiple_pipe() {
        let source = "|| bravo";
        let left = (
            " bravo",
            SpanV42::PlainText(PlainTextSpan {
                text: "||".to_string(),
            }),
        );
        let right = any_pipes(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn single_character_back_slash() {
        let source = "\\ alfa";
        let left = (
            " alfa",
            SpanV42::PlainText(PlainTextSpan {
                text: "\\".to_string(),
            }),
        );
        let right = any_back_slashes(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn allow_multiple_back_slash() {
        let source = "\\\\ bravo";
        let left = (
            " bravo",
            SpanV42::PlainText(PlainTextSpan {
                text: "\\\\".to_string(),
            }),
        );
        let right = any_back_slashes(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn single_character_colon() {
        let source = ": alfa";
        let left = (
            " alfa",
            SpanV42::PlainText(PlainTextSpan {
                text: ":".to_string(),
            }),
        );
        let right = any_colons(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn allow_multiple_colon() {
        let source = ":: bravo";
        let left = (
            " bravo",
            SpanV42::PlainText(PlainTextSpan {
                text: "::".to_string(),
            }),
        );
        let right = any_colons(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn single_character_semicolon() {
        let source = "; alfa";
        let left = (
            " alfa",
            SpanV42::PlainText(PlainTextSpan {
                text: ";".to_string(),
            }),
        );
        let right = any_semicolons(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn allow_multiple_semicolon() {
        let source = ";; bravo";
        let left = (
            " bravo",
            SpanV42::PlainText(PlainTextSpan {
                text: ";;".to_string(),
            }),
        );
        let right = any_semicolons(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn single_character_double_quote() {
        let source = "\" alfa";
        let left = (
            " alfa",
            SpanV42::PlainText(PlainTextSpan {
                text: "\"".to_string(),
            }),
        );
        let right = any_double_quotes(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn allow_multiple_double_quote() {
        let source = "\"\" bravo";
        let left = (
            " bravo",
            SpanV42::PlainText(PlainTextSpan {
                text: "\"\"".to_string(),
            }),
        );
        let right = any_double_quotes(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn single_character_single_quote() {
        let source = "' alfa";
        let left = (
            " alfa",
            SpanV42::PlainText(PlainTextSpan {
                text: "'".to_string(),
            }),
        );
        let right = any_single_quotes(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn allow_multiple_single_quote() {
        let source = "'' bravo";
        let left = (
            " bravo",
            SpanV42::PlainText(PlainTextSpan {
                text: "''".to_string(),
            }),
        );
        let right = any_single_quotes(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn single_character_comma() {
        let source = ", alfa";
        let left = (
            " alfa",
            SpanV42::PlainText(PlainTextSpan {
                text: ",".to_string(),
            }),
        );
        let right = any_commas(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn allow_multiple_comma() {
        let source = ",, bravo";
        let left = (
            " bravo",
            SpanV42::PlainText(PlainTextSpan {
                text: ",,".to_string(),
            }),
        );
        let right = any_commas(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn single_character_less_than() {
        let source = "< alfa";
        let left = (
            " alfa",
            SpanV42::PlainText(PlainTextSpan {
                text: "<".to_string(),
            }),
        );
        let right = any_less_thans(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn allow_multiple_less_than() {
        let source = "<< bravo";
        let left = (
            " bravo",
            SpanV42::PlainText(PlainTextSpan {
                text: "<<".to_string(),
            }),
        );
        let right = any_less_thans(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn single_character_greater_than() {
        let source = "> alfa";
        let left = (
            " alfa",
            SpanV42::PlainText(PlainTextSpan {
                text: ">".to_string(),
            }),
        );
        let right = any_greater_thans(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn allow_multiple_greater_than() {
        let source = ">> bravo";
        let left = (
            " bravo",
            SpanV42::PlainText(PlainTextSpan {
                text: ">>".to_string(),
            }),
        );
        let right = any_greater_thans(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn single_character_period() {
        let source = ". alfa";
        let left = (
            " alfa",
            SpanV42::PlainText(PlainTextSpan {
                text: ".".to_string(),
            }),
        );
        let right = any_periods(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn allow_multiple_period() {
        let source = ".. bravo";
        let left = (
            " bravo",
            SpanV42::PlainText(PlainTextSpan {
                text: "..".to_string(),
            }),
        );
        let right = any_periods(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn single_character_forward_slash() {
        let source = "/ alfa";
        let left = (
            " alfa",
            SpanV42::PlainText(PlainTextSpan {
                text: "/".to_string(),
            }),
        );
        let right = any_forward_slashes(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn allow_multiple_forward_slash() {
        let source = "// bravo";
        let left = (
            " bravo",
            SpanV42::PlainText(PlainTextSpan {
                text: "//".to_string(),
            }),
        );
        let right = any_forward_slashes(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn single_character_question_mark() {
        let source = "? alfa";
        let left = (
            " alfa",
            SpanV42::PlainText(PlainTextSpan {
                text: "?".to_string(),
            }),
        );
        let right = any_question_marks(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn allow_multiple_question_mark() {
        let source = "?? bravo";
        let left = (
            " bravo",
            SpanV42::PlainText(PlainTextSpan {
                text: "??".to_string(),
            }),
        );
        let right = any_question_marks(source).unwrap();
        assert_eq!(left, right);
    }


}
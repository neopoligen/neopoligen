use crate::span_v42::*;
use nom::bytes::complete::tag;
use nom::sequence::terminated;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;


pub fn single_tilde(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, output) = terminated(tag("~"), not(tag("~")))
        .context("")
        .parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: output.to_string(),
        }),
    ))
}

pub fn single_exclamation(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, output) = terminated(tag("!"), not(tag("!")))
        .context("")
        .parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: output.to_string(),
        }),
    ))
}

pub fn single_at(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, output) = terminated(tag("@"), not(tag("@")))
        .context("")
        .parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: output.to_string(),
        }),
    ))
}

pub fn single_pound_sign(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, output) = terminated(tag("#"), not(tag("#")))
        .context("")
        .parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: output.to_string(),
        }),
    ))
}

pub fn single_dollar_sign(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, output) = terminated(tag("$"), not(tag("$")))
        .context("")
        .parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: output.to_string(),
        }),
    ))
}

pub fn single_percent_sign(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, output) = terminated(tag("%"), not(tag("%")))
        .context("")
        .parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: output.to_string(),
        }),
    ))
}

pub fn single_caret(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, output) = terminated(tag("^"), not(tag("^")))
        .context("")
        .parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: output.to_string(),
        }),
    ))
}

pub fn single_ampersand(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, output) = terminated(tag("&"), not(tag("&")))
        .context("")
        .parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: output.to_string(),
        }),
    ))
}

pub fn single_asterisk(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, output) = terminated(tag("*"), not(tag("*")))
        .context("")
        .parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: output.to_string(),
        }),
    ))
}

pub fn single_open_paren(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, output) = terminated(tag("("), not(tag("(")))
        .context("")
        .parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: output.to_string(),
        }),
    ))
}

pub fn single_close_paren(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, output) = terminated(tag(")"), not(tag(")")))
        .context("")
        .parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: output.to_string(),
        }),
    ))
}

pub fn single_underscore(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, output) = terminated(tag("_"), not(tag("_")))
        .context("")
        .parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: output.to_string(),
        }),
    ))
}

pub fn single_hyphen(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, output) = terminated(tag("-"), not(tag("-")))
        .context("")
        .parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: output.to_string(),
        }),
    ))
}

pub fn single_plus_sign(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, output) = terminated(tag("+"), not(tag("+")))
        .context("")
        .parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: output.to_string(),
        }),
    ))
}

pub fn single_equal_sign(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, output) = terminated(tag("="), not(tag("=")))
        .context("")
        .parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: output.to_string(),
        }),
    ))
}

pub fn single_open_brace(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, output) = terminated(tag("{"), not(tag("{")))
        .context("")
        .parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: output.to_string(),
        }),
    ))
}

pub fn single_close_brace(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, output) = terminated(tag("}"), not(tag("}")))
        .context("")
        .parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: output.to_string(),
        }),
    ))
}

pub fn single_open_bracket(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, output) = terminated(tag("["), not(tag("[")))
        .context("")
        .parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: output.to_string(),
        }),
    ))
}

pub fn single_close_bracket(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, output) = terminated(tag("]"), not(tag("]")))
        .context("")
        .parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: output.to_string(),
        }),
    ))
}

pub fn single_pipe(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, output) = terminated(tag("|"), not(tag("|")))
        .context("")
        .parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: output.to_string(),
        }),
    ))
}

pub fn single_backslash(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, output) = terminated(tag("\\"), not(tag("\\")))
        .context("")
        .parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: output.to_string(),
        }),
    ))
}

pub fn single_colon(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, output) = terminated(tag(":"), not(tag(":")))
        .context("")
        .parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: output.to_string(),
        }),
    ))
}

pub fn single_semicolon(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, output) = terminated(tag(";"), not(tag(";")))
        .context("")
        .parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: output.to_string(),
        }),
    ))
}

pub fn single_double_quote(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, output) = terminated(tag("\""), not(tag("\"")))
        .context("")
        .parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: output.to_string(),
        }),
    ))
}

pub fn single_single_quote(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, output) = terminated(tag("'"), not(tag("'")))
        .context("")
        .parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: output.to_string(),
        }),
    ))
}

pub fn single_comma(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, output) = terminated(tag(","), not(tag(",")))
        .context("")
        .parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: output.to_string(),
        }),
    ))
}

pub fn single_less_than(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, output) = terminated(tag("<"), not(tag("<")))
        .context("")
        .parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: output.to_string(),
        }),
    ))
}

pub fn single_greater_than(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, output) = terminated(tag(">"), not(tag(">")))
        .context("")
        .parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: output.to_string(),
        }),
    ))
}

pub fn single_period(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, output) = terminated(tag("."), not(tag(".")))
        .context("")
        .parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: output.to_string(),
        }),
    ))
}

pub fn single_forward_slash(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, output) = terminated(tag("/"), not(tag("/")))
        .context("")
        .parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: output.to_string(),
        }),
    ))
}

pub fn single_question_mark(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, output) = terminated(tag("?"), not(tag("?")))
        .context("")
        .parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: output.to_string(),
        }),
    ))
}

pub fn single_backtick(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, output) = terminated(tag("`"), not(tag("`")))
        .context("")
        .parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: output.to_string(),
        }),
    ))
}


#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;


    #[test]
    fn single_at_eof_tilde() {
        let source = "~";
        let left = (
            "",
            SpanV42::PlainText(PlainTextSpan {
                text: "~".to_string(),
            }),
        );
        let right = single_tilde(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn single_with_more_content_tilde() {
        let source = "~ alfa";
        let left = (
            " alfa",
            SpanV42::PlainText(PlainTextSpan {
                text: "~".to_string(),
            }),
        );
        let right = single_tilde(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn prevent_mutiple_tildies() {
        let source = "~~";
        assert!(single_tilde(source).is_err());
    }


    #[test]
    fn single_at_eof_exclamation() {
        let source = "!";
        let left = (
            "",
            SpanV42::PlainText(PlainTextSpan {
                text: "!".to_string(),
            }),
        );
        let right = single_exclamation(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn single_with_more_content_exclamation() {
        let source = "! alfa";
        let left = (
            " alfa",
            SpanV42::PlainText(PlainTextSpan {
                text: "!".to_string(),
            }),
        );
        let right = single_exclamation(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn prevent_mutiple_exclamations() {
        let source = "!!";
        assert!(single_exclamation(source).is_err());
    }


    #[test]
    fn single_at_eof_at() {
        let source = "@";
        let left = (
            "",
            SpanV42::PlainText(PlainTextSpan {
                text: "@".to_string(),
            }),
        );
        let right = single_at(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn single_with_more_content_at() {
        let source = "@ alfa";
        let left = (
            " alfa",
            SpanV42::PlainText(PlainTextSpan {
                text: "@".to_string(),
            }),
        );
        let right = single_at(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn prevent_mutiple_ats() {
        let source = "@@";
        assert!(single_at(source).is_err());
    }


    #[test]
    fn single_at_eof_pound_sign() {
        let source = "#";
        let left = (
            "",
            SpanV42::PlainText(PlainTextSpan {
                text: "#".to_string(),
            }),
        );
        let right = single_pound_sign(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn single_with_more_content_pound_sign() {
        let source = "# alfa";
        let left = (
            " alfa",
            SpanV42::PlainText(PlainTextSpan {
                text: "#".to_string(),
            }),
        );
        let right = single_pound_sign(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn prevent_mutiple_pound_signs() {
        let source = "##";
        assert!(single_pound_sign(source).is_err());
    }


    #[test]
    fn single_at_eof_dollar_sign() {
        let source = "$";
        let left = (
            "",
            SpanV42::PlainText(PlainTextSpan {
                text: "$".to_string(),
            }),
        );
        let right = single_dollar_sign(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn single_with_more_content_dollar_sign() {
        let source = "$ alfa";
        let left = (
            " alfa",
            SpanV42::PlainText(PlainTextSpan {
                text: "$".to_string(),
            }),
        );
        let right = single_dollar_sign(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn prevent_mutiple_dollar_signs() {
        let source = "$$";
        assert!(single_dollar_sign(source).is_err());
    }


    #[test]
    fn single_at_eof_percent_sign() {
        let source = "%";
        let left = (
            "",
            SpanV42::PlainText(PlainTextSpan {
                text: "%".to_string(),
            }),
        );
        let right = single_percent_sign(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn single_with_more_content_percent_sign() {
        let source = "% alfa";
        let left = (
            " alfa",
            SpanV42::PlainText(PlainTextSpan {
                text: "%".to_string(),
            }),
        );
        let right = single_percent_sign(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn prevent_mutiple_percent_signs() {
        let source = "%%";
        assert!(single_percent_sign(source).is_err());
    }


    #[test]
    fn single_at_eof_caret() {
        let source = "^";
        let left = (
            "",
            SpanV42::PlainText(PlainTextSpan {
                text: "^".to_string(),
            }),
        );
        let right = single_caret(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn single_with_more_content_caret() {
        let source = "^ alfa";
        let left = (
            " alfa",
            SpanV42::PlainText(PlainTextSpan {
                text: "^".to_string(),
            }),
        );
        let right = single_caret(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn prevent_mutiple_carets() {
        let source = "^^";
        assert!(single_caret(source).is_err());
    }


    #[test]
    fn single_at_eof_ampersand() {
        let source = "&";
        let left = (
            "",
            SpanV42::PlainText(PlainTextSpan {
                text: "&".to_string(),
            }),
        );
        let right = single_ampersand(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn single_with_more_content_ampersand() {
        let source = "& alfa";
        let left = (
            " alfa",
            SpanV42::PlainText(PlainTextSpan {
                text: "&".to_string(),
            }),
        );
        let right = single_ampersand(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn prevent_mutiple_ampersands() {
        let source = "&&";
        assert!(single_ampersand(source).is_err());
    }


    #[test]
    fn single_at_eof_asterisk() {
        let source = "*";
        let left = (
            "",
            SpanV42::PlainText(PlainTextSpan {
                text: "*".to_string(),
            }),
        );
        let right = single_asterisk(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn single_with_more_content_asterisk() {
        let source = "* alfa";
        let left = (
            " alfa",
            SpanV42::PlainText(PlainTextSpan {
                text: "*".to_string(),
            }),
        );
        let right = single_asterisk(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn prevent_mutiple_asterisks() {
        let source = "**";
        assert!(single_asterisk(source).is_err());
    }


    #[test]
    fn single_at_eof_open_paren() {
        let source = "(";
        let left = (
            "",
            SpanV42::PlainText(PlainTextSpan {
                text: "(".to_string(),
            }),
        );
        let right = single_open_paren(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn single_with_more_content_open_paren() {
        let source = "( alfa";
        let left = (
            " alfa",
            SpanV42::PlainText(PlainTextSpan {
                text: "(".to_string(),
            }),
        );
        let right = single_open_paren(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn prevent_mutiple_open_parens() {
        let source = "((";
        assert!(single_open_paren(source).is_err());
    }


    #[test]
    fn single_at_eof_close_paren() {
        let source = ")";
        let left = (
            "",
            SpanV42::PlainText(PlainTextSpan {
                text: ")".to_string(),
            }),
        );
        let right = single_close_paren(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn single_with_more_content_close_paren() {
        let source = ") alfa";
        let left = (
            " alfa",
            SpanV42::PlainText(PlainTextSpan {
                text: ")".to_string(),
            }),
        );
        let right = single_close_paren(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn prevent_mutiple_close_parens() {
        let source = "))";
        assert!(single_close_paren(source).is_err());
    }


    #[test]
    fn single_at_eof_underscore() {
        let source = "_";
        let left = (
            "",
            SpanV42::PlainText(PlainTextSpan {
                text: "_".to_string(),
            }),
        );
        let right = single_underscore(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn single_with_more_content_underscore() {
        let source = "_ alfa";
        let left = (
            " alfa",
            SpanV42::PlainText(PlainTextSpan {
                text: "_".to_string(),
            }),
        );
        let right = single_underscore(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn prevent_mutiple_underscores() {
        let source = "__";
        assert!(single_underscore(source).is_err());
    }


    #[test]
    fn single_at_eof_hyphen() {
        let source = "-";
        let left = (
            "",
            SpanV42::PlainText(PlainTextSpan {
                text: "-".to_string(),
            }),
        );
        let right = single_hyphen(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn single_with_more_content_hyphen() {
        let source = "- alfa";
        let left = (
            " alfa",
            SpanV42::PlainText(PlainTextSpan {
                text: "-".to_string(),
            }),
        );
        let right = single_hyphen(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn prevent_mutiple_hyphens() {
        let source = "--";
        assert!(single_hyphen(source).is_err());
    }


    #[test]
    fn single_at_eof_plus_sign() {
        let source = "+";
        let left = (
            "",
            SpanV42::PlainText(PlainTextSpan {
                text: "+".to_string(),
            }),
        );
        let right = single_plus_sign(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn single_with_more_content_plus_sign() {
        let source = "+ alfa";
        let left = (
            " alfa",
            SpanV42::PlainText(PlainTextSpan {
                text: "+".to_string(),
            }),
        );
        let right = single_plus_sign(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn prevent_mutiple_plus_signs() {
        let source = "++";
        assert!(single_plus_sign(source).is_err());
    }


    #[test]
    fn single_at_eof_equal_sign() {
        let source = "=";
        let left = (
            "",
            SpanV42::PlainText(PlainTextSpan {
                text: "=".to_string(),
            }),
        );
        let right = single_equal_sign(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn single_with_more_content_equal_sign() {
        let source = "= alfa";
        let left = (
            " alfa",
            SpanV42::PlainText(PlainTextSpan {
                text: "=".to_string(),
            }),
        );
        let right = single_equal_sign(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn prevent_mutiple_equal_signs() {
        let source = "==";
        assert!(single_equal_sign(source).is_err());
    }


    #[test]
    fn single_at_eof_open_brace() {
        let source = "{";
        let left = (
            "",
            SpanV42::PlainText(PlainTextSpan {
                text: "{".to_string(),
            }),
        );
        let right = single_open_brace(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn single_with_more_content_open_brace() {
        let source = "{ alfa";
        let left = (
            " alfa",
            SpanV42::PlainText(PlainTextSpan {
                text: "{".to_string(),
            }),
        );
        let right = single_open_brace(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn prevent_mutiple_open_braces() {
        let source = "{{";
        assert!(single_open_brace(source).is_err());
    }


    #[test]
    fn single_at_eof_close_brace() {
        let source = "}";
        let left = (
            "",
            SpanV42::PlainText(PlainTextSpan {
                text: "}".to_string(),
            }),
        );
        let right = single_close_brace(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn single_with_more_content_close_brace() {
        let source = "} alfa";
        let left = (
            " alfa",
            SpanV42::PlainText(PlainTextSpan {
                text: "}".to_string(),
            }),
        );
        let right = single_close_brace(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn prevent_mutiple_close_braces() {
        let source = "}}";
        assert!(single_close_brace(source).is_err());
    }


    #[test]
    fn single_at_eof_open_bracket() {
        let source = "[";
        let left = (
            "",
            SpanV42::PlainText(PlainTextSpan {
                text: "[".to_string(),
            }),
        );
        let right = single_open_bracket(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn single_with_more_content_open_bracket() {
        let source = "[ alfa";
        let left = (
            " alfa",
            SpanV42::PlainText(PlainTextSpan {
                text: "[".to_string(),
            }),
        );
        let right = single_open_bracket(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn prevent_mutiple_open_brackets() {
        let source = "[[";
        assert!(single_open_bracket(source).is_err());
    }


    #[test]
    fn single_at_eof_close_bracket() {
        let source = "]";
        let left = (
            "",
            SpanV42::PlainText(PlainTextSpan {
                text: "]".to_string(),
            }),
        );
        let right = single_close_bracket(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn single_with_more_content_close_bracket() {
        let source = "] alfa";
        let left = (
            " alfa",
            SpanV42::PlainText(PlainTextSpan {
                text: "]".to_string(),
            }),
        );
        let right = single_close_bracket(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn prevent_mutiple_close_brackets() {
        let source = "]]";
        assert!(single_close_bracket(source).is_err());
    }


    #[test]
    fn single_at_eof_pipe() {
        let source = "|";
        let left = (
            "",
            SpanV42::PlainText(PlainTextSpan {
                text: "|".to_string(),
            }),
        );
        let right = single_pipe(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn single_with_more_content_pipe() {
        let source = "| alfa";
        let left = (
            " alfa",
            SpanV42::PlainText(PlainTextSpan {
                text: "|".to_string(),
            }),
        );
        let right = single_pipe(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn prevent_mutiple_pipes() {
        let source = "||";
        assert!(single_pipe(source).is_err());
    }


    #[test]
    fn single_at_eof_backslash() {
        let source = "\\";
        let left = (
            "",
            SpanV42::PlainText(PlainTextSpan {
                text: "\\".to_string(),
            }),
        );
        let right = single_backslash(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn single_with_more_content_backslash() {
        let source = "\\ alfa";
        let left = (
            " alfa",
            SpanV42::PlainText(PlainTextSpan {
                text: "\\".to_string(),
            }),
        );
        let right = single_backslash(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn prevent_mutiple_backslashes() {
        let source = "\\\\";
        assert!(single_backslash(source).is_err());
    }


    #[test]
    fn single_at_eof_colon() {
        let source = ":";
        let left = (
            "",
            SpanV42::PlainText(PlainTextSpan {
                text: ":".to_string(),
            }),
        );
        let right = single_colon(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn single_with_more_content_colon() {
        let source = ": alfa";
        let left = (
            " alfa",
            SpanV42::PlainText(PlainTextSpan {
                text: ":".to_string(),
            }),
        );
        let right = single_colon(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn prevent_mutiple_colons() {
        let source = "::";
        assert!(single_colon(source).is_err());
    }


    #[test]
    fn single_at_eof_semicolon() {
        let source = ";";
        let left = (
            "",
            SpanV42::PlainText(PlainTextSpan {
                text: ";".to_string(),
            }),
        );
        let right = single_semicolon(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn single_with_more_content_semicolon() {
        let source = "; alfa";
        let left = (
            " alfa",
            SpanV42::PlainText(PlainTextSpan {
                text: ";".to_string(),
            }),
        );
        let right = single_semicolon(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn prevent_mutiple_semicolons() {
        let source = ";;";
        assert!(single_semicolon(source).is_err());
    }


    #[test]
    fn single_at_eof_double_quote() {
        let source = "\"";
        let left = (
            "",
            SpanV42::PlainText(PlainTextSpan {
                text: "\"".to_string(),
            }),
        );
        let right = single_double_quote(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn single_with_more_content_double_quote() {
        let source = "\" alfa";
        let left = (
            " alfa",
            SpanV42::PlainText(PlainTextSpan {
                text: "\"".to_string(),
            }),
        );
        let right = single_double_quote(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn prevent_mutiple_double_quotes() {
        let source = "\"\"";
        assert!(single_double_quote(source).is_err());
    }


    #[test]
    fn single_at_eof_single_quote() {
        let source = "'";
        let left = (
            "",
            SpanV42::PlainText(PlainTextSpan {
                text: "'".to_string(),
            }),
        );
        let right = single_single_quote(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn single_with_more_content_single_quote() {
        let source = "' alfa";
        let left = (
            " alfa",
            SpanV42::PlainText(PlainTextSpan {
                text: "'".to_string(),
            }),
        );
        let right = single_single_quote(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn prevent_mutiple_single_quotes() {
        let source = "''";
        assert!(single_single_quote(source).is_err());
    }


    #[test]
    fn single_at_eof_comma() {
        let source = ",";
        let left = (
            "",
            SpanV42::PlainText(PlainTextSpan {
                text: ",".to_string(),
            }),
        );
        let right = single_comma(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn single_with_more_content_comma() {
        let source = ", alfa";
        let left = (
            " alfa",
            SpanV42::PlainText(PlainTextSpan {
                text: ",".to_string(),
            }),
        );
        let right = single_comma(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn prevent_mutiple_commas() {
        let source = ",,";
        assert!(single_comma(source).is_err());
    }


    #[test]
    fn single_at_eof_less_than() {
        let source = "<";
        let left = (
            "",
            SpanV42::PlainText(PlainTextSpan {
                text: "<".to_string(),
            }),
        );
        let right = single_less_than(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn single_with_more_content_less_than() {
        let source = "< alfa";
        let left = (
            " alfa",
            SpanV42::PlainText(PlainTextSpan {
                text: "<".to_string(),
            }),
        );
        let right = single_less_than(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn prevent_mutiple_less_thans() {
        let source = "<<";
        assert!(single_less_than(source).is_err());
    }


    #[test]
    fn single_at_eof_greater_than() {
        let source = ">";
        let left = (
            "",
            SpanV42::PlainText(PlainTextSpan {
                text: ">".to_string(),
            }),
        );
        let right = single_greater_than(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn single_with_more_content_greater_than() {
        let source = "> alfa";
        let left = (
            " alfa",
            SpanV42::PlainText(PlainTextSpan {
                text: ">".to_string(),
            }),
        );
        let right = single_greater_than(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn prevent_mutiple_greater_thans() {
        let source = ">>";
        assert!(single_greater_than(source).is_err());
    }


    #[test]
    fn single_at_eof_period() {
        let source = ".";
        let left = (
            "",
            SpanV42::PlainText(PlainTextSpan {
                text: ".".to_string(),
            }),
        );
        let right = single_period(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn single_with_more_content_period() {
        let source = ". alfa";
        let left = (
            " alfa",
            SpanV42::PlainText(PlainTextSpan {
                text: ".".to_string(),
            }),
        );
        let right = single_period(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn prevent_mutiple_periods() {
        let source = "..";
        assert!(single_period(source).is_err());
    }


    #[test]
    fn single_at_eof_forward_slash() {
        let source = "/";
        let left = (
            "",
            SpanV42::PlainText(PlainTextSpan {
                text: "/".to_string(),
            }),
        );
        let right = single_forward_slash(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn single_with_more_content_forward_slash() {
        let source = "/ alfa";
        let left = (
            " alfa",
            SpanV42::PlainText(PlainTextSpan {
                text: "/".to_string(),
            }),
        );
        let right = single_forward_slash(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn prevent_mutiple_forward_slashes() {
        let source = "//";
        assert!(single_forward_slash(source).is_err());
    }


    #[test]
    fn single_at_eof_question_mark() {
        let source = "?";
        let left = (
            "",
            SpanV42::PlainText(PlainTextSpan {
                text: "?".to_string(),
            }),
        );
        let right = single_question_mark(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn single_with_more_content_question_mark() {
        let source = "? alfa";
        let left = (
            " alfa",
            SpanV42::PlainText(PlainTextSpan {
                text: "?".to_string(),
            }),
        );
        let right = single_question_mark(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn prevent_mutiple_question_marks() {
        let source = "??";
        assert!(single_question_mark(source).is_err());
    }


    #[test]
    fn single_at_eof_backtick() {
        let source = "`";
        let left = (
            "",
            SpanV42::PlainText(PlainTextSpan {
                text: "`".to_string(),
            }),
        );
        let right = single_backtick(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn single_with_more_content_backtick() {
        let source = "` alfa";
        let left = (
            " alfa",
            SpanV42::PlainText(PlainTextSpan {
                text: "`".to_string(),
            }),
        );
        let right = single_backtick(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn prevent_mutiple_backticks() {
        let source = "``";
        assert!(single_backtick(source).is_err());
    }



}
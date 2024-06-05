use crate::span::*;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::multi::many0;
use nom::multi::many1;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;

pub fn code_shorthand(source: &str) -> IResult<&str, Span, ErrorTree<&str>> {
    let initial_source = source;
    let (source, _) = tag("``").context("").parse(source)?;
    let (source, parts) = many1(alt((
        wordpart,
        space,
        newline,
        hyphen,
        colon,
        single_lessthan,
        single_greaterthan,
        single_backtick,
        escaped_backtick,
        escaped_pipe,
        escaped_greaterthan,
        escaped_backslash,
        non_escape_backslash,
    )))
    .context("")
    .parse(source)?;
    let (source, attrs) = many0(code_shorthand_attr).context("").parse(source)?;
    let (source, _) = tag("``").context("").parse(source)?;
    let source_text = initial_source.replace(source, "").to_string();
    let parsed_text = parts
        .iter()
        .map(|word| word.parsed_text.clone())
        .collect::<Vec<String>>()
        .join("");
    Ok((
        source,
        Span {
            attrs,
            source_text,
            parsed_text,
            kind: SpanKind::CodeShorthand,
        },
    ))
}

// pub fn code_shorthand_text(source: &str) -> IResult<&str, Span, ErrorTree<&str>> {
//     let (source, span) = alt((wordpart, space, newline, single_backtick, escaped_pipe))
//         .context("")
//         .parse(source)?;
//     Ok((source, span))
// }

pub fn code_shorthand_attr(source: &str) -> IResult<&str, SpanAttr, ErrorTree<&str>> {
    let (source, attr) = alt((code_shorthand_key_value_attr, code_shorthand_flag_attr))
        .context("")
        .parse(source)?;
    Ok((source, attr))
}

pub fn code_shorthand_flag_attr(source: &str) -> IResult<&str, SpanAttr, ErrorTree<&str>> {
    let (source, the_tag) = tag("|").context("").parse(source)?;
    // TODO: Allow spaces here
    let (source, words) = many1(alt((
        wordpart,
        space,
        newline,
        colon,
        hyphen,
        single_lessthan,
        single_greaterthan,
        single_backtick,
        escaped_backtick,
        escaped_pipe,
        escaped_greaterthan,
        escaped_backslash,
    )))
    .context("")
    .parse(source)?;
    let source_text = words
        .iter()
        .map(|word| word.source_text.clone())
        .collect::<Vec<String>>()
        .join("");
    let value = words
        .iter()
        .map(|word| word.parsed_text.clone())
        .collect::<Vec<String>>()
        .join("");
    let attr = SpanAttr {
        source_text: format!("{}{}", the_tag, source_text),
        kind: SpanAttrKind::Flag { value },
    };
    Ok((source, attr))
}

pub fn code_shorthand_key_value_attr(source: &str) -> IResult<&str, SpanAttr, ErrorTree<&str>> {
    let initial_source = source;
    let (source, _) = tag("|").context("").parse(source)?;
    // TODO: Allow spaces here
    let (source, key_parts) = many1(alt((
        wordpart,
        hyphen,
        single_lessthan,
        single_greaterthan,
        single_backtick,
        escaped_backtick,
        escaped_pipe,
        escaped_greaterthan,
        escaped_backslash,
    )))
    .context("")
    .parse(source)?;
    let (source, _) = tag(":").context("").parse(source)?;
    let (source, _) = space1.context("").parse(source)?;
    let (source, tokens) = many1(alt((
        wordpart,
        space,
        newline,
        colon,
        hyphen,
        single_lessthan,
        single_greaterthan,
        single_backtick,
        escaped_backtick,
        escaped_pipe,
        escaped_greaterthan,
        escaped_backslash,
    )))
    .context("")
    .parse(source)?;
    let key = key_parts
        .iter()
        .map(|p| p.parsed_text.clone())
        .collect::<Vec<String>>()
        .join(" ");
    let value = tokens
        .iter()
        .map(|word| word.parsed_text.clone())
        .collect::<Vec<String>>()
        .join("");
    let source_text = initial_source.replace(source, "").to_string();
    let attr = SpanAttr {
        source_text,
        kind: SpanAttrKind::KeyValue {
            key: key.to_string(),
            value,
        },
    };
    Ok((source, attr))
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;
    #[rstest]
    #[case("``alfa-bravo``", "", "hyphen in text")]
    #[case("``alfa\\\\bravo``", "", "escaped backslash in text")]
    #[case("``alfa\\bravo``", "", "non-escaped backslash in text")]
    #[case("``gap|css``", "", "pipe for attr")]
    #[case("``margin: 0|css``", "", "colon in text")]
    fn run_test(#[case] input: &str, #[case] left: &str, #[case] _description: &str) {
        let right = code_shorthand(input).unwrap().0;
        assert_eq!(left, right, "asdf");
    }

    #[rstest]
    #[case("``\\``", "can't have a single backslash at the end of code text")]
    fn error_cases(#[case] input: &str, #[case] _desc: &str) {
        assert!(code_shorthand(input).is_err());
    }

    #[test]
    fn code_shorthand_basic() {
        let source = "``ping``";
        let left = (
            "",
            Span {
                attrs: vec![],
                source_text: "``ping``".to_string(),
                parsed_text: "ping".to_string(),
                kind: SpanKind::CodeShorthand,
            },
        );
        let right = code_shorthand(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn code_shorthand_with_escaped_pipe() {
        let source = "``ping\\|ping``";
        let left = (
            "",
            Span {
                attrs: vec![],
                source_text: "``ping\\|ping``".to_string(),
                parsed_text: "ping|ping".to_string(),
                kind: SpanKind::CodeShorthand,
            },
        );
        let right = code_shorthand(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn code_shorthand_with_escaped_backslash() {
        let source = "``ping\\\\ping``";
        let left = (
            "",
            Span {
                attrs: vec![],
                source_text: "``ping\\\\ping``".to_string(),
                parsed_text: "ping\\ping".to_string(),
                kind: SpanKind::CodeShorthand,
            },
        );
        let right = code_shorthand(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn code_shorthand_with_escaped_backtick() {
        let source = "``ping\\`ping``";
        let left = (
            "",
            Span {
                attrs: vec![],
                source_text: "``ping\\`ping``".to_string(),
                parsed_text: "ping`ping".to_string(),
                kind: SpanKind::CodeShorthand,
            },
        );
        let right = code_shorthand(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn code_shorthand_with_flag_attr() {
        let source = "``code|rust``";
        let left = (
            "",
            Span {
                attrs: vec![SpanAttr {
                    source_text: "|rust".to_string(),
                    kind: SpanAttrKind::Flag {
                        value: "rust".to_string(),
                    },
                }],
                source_text: "``code|rust``".to_string(),
                parsed_text: "code".to_string(),
                kind: SpanKind::CodeShorthand {},
            },
        );
        let right = code_shorthand(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn code_shorthand_with_multiple_flag_attrs() {
        let source = "``code|rust|hidden``";
        let left = (
            "",
            Span {
                attrs: vec![
                    SpanAttr {
                        source_text: "|rust".to_string(),
                        kind: SpanAttrKind::Flag {
                            value: "rust".to_string(),
                        },
                    },
                    SpanAttr {
                        source_text: "|hidden".to_string(),
                        kind: SpanAttrKind::Flag {
                            value: "hidden".to_string(),
                        },
                    },
                ],
                source_text: "``code|rust|hidden``".to_string(),
                parsed_text: "code".to_string(),
                kind: SpanKind::CodeShorthand,
            },
        );
        let right = code_shorthand(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn flag_attr_for_code_basic_end_at_block() {
        let source = "|rust``";
        let left = (
            "``",
            SpanAttr {
                source_text: "|rust".to_string(),
                kind: SpanAttrKind::Flag {
                    value: "rust".to_string(),
                },
            },
        );
        let right = code_shorthand_flag_attr(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn flag_attr_for_code_url() {
        let source = "|https://www.example.com``";
        let left = (
            "``",
            SpanAttr {
                source_text: "|https://www.example.com".to_string(),
                kind: SpanAttrKind::Flag {
                    value: "https://www.example.com".to_string(),
                },
            },
        );
        let right = code_shorthand_flag_attr(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn flag_attr_for_code_with_pipe_escape() {
        let source = "|rust\\|here``";
        let left = (
            "``",
            SpanAttr {
                source_text: "|rust\\|here".to_string(),
                kind: SpanAttrKind::Flag {
                    value: "rust|here".to_string(),
                },
            },
        );
        let right = code_shorthand_flag_attr(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn integration_1_basic() {
        let source = "alfa ``bravo`` charlie";
        let left = (
            "",
            vec![
                Span {
                    attrs: vec![],
                    kind: SpanKind::WordPart,
                    parsed_text: "alfa ".to_string(),
                    source_text: "alfa ".to_string(),
                },
                Span {
                    attrs: vec![],
                    kind: SpanKind::CodeShorthand,
                    parsed_text: "bravo".to_string(),
                    source_text: "``bravo``".to_string(),
                },
                Span {
                    attrs: vec![],
                    kind: SpanKind::WordPart,
                    parsed_text: " charlie".to_string(),
                    source_text: " charlie".to_string(),
                },
            ],
        );
        let right = many1(span_for_body_text).parse(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn integration_2_escaped_pipe() {
        let source = r#"alfa ``bravo\|delta`` charlie"#;
        let left = (
            "",
            vec![
                Span {
                    attrs: vec![],
                    kind: SpanKind::WordPart,
                    parsed_text: "alfa ".to_string(),
                    source_text: "alfa ".to_string(),
                },
                Span {
                    attrs: vec![],
                    kind: SpanKind::CodeShorthand,
                    parsed_text: "bravo|delta".to_string(),
                    source_text: "``bravo\\|delta``".to_string(),
                },
                Span {
                    attrs: vec![],
                    kind: SpanKind::WordPart,
                    parsed_text: " charlie".to_string(),
                    source_text: " charlie".to_string(),
                },
            ],
        );
        let right = many1(span_for_body_text).parse(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn integration_2_escaped_single_backtick() {
        let source = r#"alfa ``bravo`delta`` charlie"#;
        let left = (
            "",
            vec![
                Span {
                    attrs: vec![],
                    kind: SpanKind::WordPart,
                    parsed_text: "alfa ".to_string(),
                    source_text: "alfa ".to_string(),
                },
                Span {
                    attrs: vec![],
                    kind: SpanKind::CodeShorthand,
                    parsed_text: "bravo`delta".to_string(),
                    source_text: "``bravo`delta``".to_string(),
                },
                Span {
                    attrs: vec![],
                    kind: SpanKind::WordPart,
                    parsed_text: " charlie".to_string(),
                    source_text: " charlie".to_string(),
                },
            ],
        );
        let right = many1(span_for_body_text).parse(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn code_shorthand_key_value_attrs_basic_key_value_attr() {
        let source = "|class: green";
        let left = (
            "",
            SpanAttr {
                source_text: "|class: green".to_string(),
                kind: SpanAttrKind::KeyValue {
                    key: "class".to_string(),
                    value: "green".to_string(),
                },
            },
        );
        let right = code_shorthand_key_value_attr(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn code_shorthand_basic_with_key_value() {
        let source = "``ping|class: blue``";
        let left = (
            "",
            Span {
                attrs: vec![SpanAttr {
                    source_text: "|class: blue".to_string(),
                    kind: SpanAttrKind::KeyValue {
                        key: "class".to_string(),
                        value: "blue".to_string(),
                    },
                }],
                source_text: "``ping|class: blue``".to_string(),
                parsed_text: "ping".to_string(),
                kind: SpanKind::CodeShorthand {},
            },
        );
        let right = code_shorthand(source).unwrap();
        assert_eq!(left, right);
    }

    //
}

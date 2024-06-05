use crate::span::escaped_colon::*;
use crate::span::*;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::character::complete::digit1;
use nom::multi::many0;
use nom::multi::many1;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;

pub fn named_span(source: &str) -> IResult<&str, Span, ErrorTree<&str>> {
    let initial_source = source;
    let (source, _) = tag("<<").context("").parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    let (source, type_parts) = many1(alt((alpha1, digit1, tag("-"), tag("_"))))
        .context("")
        .parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    let (source, _) = tag("|").context("").parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    let (source, spans) = many0(alt((
        wordpart,
        newline,
        space,
        colon,
        hyphen,
        named_span,
        single_lessthan,
        single_greaterthan,
        single_backtick,
        escaped_backtick,
        escaped_colon,
        escaped_pipe,
        escaped_greaterthan,
        escaped_backslash,
        non_escape_backslash,
    )))
    .context("")
    .parse(source)?;
    let (source, attrs) = many0(named_span_attr).context("").parse(source)?;
    let (source, _) = tag(">>").context("").parse(source)?;
    let source_text = initial_source.replace(source, "").to_string();
    let parsed_text = spans
        .iter()
        .map(|word| word.parsed_text.clone())
        .collect::<Vec<String>>()
        .join("");
    let r#type = type_parts
        .iter()
        .map(|p| p.to_string())
        .collect::<Vec<String>>()
        .join("");
    Ok((
        source,
        Span {
            attrs,
            source_text,
            parsed_text,
            kind: SpanKind::NamedSpan {
                r#type: r#type.to_string(),
                spans,
            },
        },
    ))
}

pub fn named_span_attr(source: &str) -> IResult<&str, SpanAttr, ErrorTree<&str>> {
    let (source, attr) = alt((named_span_key_value_attr, named_span_flag_attr))
        .context("")
        .parse(source)?;
    Ok((source, attr))
}

pub fn named_span_flag_attr(source: &str) -> IResult<&str, SpanAttr, ErrorTree<&str>> {
    let (source, the_tag) = tag("|").context("").parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    let (source, words) = many1(alt((
        wordpart,
        colon,
        newline,
        hyphen,
        single_lessthan,
        single_greaterthan,
        single_backtick,
        escaped_backtick,
        escaped_colon,
        escaped_pipe,
        escaped_greaterthan,
        escaped_backslash,
        non_escape_backslash,
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

pub fn named_span_key_value_attr(source: &str) -> IResult<&str, SpanAttr, ErrorTree<&str>> {
    let initial_source = source;
    let (source, _) = tag("|").context("").parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    // TODO: allow for spaces here
    // TODO: Move this to span_for_shorthand_attr_key
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
        non_escape_backslash,
    )))
    .context("")
    .parse(source)?;
    let (source, _) = tag(":").context("").parse(source)?;
    let (source, _) = space1.context("").parse(source)?;
    let (source, tokens) = many1(alt((
        wordpart,
        colon,
        hyphen,
        newline,
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
    let value = tokens
        .iter()
        .map(|word| word.parsed_text.clone())
        .collect::<Vec<String>>()
        .join("");
    let source_text = initial_source.replace(source, "").to_string();
    let key = key_parts
        .iter()
        .map(|p| p.parsed_text.clone())
        .collect::<Vec<String>>()
        .join(" ");
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
    #[case("<<alfa|bravo>>", 0, "single word", true)]
    #[case("<<alfa|bravo charlie>>", 0, "space in text", true)]
    #[case("<<alfa|bravo-charlie>>", 0, "hyphen in text", true)]
    #[case("<<alfa|bravo`charlie>>", 0, "single backtick in text", true)]
    #[case("<<alfa|bravo\ncharlie>>", 0, "newline in text", true)]
    #[case("<<alfa|bravo\\charlie>>", 0, "non-escaped backslash in text", true)]
    #[case("<<alfa|bravo\\`charlie>>", 0, "escaped backtick in text", true)]
    #[case("<<alfa|bravo\\:charlie>>", 0, "escaped colon in text", true)]
    #[case("<<alfa|bravo\\|charlie>>", 0, "escaped pipe in text", true)]
    #[case("<<alfa|bravo\\\\charlie>>", 0, "escaped baskslash in text", true)]
    #[case("<<alfa|bravo:charlie>>", 0, "colon in text surrounded", true)]
    #[case("<<alfa|bravo: charlie>>", 0, "colon in text before space", true)]
    #[case("<<alfa|bravo :charlie>>", 0, "colon in text after space", true)]
    #[case("<<alfa|bravo : charlie>>", 0, "colon in text floating", true)]
    #[case("<<alfa|bravo|charlie>>", 1, "single flag", true)]
    #[case("<<alfa|bravo|charlie delta>>", 1, "space in flag ", true)]
    #[case("<<alfa|bravo|charlie`delta>>", 1, "single backtick in flag", true)]
    #[case("<<alfa|bravo|charlie\ndelta>>", 1, "newline in flag", true)]
    #[case(
        "<<alfa|bravo|charlie\\delta>>",
        1,
        "non-escaped backslash in flag",
        true
    )]
    #[case("<<alfa|bravo|charlie\\`delta>>", 1, "escaped backtick in flag", true)]
    #[case("<<alfa|bravo|charlie\\:delta>>", 1, "escaped colon in flag", true)]
    #[case("<<alfa|bravo|charlie\\|delta>>", 1, "escaped pipe in flag", true)]
    #[case(
        "<<alfa|bravo|charlie\\\\delta>>",
        1,
        "escaped backslash in flag",
        true
    )]
    #[case("<<alfa-bravo|charlie>>", 0, "hyphen in name", true)]
    #[case("<<alfa|<<bravo|clarlie>>>>", 0, "nested span", true)]
    fn run_test(
        #[case] input: &str,
        #[case] attrs: usize,
        #[case] _description: &str,
        #[case] should_be_valid: bool,
    ) {
        if should_be_valid {
            let (remainder, span) = named_span(input).unwrap();
            assert_eq!(remainder, "");
            assert_eq!(span.attrs.len(), attrs);
        } else {
            assert!(named_span(input).is_err());
        }
    }

    #[test]
    fn basic_test() {
        let source = "<<em|alfa>>";
        let left = (
            "",
            Span {
                attrs: vec![],
                source_text: "<<em|alfa>>".to_string(),
                parsed_text: "alfa".to_string(),
                kind: SpanKind::NamedSpan {
                    r#type: "em".to_string(),
                    spans: vec![Span {
                        attrs: vec![],
                        source_text: "alfa".to_string(),
                        parsed_text: "alfa".to_string(),
                        kind: SpanKind::WordPart,
                    }],
                },
            },
        );
        let right = named_span(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn with_escaped_pipe() {
        let source = "<<alfa|ping\\|ping>>";
        let left = (
            "",
            Span {
                attrs: vec![],
                source_text: "<<alfa|ping\\|ping>>".to_string(),
                parsed_text: "ping|ping".to_string(),
                kind: SpanKind::NamedSpan {
                    r#type: "alfa".to_string(),
                    spans: vec![
                        Span {
                            attrs: vec![],
                            source_text: "ping".to_string(),
                            parsed_text: "ping".to_string(),
                            kind: SpanKind::WordPart,
                        },
                        Span {
                            attrs: vec![],
                            source_text: "\\|".to_string(),
                            parsed_text: "|".to_string(),
                            kind: SpanKind::EscapedPipe,
                        },
                        Span {
                            attrs: vec![],
                            source_text: "ping".to_string(),
                            parsed_text: "ping".to_string(),
                            kind: SpanKind::WordPart,
                        },
                    ],
                },
            },
        );
        let right = named_span(source).unwrap();
        assert_eq!(left, right);
    }

    // #[test]
    // fn with_escaped_backslash() {
    //     let source = "``ping\\\\ping``";
    //     let left = (
    //         "",
    //         Span {
    //             attrs: vec![],
    //             source_text: "``ping\\\\ping``".to_string(),
    //             parsed_text: "ping\\ping".to_string(),
    //             kind: SpanKind::CodeShorthand,
    //         },
    //     );
    //     let right = code_shorthand(source).unwrap();
    //     assert_eq!(left, right);
    // }

    // #[test]
    // fn with_escaped_backtick() {
    //     let source = "``ping\\`ping``";
    //     let left = (
    //         "",
    //         Span {
    //             attrs: vec![],
    //             source_text: "``ping\\`ping``".to_string(),
    //             parsed_text: "ping`ping".to_string(),
    //             kind: SpanKind::CodeShorthand,
    //         },
    //     );
    //     let right = code_shorthand(source).unwrap();
    //     assert_eq!(left, right);
    // }

    #[test]
    fn with_flag_attr() {
        let source = "<<code|something|rust>>";
        let left = (
            "",
            Span {
                attrs: vec![SpanAttr {
                    source_text: "|rust".to_string(),
                    kind: SpanAttrKind::Flag {
                        value: "rust".to_string(),
                    },
                }],
                source_text: "<<code|something|rust>>".to_string(),
                parsed_text: "something".to_string(),
                kind: SpanKind::NamedSpan {
                    r#type: "code".to_string(),
                    spans: vec![Span {
                        attrs: vec![],
                        source_text: "something".to_string(),
                        parsed_text: "something".to_string(),
                        kind: SpanKind::WordPart,
                    }],
                },
            },
        );
        let right = named_span(source).unwrap();
        assert_eq!(left, right);
    }

    // #[test]
    // fn with_multiple_flag_attrs() {
    //     let source = "``code|rust|hidden``";
    //     let left = (
    //         "",
    //         Span {
    //             attrs: vec![
    //                 SpanAttr {
    //                     source_text: "|rust".to_string(),
    //                     kind: SpanAttrKind::Flag {
    //                         value: "rust".to_string(),
    //                     },
    //                 },
    //                 SpanAttr {
    //                     source_text: "|hidden".to_string(),
    //                     kind: SpanAttrKind::Flag {
    //                         value: "hidden".to_string(),
    //                     },
    //                 },
    //             ],
    //             source_text: "``code|rust|hidden``".to_string(),
    //             parsed_text: "code".to_string(),
    //             kind: SpanKind::CodeShorthand,
    //         },
    //     );
    //     let right = code_shorthand(source).unwrap();
    //     assert_eq!(left, right);
    // }

    // #[test]
    // fn flag_attr_for_code_basic_end_at_block() {
    //     let source = "|rust``";
    //     let left = (
    //         "``",
    //         SpanAttr {
    //             source_text: "|rust".to_string(),
    //             kind: SpanAttrKind::Flag {
    //                 value: "rust".to_string(),
    //             },
    //         },
    //     );
    //     let right = flag_attr(source).unwrap();
    //     assert_eq!(left, right);
    // }

    // #[test]
    // fn flag_attr_for_code_url() {
    //     let source = "|https://www.example.com``";
    //     let left = (
    //         "``",
    //         SpanAttr {
    //             source_text: "|https://www.example.com".to_string(),
    //             kind: SpanAttrKind::Flag {
    //                 value: "https://www.example.com".to_string(),
    //             },
    //         },
    //     );
    //     let right = flag_attr(source).unwrap();
    //     assert_eq!(left, right);
    // }

    // #[test]
    // fn flag_attr_for_code_with_pipe_escape() {
    //     let source = "|rust\\|here``";
    //     let left = (
    //         "``",
    //         SpanAttr {
    //             source_text: "|rust\\|here".to_string(),
    //             kind: SpanAttrKind::Flag {
    //                 value: "rust|here".to_string(),
    //             },
    //         },
    //     );
    //     let right = flag_attr(source).unwrap();
    //     assert_eq!(left, right);
    // }

    // #[test]
    // fn integration_1_basic() {
    //     let source = "alfa ``bravo`` charlie";
    //     let left = (
    //         "",
    //         vec![
    //             Span {
    //                 attrs: vec![],
    //                 kind: SpanKind::WordPart,
    //                 parsed_text: "alfa".to_string(),
    //                 source_text: "alfa".to_string(),
    //             },
    //             Span {
    //                 attrs: vec![],
    //                 kind: SpanKind::Space,
    //                 parsed_text: " ".to_string(),
    //                 source_text: " ".to_string(),
    //             },
    //             Span {
    //                 attrs: vec![],
    //                 kind: SpanKind::CodeShorthand,
    //                 parsed_text: "bravo".to_string(),
    //                 source_text: "``bravo``".to_string(),
    //             },
    //             Span {
    //                 attrs: vec![],
    //                 kind: SpanKind::Space,
    //                 parsed_text: " ".to_string(),
    //                 source_text: " ".to_string(),
    //             },
    //             Span {
    //                 attrs: vec![],
    //                 kind: SpanKind::WordPart,
    //                 parsed_text: "charlie".to_string(),
    //                 source_text: "charlie".to_string(),
    //             },
    //         ],
    //     );
    //     let right = many1(span).parse(source).unwrap();
    //     assert_eq!(left, right);
    // }

    // #[test]
    // fn integration_2_escaped_pipe() {
    //     let source = r#"alfa ``bravo\|delta`` charlie"#;
    //     let left = (
    //         "",
    //         vec![
    //             Span {
    //                 attrs: vec![],
    //                 kind: SpanKind::WordPart,
    //                 parsed_text: "alfa".to_string(),
    //                 source_text: "alfa".to_string(),
    //             },
    //             Span {
    //                 attrs: vec![],
    //                 kind: SpanKind::Space,
    //                 parsed_text: " ".to_string(),
    //                 source_text: " ".to_string(),
    //             },
    //             Span {
    //                 attrs: vec![],
    //                 kind: SpanKind::CodeShorthand,
    //                 parsed_text: "bravo|delta".to_string(),
    //                 source_text: "``bravo\\|delta``".to_string(),
    //             },
    //             Span {
    //                 attrs: vec![],
    //                 kind: SpanKind::Space,
    //                 parsed_text: " ".to_string(),
    //                 source_text: " ".to_string(),
    //             },
    //             Span {
    //                 attrs: vec![],
    //                 kind: SpanKind::WordPart,
    //                 parsed_text: "charlie".to_string(),
    //                 source_text: "charlie".to_string(),
    //             },
    //         ],
    //     );
    //     let right = many1(span).parse(source).unwrap();
    //     assert_eq!(left, right);
    // }

    // #[test]
    // fn integration_2_escaped_single_backtick() {
    //     let source = r#"alfa ``bravo`delta`` charlie"#;
    //     let left = (
    //         "",
    //         vec![
    //             Span {
    //                 attrs: vec![],
    //                 kind: SpanKind::WordPart,
    //                 parsed_text: "alfa".to_string(),
    //                 source_text: "alfa".to_string(),
    //             },
    //             Span {
    //                 attrs: vec![],
    //                 kind: SpanKind::Space,
    //                 parsed_text: " ".to_string(),
    //                 source_text: " ".to_string(),
    //             },
    //             Span {
    //                 attrs: vec![],
    //                 kind: SpanKind::CodeShorthand,
    //                 parsed_text: "bravo`delta".to_string(),
    //                 source_text: "``bravo`delta``".to_string(),
    //             },
    //             Span {
    //                 attrs: vec![],
    //                 kind: SpanKind::Space,
    //                 parsed_text: " ".to_string(),
    //                 source_text: " ".to_string(),
    //             },
    //             Span {
    //                 attrs: vec![],
    //                 kind: SpanKind::WordPart,
    //                 parsed_text: "charlie".to_string(),
    //                 source_text: "charlie".to_string(),
    //             },
    //         ],
    //     );
    //     let right = many1(span).parse(source).unwrap();
    //     assert_eq!(left, right);
    // }

    // #[test]
    // fn key_value_attrs_basic_key_value_attr() {
    //     let source = "|class: green";
    //     let left = (
    //         "",
    //         SpanAttr {
    //             source_text: "|class: green".to_string(),
    //             kind: SpanAttrKind::KeyValue {
    //                 key: "class".to_string(),
    //                 value: "green".to_string(),
    //             },
    //         },
    //     );
    //     let right = key_value_attr(source).unwrap();
    //     assert_eq!(left, right);
    // }

    // #[test]
    // fn basic_with_key_value() {
    //     let source = "``ping|class: blue``";
    //     let left = (
    //         "",
    //         Span {
    //             attrs: vec![SpanAttr {
    //                 source_text: "|class: blue".to_string(),
    //                 kind: SpanAttrKind::KeyValue {
    //                     key: "class".to_string(),
    //                     value: "blue".to_string(),
    //                 },
    //             }],
    //             source_text: "``ping|class: blue``".to_string(),
    //             parsed_text: "ping".to_string(),
    //             kind: SpanKind::CodeShorthand {},
    //         },
    //     );
    //     let right = code_shorthand(source).unwrap();
    //     assert_eq!(left, right);
    // }

    //
}

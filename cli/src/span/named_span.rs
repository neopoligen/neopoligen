//use crate::span::escaped_colon::*;
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

use self::any_asterisk::any_asterisk;
use self::any_backtick::any_backtick;
use self::any_caret::any_caret;
use self::any_colon::any_colon;
use self::any_hyphen::any_hyphen;
use self::any_lessthan::any_lessthan;
use self::any_newline::any_newline;
use self::any_underscore::any_underscore;
use self::colon_followed_by_multispace1::colon_followed_by_multispace1;
use self::escaped_lessthan::escaped_lessthan;

// TODO: Deprecated remove when V42 is ready
pub fn named_span(source: &str) -> IResult<&str, Span, ErrorTree<&str>> {
    let (source, _) = tag("<<").context("").parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    let (source, type_parts) = many1(alt((alpha1, digit1, tag("-"), tag("_"))))
        .context("")
        .parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    let (source, _) = tag("|").context("").parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    let (source, children) = many0(alt((base_span_for_all_text,)))
        .context("")
        .parse(source)?;
    let (source, attrs) = many0(named_span_attr).context("").parse(source)?;
    let (source, _) = tag(">>").context("").parse(source)?;
    let r#type = type_parts
        .iter()
        .map(|p| p.to_string())
        .collect::<Vec<String>>()
        .join("");
    Ok((
        source,
        Span {
            attrs,
            parsed_text: "".to_string(),
            kind: SpanKind::NamedSpan {
                r#type: r#type.to_string(),
                children,
            },
        },
    ))
}

// TODO: Deprecated remove when V42 is ready
pub fn named_span_attr(source: &str) -> IResult<&str, SpanAttr, ErrorTree<&str>> {
    let (source, attr) = alt((named_span_key_value_attr, named_span_flag_attr))
        .context("")
        .parse(source)?;
    Ok((source, attr))
}

// TODO: Deprecated remove when V42 is ready
pub fn named_span_flag_attr(source: &str) -> IResult<&str, SpanAttr, ErrorTree<&str>> {
    let (source, _) = tag("|").context("").parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    let (source, words) = many1(alt((base_span_for_all_text,)))
        .context("")
        .parse(source)?;
    let value = words
        .iter()
        .map(|word| word.parsed_text.clone())
        .collect::<Vec<String>>()
        .join("");
    let attr = SpanAttr {
        kind: SpanAttrKind::Flag { value },
    };
    Ok((source, attr))
}

// TODO: Deprecated remove when V42 is ready
pub fn named_span_key_value_attr(source: &str) -> IResult<&str, SpanAttr, ErrorTree<&str>> {
    let (source, _) = tag("|").context("").parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    // TODO: allow for spaces here
    // TODO: Move this to span_for_shorthand_attr_key
    let (source, key_parts) = many1(alt((alpha1, digit1, tag("-"), tag("_"))))
        .context("")
        .parse(source)?;
    let (source, _) = tag(":").context("").parse(source)?;
    let (source, _) = space1.context("").parse(source)?;
    let (source, tokens) = many1(alt((
        colon,
        newline,
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
    let value = tokens
        .iter()
        .map(|word| word.parsed_text.clone())
        .collect::<Vec<String>>()
        .join("");
    let key = key_parts
        .iter()
        .map(|p| p.to_string())
        .collect::<Vec<String>>()
        .join("");
    let attr = SpanAttr {
        kind: SpanAttrKind::KeyValue {
            key: key.to_string(),
            value,
        },
    };
    Ok((source, attr))
}

// NEW V42 STUFF ////////////////////////////////////////////////////////////////

pub fn named_span_attr_key_v42(source: &str) -> IResult<&str, Vec<Span>, ErrorTree<&str>> {
    let (source, _) = multispace0.context("").parse(source)?;
    let (source, spans) = many1(alt((
        plaintext,
        any_hyphen,
        any_caret,
        any_asterisk,
        escaped_backtick,
        single_backtick,
        any_underscore,
        any_hyphen,
        escaped_pipe,
        escaped_colon,
        escaped_backslash,
        colon_not_followed_by_space,
        single_lessthan,
        escaped_lessthan,
        single_greaterthan,
        escaped_greaterthan,
    )))
    .context("")
    .parse(source)?;
    let (source, _) = colon_followed_by_multispace1.context("").parse(source)?;
    Ok((source, spans))
}

pub fn named_span_attr_v42(source: &str) -> IResult<&str, SpanAttr, ErrorTree<&str>> {
    let (source, _) = tag("|").context("").parse(source)?;
    let (source, key_spans) = named_span_attr_key_v42.context("").parse(source)?;
    Ok((
        source,
        SpanAttr {
            kind: SpanAttrKind::KeyValueSpansV42 {
                key: key_spans,
                value: vec![],
            },
        },
    ))
}

pub fn named_span_flag_v42(source: &str) -> IResult<&str, SpanAttr, ErrorTree<&str>> {
    let (source, _) = tag("|").context("").parse(source)?;
    Ok((
        source,
        SpanAttr {
            kind: SpanAttrKind::FlagV42 { value: vec![] },
        },
    ))
}

pub fn named_span_type_span_v42(source: &str) -> IResult<&str, Span, ErrorTree<&str>> {
    let (source, span) = alt((
        plaintext,
        any_newline,
        single_lessthan,
        escaped_lessthan,
        any_caret,
        single_greaterthan,
        escaped_greaterthan,
        any_asterisk,
        any_backtick,
        any_underscore,
        any_hyphen,
        escaped_pipe,
        any_colon,
        escaped_backslash,
    ))
    .context("")
    .parse(source)?;
    Ok((source, span))
}

pub fn named_span_content_value_span_v42(source: &str) -> IResult<&str, Span, ErrorTree<&str>> {
    let (source, span) = alt((
        plaintext,
        any_newline,
        single_lessthan,
        escaped_lessthan,
        any_caret,
        single_greaterthan,
        escaped_greaterthan,
        any_asterisk,
        any_backtick,
        any_underscore,
        any_hyphen,
        escaped_pipe,
        any_colon,
        escaped_backslash,
    ))
    .context("")
    .parse(source)?;
    Ok((source, span))
}

pub fn named_span_v42(source: &str) -> IResult<&str, Span, ErrorTree<&str>> {
    let (source, _) = tag("<<").context("").parse(source)?;
    let (source, type_spans) = many1(named_span_type_span_v42).context("").parse(source)?;
    let (source, _) = tag("|").context("").parse(source)?;
    let (source, value_spans) = many1(named_span_content_value_span_v42)
        .context("")
        .parse(source)?;
    let (source, attr_spans) = many0(alt((named_span_flag_v42, named_span_attr_v42)))
        .context("")
        .parse(source)?;
    let (source, _) = tag(">>").context("").parse(source)?;
    Ok((
        source,
        Span {
            attrs: attr_spans,
            kind: SpanKind::NamedSpanV42 {
                r#type: type_spans,
                value: value_spans,
            },
            parsed_text: "".to_string(),
        },
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    // Valid Named Spans
    #[rstest]
    #[case("<<alfa|bravo>>")]
    #[case("<<\nalfa\nbravo\n|\nalfa\nbravo\n>>")]
    #[case("<<alfa<bravo|alfa<bravo>>")]
    #[case("<<alfa\\<<bravo|alfa\\<<bravo>>")]
    #[case("<<alfa^bravo|alfa^bravo>>")]
    #[case("<<alfa>bravo|alfa>bravo>>")]
    #[case("<<alfa\\>>bravo|alfa\\>>bravo>>")]
    #[case("<<alfa*bravo|alfa*bravo>>")]
    #[case("<<alfa`bravo|alfa`bravo>>")]
    #[case("<<alfa_bravo|alfa_bravo>>")]
    #[case("<<alfa_bravo|alfa_bravo>>")]
    #[case("<<alfa-bravo|alfa-bravo>>")]
    #[case("<<alfa\\|bravo|alfa\\|bravo>>")]
    #[case("<<alfa:bravo|alfa:bravo>>")]
    #[case("<<alfa\\\\bravo|alfa\\\\bravo>>")]
    fn solo_valid_named_span_cases(#[case] source: &str) {
        let result = named_span_v42(source);
        if result.is_ok() {
            //dbg!(&result.unwrap().clone());
            assert!(true)
        } else {
            dbg!(&result);
            assert!(false);
        }
    }

    // Valid named span key value keys
    // \n<^*`_-|:\\
    #[rstest]
    #[case("spaced are allowed: ")]
    #[case("colon:without:spaces: ")]
    #[case("alfa: ")]
    #[case("bravo-charlie: ")]
    #[case("\ndelta: ")]
    #[case("\necho:\n")]
    #[case("fox<trot: ")]
    #[case("fox\\<<trot: ")]
    #[case("fox>trot: ")]
    #[case("fox\\>>trot: ")]
    #[case("go^lf: ")]
    #[case("go^^lf: ")]
    #[case("hot*el: ")]
    #[case("hot**el: ")]
    #[case("ind`ia: ")]
    #[case("ind\\``ia: ")]
    #[case("jul_iet: ")]
    #[case("jul__iet: ")]
    #[case("ki-lo: ")]
    #[case("ki--lo: ")]
    #[case("li\\|ma: ")]
    #[case("mi\\:ke: ")]
    #[case("mi\\::ke: ")]
    #[case("mi::ke: ")]
    #[case("nov\\\\ember: ")]
    fn named_span_attr_key_v42_tests(#[case] source: &str) {
        let result = named_span_attr_key_v42(source);
        if result.is_ok() {
            //dbg!(&result.unwrap().clone());
            assert!(true)
        } else {
            dbg!(&result);
            assert!(false);
        }
    }

    #[test]
    fn solo_ns_basic_v42() {
        let source = "<<em|alfa>>";
        let left = (
            "",
            Span {
                attrs: vec![],
                kind: SpanKind::NamedSpanV42 {
                    r#type: vec![Span {
                        attrs: vec![],
                        kind: SpanKind::PlainText,
                        parsed_text: "em".to_string(),
                    }],
                    value: vec![Span {
                        attrs: vec![],
                        kind: SpanKind::PlainText,
                        parsed_text: "alfa".to_string(),
                    }],
                },
                parsed_text: "".to_string(),
            },
        );
        let right = named_span_v42(source).unwrap();
        assert_eq!(left, right);
    }

    #[rstest]
    #[case("<<alfa|bravo>>", 0, "single word", true)]
    #[case("<<alfa|bravo charlie>>", 0, "space in text", true)]
    #[case("<<alfa|bravo-charlie>>", 0, "hyphen in text", true)]
    #[case("<<alfa|bravo`charlie>>", 0, "single backtick in text", true)]
    #[case("<<alfa|bravo\ncharlie>>", 0, "newline in text", true)]
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
    #[case(
        "<<link|Perl|https://en.wikipedia.org/wiki/Perl>>",
        1,
        "link span with link",
        true
    )]
    #[ignore]
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
    #[ignore]
    fn attr_with_hyphen() {
        let source = "<<em|alfa|data-ping: bravo>>";
        let left = (
            "",
            Span {
                attrs: vec![SpanAttr {
                    kind: SpanAttrKind::KeyValue {
                        key: "data-ping".to_string(),
                        value: "bravo".to_string(),
                    },
                }],
                parsed_text: "".to_string(),
                kind: SpanKind::NamedSpan {
                    r#type: "em".to_string(),
                    children: vec![Span {
                        attrs: vec![],
                        parsed_text: "alfa".to_string(),
                        kind: SpanKind::PlainText,
                    }],
                },
            },
        );
        let right = named_span(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    #[ignore]
    fn basic_test() {
        let source = "<<em|alfa>>";
        let left = (
            "",
            Span {
                attrs: vec![],
                parsed_text: "".to_string(),
                kind: SpanKind::NamedSpan {
                    r#type: "em".to_string(),
                    children: vec![Span {
                        attrs: vec![],
                        parsed_text: "alfa".to_string(),
                        kind: SpanKind::PlainText,
                    }],
                },
            },
        );
        let right = named_span(source).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    #[ignore]
    fn with_escaped_pipe() {
        let source = "<<alfa|ping\\|ping>>";
        let left = (
            "",
            Span {
                attrs: vec![],
                parsed_text: "".to_string(),
                kind: SpanKind::NamedSpan {
                    r#type: "alfa".to_string(),
                    children: vec![
                        Span {
                            attrs: vec![],
                            parsed_text: "ping".to_string(),
                            kind: SpanKind::PlainText,
                        },
                        Span {
                            attrs: vec![],
                            parsed_text: "|".to_string(),
                            kind: SpanKind::EscapedPipe,
                        },
                        Span {
                            attrs: vec![],
                            parsed_text: "ping".to_string(),
                            kind: SpanKind::PlainText,
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
    #[ignore]
    fn with_flag_attr() {
        let source = "<<code|something|rust>>";
        let left = (
            "",
            Span {
                attrs: vec![SpanAttr {
                    kind: SpanAttrKind::Flag {
                        value: "rust".to_string(),
                    },
                }],
                parsed_text: "".to_string(),
                kind: SpanKind::NamedSpan {
                    r#type: "code".to_string(),
                    children: vec![Span {
                        attrs: vec![],
                        parsed_text: "something".to_string(),
                        kind: SpanKind::PlainText,
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
    //                 kind: SpanKind::PlainText,
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
    //                 kind: SpanKind::PlainText,
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
    //                 kind: SpanKind::PlainText,
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
    //                 kind: SpanKind::PlainText,
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
    //                 kind: SpanKind::PlainText,
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
    //                 kind: SpanKind::PlainText,
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

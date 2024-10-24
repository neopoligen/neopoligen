#![allow(unused_variables)]
#![allow(unused_imports)]
use crate::span::any_lessthan::any_lessthan;
use crate::span::greaterthan::*;
use crate::span::lessthan::*;
use crate::span::*;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::alpha1;
use nom::character::complete::digit1;
use nom::character::complete::multispace1;
use nom::multi::many0;
use nom::multi::many1;
use nom::sequence::pair;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;

use self::any_asterisk::any_asterisk;
use self::any_caret::any_caret;
use self::any_hyphen::any_hyphen;
use self::any_newline::any_newline;
use self::any_underscore::any_underscore;
use self::escaped_lessthan::escaped_lessthan;

pub fn code_shorthand_flag_v42(source: &str) -> IResult<&str, SpanAttr, ErrorTree<&str>> {
    let (source, _) = tag("|").context("").parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    let (source, text) = is_not("`| \n\t").context("").parse(source)?;
    let (source, _) = multispace0.context("").parse(source)?;
    let flag = SpanAttr {
        kind: SpanAttrKind::Flag {
            value: text.to_string(),
        },
    };
    Ok((source, flag))
}

pub fn code_shorthand_base_span(source: &str) -> IResult<&str, Span, ErrorTree<&str>> {
    let (source, span) = alt((
        plaintext.context(""),
        single_backtick,
        any_asterisk,
        any_caret,
        any_hyphen,
        any_lessthan,
        any_newline,
        any_underscore,
        escaped_backtick,
        escaped_backslash,
        escaped_pipe,
    ))
    .parse(source)?;
    Ok((source, span))
}

pub fn code_shorthand(source: &str) -> IResult<&str, Span, ErrorTree<&str>> {
    let (source, _) = tag("``").context("").context("").parse(source)?;
    let (source, spans) = many1(code_shorthand_base_span).context("").parse(source)?;
    let (source, attrs) = many0(code_shorthand_flag_v42).context("").parse(source)?;
    // let (source, parts) = many1(alt((base_span_for_all_text, lessthan, greaterthan)))
    //     .context("")
    //     .parse(source)?;
    //    let (source, attrs) = many0(code_shorthand_attr).context("").parse(source)?;
    let (source, _) = tag("``").context("").parse(source)?;
    // let parsed_text = parts
    //     .iter()
    //     .map(|word| word.parsed_text.clone())
    //     .collect::<Vec<String>>()
    //     .join("");
    Ok((
        source,
        Span {
            attrs,
            kind: SpanKind::CodeShorthand { children: spans },
            parsed_text: "".to_string(),
        },
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    // Valid Code Shorthands
    #[rstest]
    #[case("``alfa``")]
    #[case("``bravo charlie``")]
    #[case("``delta echo|foxtrot``")]
    #[case("``golf| hotel | india``")]
    #[case("``contains \n neline``")]
    #[case("``contains \n\n nelines``")]
    #[case("``single < lessthan``")]
    #[case("``multiple << lessthans``")]
    #[case("``html <div> tag``")]
    #[case("``single ^ caret``")]
    #[case("``multiple ^^ carets``")]
    #[case("``single * asterisk``")]
    #[case("``multiple ** asterisks``")]
    #[case("``single _ underscore``")]
    #[case("``multiple __ underscores``")]
    #[case("``single - hyphen``")]
    #[case("``multiple -- hyphens``")]
    #[case("``single ` backtick``")]
    #[case("``single \\` backtick esaped``")]
    #[case("``escaped \\`` backtick before single backtick``")]
    #[case("``escaped \\| pipe``")]
    #[case("``escaped \\\\ backslash``")]
    fn solo_valid_code_shorthands(#[case] source: &str) {
        let result = code_shorthand(source);
        if result.is_ok() {
            assert!(true)
        } else {
            dbg!(&result);
            assert!(false);
        }
    }

    // Invalid Code Shorthands
    #[rstest]
    #[case("``alfa|no spaces``")]
    #[case("``no trailing pipe|``")]
    fn solo_invalid_code_shorthands(#[case] source: &str) {
        let result = code_shorthand(source);
        if result.is_ok() {
            dbg!(&result);
            assert!(false);
        } else {
            assert!(true)
        }
    }

    // Valid Code Shorthand Flags
    #[rstest]
    #[case("|alfa")]
    #[case("| bravo ")]
    #[case("| charlie-delta ")]
    #[case("|https://www.example.com/")]
    fn solo_valid_code_shorthand_flags(#[case] source: &str) {
        let result = code_shorthand_flag_v42(source);
        if result.as_ref().is_ok() {
            if result.as_ref().unwrap().0 == "" {
                assert!(true)
            } else {
                dbg!(format!("missed: {}", result.unwrap().0));
                assert!(false);
            }
        } else {
            dbg!(&result);
            assert!(false);
        }
    }

    // #[test]
    // fn solo_code_shorthand_with_esaped_pipe() {
    //     let source = "``esacped\\|pipe``";
    //     let left = (
    //         "",
    //         Span {
    //             attrs: vec![],
    //             parsed_text: "escaped|pipe".to_string(),
    //             kind: SpanKind::CodeShorthand {},
    //         },
    //     );
    //     let result = code_shorthand(source);
    //     dbg!(&result);
    //     let right = result.unwrap();
    //     assert_eq!(left, right);
    // }

    #[test]
    fn solo_code_shorthand_with_flag_attr() {
        let source = "``code|rust``";
        let left = (
            "",
            Span {
                attrs: vec![SpanAttr {
                    kind: SpanAttrKind::Flag {
                        value: "rust".to_string(),
                    },
                }],
                parsed_text: "".to_string(),
                kind: SpanKind::CodeShorthand {
                    children: vec![Span {
                        attrs: vec![],
                        kind: SpanKind::PlainText,
                        parsed_text: "code".to_string(),
                    }],
                },
            },
        );
        let result = code_shorthand(source);
        // dbg!(&result);
        let right = result.unwrap();
        assert_eq!(left, right);
    }

    // #[test]
    // fn code_shorthand_with_multiple_flags() {
    //     let source = "``code|rust|hidden``";
    //     let left = (
    //         "",
    //         Span {
    //             attrs: vec![
    //                 SpanAttr {
    //                     kind: SpanAttrKind::Flag {
    //                         value: "rust".to_string(),
    //                     },
    //                 },
    //                 SpanAttr {
    //                     kind: SpanAttrKind::Flag {
    //                         value: "hidden".to_string(),
    //                     },
    //                 },
    //             ],
    //             parsed_text: "code".to_string(),
    //             kind: SpanKind::CodeShorthand {},
    //         },
    //     );
    //     let right = code_shorthand(source).unwrap();
    //     assert_eq!(left, right);
    // }

    #[rstest]
    #[case("``alfa``", 0, "single word")]
    #[case("``alfa bravo``", 0, "space in text")]
    #[case("``alfa-bravo``", 0, "hyphen in text")]
    #[case("``alfa`bravo``", 0, "single backtick in text")]
    #[case("``alfa_bravo``", 0, "with single underscore")]
    #[case("``alfa\\`bravo``", 0, "escaped backtick in text")]
    #[case("``alfa\\|bravo``", 0, "escaped pipe in text")]
    #[case("``alfa\\\\bravo``", 0, "escaped backslash in text")]
    #[case("``alfa:bravo``", 0, "colon in text")]
    #[case("``alfa: bravo``", 0, "colon in text before space")]
    #[case("``alfa :bravo``", 0, "colon in text after space")]
    #[case("``alfa\\|bravo``", 0, "escaped pipe in text")]
    #[case("``alfa\\`bravo``", 0, "escaped backtick in text")]
    #[case("``alfa|bravo``", 1, "single flag attr")]
    #[case("``alfa|bravo charlie``", 1, "space in flag")]
    #[case("``alfa|bravo`charlie``", 1, "single backtick in flag")]
    #[case("``alfa|bravo\ncharlie``", 1, "newline in flag")]
    #[case("``alfa|bravo\\|charlie``", 1, "escaped pipe in flag")]
    #[case("``alfa|bravo\\`charlie``", 1, "escaped backtick in flag")]
    #[case("``alfa|bravo\\\\charlie``", 1, "escaped baskslash in flag")]
    #[case("``alfa|bravo|charlie``", 2, "two flag attrs")]
    #[case("``alfa|bravo: charlie``", 1, "single key value attr")]
    #[case("``alfa|bravo: charlie|delta: echo``", 2, "single key value attr")]
    #[case("``\nalfa\n|\nbravo\n``", 1, "newlines in shorthand")]
    #[case(
        "``\nalfa\n|\nbravo\n|\ncharlie\n``",
        2,
        "newlines in shorthand multiple attrs"
    )]
    #[case("``Result<(), Box<dyn std::error::Error>>``", 0, "rust example")]
    fn code_shorhand_fixture(
        #[case] input: &str,
        #[case] attrs: usize,
        #[case] _description: &str,
    ) {
        let (remainder, span) = code_shorthand(input).unwrap();
        assert_eq!(remainder, "");
        assert_eq!(span.attrs.len(), attrs);
    }

    #[rstest]
    #[case("``\\``", "can't have a single backslash at the end of code text")]
    fn error_cases(#[case] input: &str, #[case] _desc: &str) {
        assert!(code_shorthand(input).is_err());
    }

    // #[test]
    // fn code_shorthand_basic() {
    //     let source = "``ping``";
    //     let left = (
    //         "",
    //         Span {
    //             attrs: vec![],
    //             parsed_text: "ping".to_string(),
    //             kind: SpanKind::CodeShorthand,
    //         },
    //     );
    //     let right = code_shorthand(source).unwrap();
    //     assert_eq!(left, right);
    // }

    // #[test]
    // fn code_shorthand_with_escaped_pipe() {
    //     let source = "``ping\\|ping``";
    //     let left = (
    //         "",
    //         Span {
    //             attrs: vec![],
    //             parsed_text: "ping|ping".to_string(),
    //             kind: SpanKind::CodeShorthand,
    //         },
    //     );
    //     let right = code_shorthand(source).unwrap();
    //     assert_eq!(left, right);
    // }

    // #[test]
    // fn code_shorthand_with_escaped_backslash() {
    //     let source = "``ping\\\\ping``";
    //     let left = (
    //         "",
    //         Span {
    //             attrs: vec![],
    //             parsed_text: "ping\\ping".to_string(),
    //             kind: SpanKind::CodeShorthand,
    //         },
    //     );
    //     let right = code_shorthand(source).unwrap();
    //     assert_eq!(left, right);
    // }

    // #[test]
    // fn code_shorthand_with_escaped_backtick() {
    //     let source = "``ping\\`ping``";
    //     let left = (
    //         "",
    //         Span {
    //             attrs: vec![],
    //             parsed_text: "ping`ping".to_string(),
    //             kind: SpanKind::CodeShorthand,
    //         },
    //     );
    //     let right = code_shorthand(source).unwrap();
    //     assert_eq!(left, right);
    // }

    // #[test]
    // fn code_shorthand_with_multiple_flag_attrs() {
    //     let source = "``code|rust|hidden``";
    //     let left = (
    //         "",
    //         Span {
    //             attrs: vec![
    //                 SpanAttr {
    //                     kind: SpanAttrKind::Flag {
    //                         value: "rust".to_string(),
    //                     },
    //                 },
    //                 SpanAttr {
    //                     kind: SpanAttrKind::Flag {
    //                         value: "hidden".to_string(),
    //                     },
    //                 },
    //             ],
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
    //             kind: SpanAttrKind::Flag {
    //                 value: "rust".to_string(),
    //             },
    //         },
    //     );
    //     let right = code_shorthand_flag_attr(source).unwrap();
    //     assert_eq!(left, right);
    // }

    // #[test]
    // fn flag_attr_for_code_url() {
    //     let source = "|https://www.example.com``";
    //     let left = (
    //         "``",
    //         SpanAttr {
    //             kind: SpanAttrKind::Flag {
    //                 value: "https://www.example.com".to_string(),
    //             },
    //         },
    //     );
    //     let right = code_shorthand_flag_attr(source).unwrap();
    //     assert_eq!(left, right);
    // }

    // #[test]
    // fn flag_attr_for_code_with_pipe_escape() {
    //     let source = "|rust\\|here``";
    //     let left = (
    //         "``",
    //         SpanAttr {
    //             kind: SpanAttrKind::Flag {
    //                 value: "rust|here".to_string(),
    //             },
    //         },
    //     );
    //     let right = code_shorthand_flag_attr(source).unwrap();
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
    //                 parsed_text: "alfa ".to_string(),
    //             },
    //             Span {
    //                 attrs: vec![],
    //                 kind: SpanKind::CodeShorthand,
    //                 parsed_text: "bravo".to_string(),
    //             },
    //             Span {
    //                 attrs: vec![],
    //                 kind: SpanKind::PlainText,
    //                 parsed_text: " charlie".to_string(),
    //             },
    //         ],
    //     );
    //     let right = many1(span_for_body_text).parse(source).unwrap();
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
    //                 parsed_text: "alfa ".to_string(),
    //             },
    //             Span {
    //                 attrs: vec![],
    //                 kind: SpanKind::CodeShorthand,
    //                 parsed_text: "bravo|delta".to_string(),
    //             },
    //             Span {
    //                 attrs: vec![],
    //                 kind: SpanKind::PlainText,
    //                 parsed_text: " charlie".to_string(),
    //             },
    //         ],
    //     );
    //     let right = many1(span_for_body_text).parse(source).unwrap();
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
    //                 parsed_text: "alfa ".to_string(),
    //             },
    //             Span {
    //                 attrs: vec![],
    //                 kind: SpanKind::CodeShorthand,
    //                 parsed_text: "bravo`delta".to_string(),
    //             },
    //             Span {
    //                 attrs: vec![],
    //                 kind: SpanKind::PlainText,
    //                 parsed_text: " charlie".to_string(),
    //             },
    //         ],
    //     );
    //     let right = many1(span_for_body_text).parse(source).unwrap();
    //     assert_eq!(left, right);
    // }

    // #[test]
    // fn code_shorthand_key_value_attrs_basic_key_value_attr() {
    //     let source = "|class: green";
    //     let left = (
    //         "",
    //         SpanAttr {
    //             kind: SpanAttrKind::KeyValue {
    //                 key: "class".to_string(),
    //                 value: "green".to_string(),
    //             },
    //         },
    //     );
    //     let right = code_shorthand_key_value_attr(source).unwrap();
    //     assert_eq!(left, right);
    // }

    // #[test]
    // fn code_shorthand_basic_with_key_value() {
    //     let source = "``ping|class: blue``";
    //     let left = (
    //         "",
    //         Span {
    //             attrs: vec![SpanAttr {
    //                 kind: SpanAttrKind::KeyValue {
    //                     key: "class".to_string(),
    //                     value: "blue".to_string(),
    //                 },
    //             }],
    //             parsed_text: "ping".to_string(),
    //             kind: SpanKind::CodeShorthand {},
    //         },
    //     );
    //     let right = code_shorthand(source).unwrap();
    //     assert_eq!(left, right);
    // }

    //
}

// // TODO: Deprecated: Remove when V42 is done
// pub fn code_shorthand_key_value_attr(source: &str) -> IResult<&str, SpanAttr, ErrorTree<&str>> {
//     let (source, _) = tag("|").context("").parse(source)?;
//     let (source, key_parts) = many1(alt((alpha1, digit1, tag("-"), tag("_"))))
//         .context("")
//         .parse(source)?;
//     let (source, _) = tag(":").context("").parse(source)?;
//     let (source, _) = space1.context("").parse(source)?;
//     let (source, tokens) = many1(alt((
//         space,
//         newline,
//         colon,
//         hyphen_with_trailing_non_hyphen,
//         single_lessthan,
//         single_greaterthan,
//         single_backtick,
//         escaped_backtick,
//         escaped_pipe,
//         escaped_greaterthan,
//         escaped_backslash,
//     )))
//     .context("")
//     .parse(source)?;
//     let key = key_parts
//         .iter()
//         .map(|p| p.to_string())
//         .collect::<Vec<String>>()
//         .join("");
//     let value = tokens
//         .iter()
//         .map(|word| word.parsed_text.clone())
//         .collect::<Vec<String>>()
//         .join("");
//     let attr = SpanAttr {
//         kind: SpanAttrKind::KeyValue {
//             key: key.to_string(),
//             value,
//         },
//     };
//     Ok((source, attr))
// }

// // TODO: Deprecated: remove when v42 is done
// pub fn code_shorthand_attr(source: &str) -> IResult<&str, SpanAttr, ErrorTree<&str>> {
//     let (source, attr) = alt((code_shorthand_key_value_attr, code_shorthand_flag_attr))
//         .context("")
//         .parse(source)?;
//     Ok((source, attr))
// }

use crate::span::escaped_pipe::*;
use crate::span::*;
// use crate::span_attr::SpanAttrKind;
// use crate::span_shorthand_token::*;
use nom::branch::alt;
// use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::multi::many0;
// use nom::multi::many1;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;
// use nom::character::complete::line_ending;
// use nom::character::complete::multispace0;
// use nom::character::complete::space0;
// use nom::character::complete::space1;
// use nom::combinator::eof;
// use nom::combinator::not;
// use nom::sequence::tuple;
// use serde::Serialize;

pub fn code_shorthand(source: &str) -> IResult<&str, Span, ErrorTree<&str>> {
    let initial_source = source;
    let (source, _) = tag("``").context("").parse(source)?;
    let (source, parts) = many0(code_shorthand_span).context("").parse(source)?;
    //let (source, tokens) = many1(code_shorthand_token).context("").parse(source)?;
    //let (source, attrs) = many0(code_shorthand_attr).context("").parse(source)?;
    let (source, _) = tag("``").context("").parse(source)?;
    let source_text = initial_source.replace(source, "").to_string();
    let parsed_text = parts
        .iter()
        .map(|word| word.parsed_text.clone())
        .collect::<Vec<String>>()
        .join("");
    let attrs = vec![];
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

pub fn code_shorthand_span(source: &str) -> IResult<&str, Span, ErrorTree<&str>> {
    let (source, span) = alt((wordpart, escaped_pipe)).context("").parse(source)?;
    Ok((source, span))
}

// pub fn code_shorthand_attr(source: &str) -> IResult<&str, SpanAttr, ErrorTree<&str>> {
//     let (source, attr) = alt((code_shorthand_key_value_attr, code_shorthand_flag_attr))
//         .context("")
//         .parse(source)?;
//     Ok((source, attr))
// }

// pub fn code_shorthand_flag_attr(source: &str) -> IResult<&str, SpanAttr, ErrorTree<&str>> {
//     let (source, the_tag) = tag("|").context("").parse(source)?;
//     let (source, words) = many1(code_shorthand_token).context("").parse(source)?;
//     let source_text = words
//         .iter()
//         .map(|word| word.source_text.clone())
//         .collect::<Vec<String>>()
//         .join("");
//     let value = words
//         .iter()
//         .map(|word| word.parsed_text.clone())
//         .collect::<Vec<String>>()
//         .join("");
//     let attr = SpanAttr {
//         source_text: format!("{}{}", the_tag, source_text),
//         kind: SpanAttrKind::Flag { value },
//     };
//     Ok((source, attr))
// }

// pub fn code_shorthand_token(source: &str) -> IResult<&str, SpanShorthandToken, ErrorTree<&str>> {
//     let (source, token) = alt((
//         // shorthand_token_escaped_pipe,
//         // shorthand_token_escaped_backslash,
//         // shorthand_token_escaped_backtick,
//         // shorthand_token_escaped_colon,
//         code_shorthand_token_word_part,
//         // shorthand_token_single_backslash,
//         // shorthand_token_single_backtick,
//     ))
//     .context("")
//     .parse(source)?;
//     Ok((source, token))
// }

// pub fn code_shorthand_token_word_part(
//     source: &str,
// ) -> IResult<&str, SpanShorthandToken, ErrorTree<&str>> {
//     let (source, text) = is_not("\\`|").context("").parse(source)?;
//     let token = SpanShorthandToken {
//         source_text: text.to_string(),
//         parsed_text: text.to_string(),
//         kind: SpanShorthandTokenKind::WordPart,
//     };
//     Ok((source, token))
// }

// pub fn code_shorthand_key_value_attr(source: &str) -> IResult<&str, SpanAttr, ErrorTree<&str>> {
//     let initial_source = source;
//     let (source, _) = tag("|").context("").parse(source)?;
//     // TODO: allow for spaces here
//     let (source, key) = is_not(" :|`").context("").parse(source)?;
//     let (source, _) = tag(":").context("").parse(source)?;
//     let (source, _) = space1.context("").parse(source)?;
//     let (source, tokens) = many1(code_shorthand_token).context("").parse(source)?;
//     let value = tokens
//         .iter()
//         .map(|word| word.parsed_text.clone())
//         .collect::<Vec<String>>()
//         .join("");
//     let source_text = initial_source.replace(source, "").to_string();
//     let attr = SpanAttr {
//         source_text,
//         kind: SpanAttrKind::KeyValue {
//             key: key.to_string(),
//             value,
//         },
//     };
//     Ok((source, attr))
// }

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

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

    // #[test]
    // fn code_shorthand_with_escaped_backslash() {
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
    // fn code_shorthand_with_escaped_backtick() {
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

    // #[test]
    // fn code_shorthand_with_single_backslash() {
    //     let source = "``ping\\ping``";
    //     let left = (
    //         "",
    //         Span {
    //             attrs: vec![],
    //             source_text: "``ping\\ping``".to_string(),
    //             parsed_text: "ping\\ping".to_string(),
    //             kind: SpanKind::CodeShorthand,
    //         },
    //     );
    //     let right = code_shorthand(source).unwrap();
    //     assert_eq!(left, right);
    // }

    // #[test]
    // fn code_shorthand_with_flag_attr() {
    //     let source = "``code|rust``";
    //     let left = (
    //         "",
    //         Span {
    //             attrs: vec![SpanAttr {
    //                 source_text: "|rust".to_string(),
    //                 kind: SpanAttrKind::Flag {
    //                     value: "rust".to_string(),
    //                 },
    //             }],
    //             source_text: "``code|rust``".to_string(),
    //             parsed_text: "code".to_string(),
    //             kind: SpanKind::CodeShorthand {},
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
    //     let right = code_shorthand_flag_attr(source).unwrap();
    //     assert_eq!(left, right);
    // }

    // #[test]
    // fn flag_attr_for_code_with_escaped_colon() {
    //     let source = "|rust\\:``";
    //     let left = (
    //         "``",
    //         SpanAttr {
    //             source_text: "|rust\\:".to_string(),
    //             kind: SpanAttrKind::Flag {
    //                 value: "rust:".to_string(),
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
    //             source_text: "|https://www.example.com".to_string(),
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
    //             source_text: "|rust\\|here".to_string(),
    //             kind: SpanAttrKind::Flag {
    //                 value: "rust|here".to_string(),
    //             },
    //         },
    //     );
    //     let right = code_shorthand_flag_attr(source).unwrap();
    //     assert_eq!(left, right);
    // }

    // #[test]
    // fn token_for_code_shorthand_word_part() {
    //     let source = "alfa``";
    //     let left = (
    //         "``",
    //         SpanShorthandToken {
    //             kind: SpanShorthandTokenKind::WordPart,
    //             parsed_text: "alfa".to_string(),
    //             source_text: "alfa".to_string(),
    //         },
    //     );
    //     let right = code_shorthand_token_word_part(source).unwrap();
    //     assert_eq!(left, right);
    // }

    // #[test]
    // fn integration_1_basic() {
    //     let config = SiteConfig::mock1();
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
    //     let right = many1(|src| span(src, &config.spans)).parse(source).unwrap();
    //     assert_eq!(left, right);
    // }

    // #[test]
    // fn integration_2_escaped_pipe() {
    //     let config = SiteConfig::mock1();
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
    //     let right = many1(|src| span(src, &config.spans)).parse(source).unwrap();
    //     assert_eq!(left, right);
    // }

    // #[test]
    // fn integration_2_escaped_single_backtick() {
    //     let config = SiteConfig::mock1();
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
    //     let right = many1(|src| span(src, &config.spans)).parse(source).unwrap();
    //     assert_eq!(left, right);
    // }

    // #[test]
    // fn code_shorthand_key_value_attrs_basic_key_value_attr() {
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
}

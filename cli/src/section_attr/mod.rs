#![allow(unused_imports)]

pub mod flag;

use crate::section_attr::flag::SectionFlagAttrV42;
// use crate::section::block::block_of_main_content_v42;
use crate::section::block::block_of_section_attr_value_v42;
// use crate::span::colon::colon;
// use crate::span::double_underscore::double_underscore;
// use crate::span::escaped_pipe::escaped_pipe;
// use crate::span::hyphen::hyphen;
// use crate::span::pipe::pipe;
// use crate::span::single_underscore::single_underscore;
use crate::helpers::dump_parser_if_debug;
use crate::span::*;
use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::character::complete::anychar;
use nom::character::complete::multispace0;
use nom::character::complete::multispace1;
use nom::character::complete::space1;
use nom::combinator::eof;
use nom::combinator::not;
use nom::multi::many1;
use nom::sequence::pair;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;
use serde::{Deserialize, Serialize};

use self::colon_not_followed_by_space::colon_not_followed_by_space;
use self::escaped_backslash::escaped_backslash;
use self::escaped_backtick::escaped_backtick;
use self::escaped_pipe::escaped_pipe;
use self::single_backtick::single_backtick;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SectionKeyValueAttrV42 {
    value: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
enum SectionAttrV42 {
    FlagV42(SectionFlagAttrV42),
    KeyValueV42(SectionKeyValueAttrV42),
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SectionAttr {
    // TODO: Rename this payload
    // TODO: Or, see if you can hoist it up a level?
    pub kind: SectionAttrKind,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum SectionAttrKind {
    // DEPRECATED: TODO: Remove KeyValue in favor of KeyValueSpans
    KeyValue { key: String, value: String },
    KeyValueSpans { key: String, spans: Vec<Span> },
    Flag { flag: String }, // TODO: Deprecate in favor of V42
                           // FlagV42 { value: Vec<Span> },
}

// pub fn section_attr_old<'a>(source: &'a str) -> IResult<&'a str, SectionAttr, ErrorTree<&'a str>> {
//     let (source, attr) = alt((section_key_value_attr_39, section_flag_attr))(source)?;
//     Ok((source, attr))
// }

pub fn section_attr<'a>(source: &'a str) -> IResult<&'a str, SectionAttr, ErrorTree<&'a str>> {
    let (source, attr) = alt((section_key_value_attr, section_flag))(source)?;
    Ok((source, attr))
}

pub fn section_key_value_attr<'a>(
    source: &'a str,
) -> IResult<&'a str, SectionAttr, ErrorTree<&'a str>> {
    // TODO: get debug value from the ast tree
    let debug = false;
    let dbg_header = "SECTION_KEY_VALUE_ATTR";
    dump_parser_if_debug(debug, dbg_header, source, 1);
    let (source, _) = tag("--").context("").parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 2);
    let (source, _) = space1.context("").parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 3);
    let (source, key_parts) = many1(alt((
        is_not(": \n\t"),
        pair(tag(":"), not(alt((space1, eof)))).map(|x| x.0),
    )))
    .context("")
    .parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 4);
    let (source, _) = tag(":").context("").parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 5);
    let (source, _) = space1.context("").parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 6);
    let (source, spans) = block_of_section_attr_value_v42.context("").parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 7);
    let key = key_parts
        .iter()
        .map(|part| part.to_string())
        .collect::<Vec<String>>()
        .join("");
    Ok((
        source,
        SectionAttr {
            kind: SectionAttrKind::KeyValueSpans {
                key: key.trim().to_string(),
                spans,
            },
        },
    ))
}

pub fn section_flag<'a>(source: &'a str) -> IResult<&'a str, SectionAttr, ErrorTree<&'a str>> {
    let (source, _) = tag("--").context("").parse(source)?;
    let (source, _) = space1.context("").parse(source)?;
    let (source, parts) = many1(alt((
        is_not(": \n\t"),
        pair(tag(":"), not(alt((multispace1, eof)))).map(|x| x.0),
    )))
    .context("")
    .parse(source)?;
    let (source, _) = structure_empty_until_newline_or_eof
        .context("")
        .parse(source)?;
    let flag = parts
        .iter()
        .map(|part| part.to_string())
        .collect::<Vec<String>>()
        .join("");
    Ok((
        source,
        SectionAttr {
            kind: SectionAttrKind::Flag { flag },
        },
    ))
}

// // DEPRECATED: Replace with keyvaluespans version
// pub fn section_key_value_attr_39<'a>(
//     source: &'a str,
// ) -> IResult<&'a str, SectionAttr, ErrorTree<&'a str>> {
//     let (source, _) = tag("--").context("").parse(source)?;
//     let (source, _) = space1.context("").parse(source)?;
//     let (source, key) = is_not(": \n").context("").parse(source)?;
//     let (source, _) = tag(":").context("").parse(source)?;
//     let (source, value) = not_line_ending.context("").parse(source)?;
//     let (source, _) = structure_empty_until_newline_or_eof
//         .context("")
//         .parse(source)?;
//     Ok((
//         source,
//         SectionAttr {
//             kind: SectionAttrKind::KeyValue {
//                 key: key.trim().to_string(),
//                 value: value.trim().to_string(),
//             },
//         },
//     ))
// }

// pub fn section_flag_base_span_v42(source: &str) -> IResult<&str, Span, ErrorTree<&str>> {
//     let (source, result) = is_not(":|\n\\").context("").parse(source)?;
//     Ok((
//         source,
//         Span {
//             attrs: vec![],
//             kind: SpanKind::PlainText,
//             parsed_text: result.to_string(),
//         },
//     ))
// }

// TODO: Remove this becuase moving back to having flag just be a string
// pub fn section_flag_v42(source: &str) -> IResult<&str, SectionAttr, ErrorTree<&str>> {
//     let (source, _) = tag("-- ").context("").parse(source)?;
//     let (source, _) = multispace0.context("").parse(source)?;
//     let (source, span_hits) = many1(pair(
//         alt((
//             section_flag_base_span_v42,
//             colon_not_followed_by_space,
//             escaped_pipe,
//             escaped_backslash,
//         )),
//         not(tag(": ")),
//     ))
//     .context("")
//     .parse(source)?;
//     // let (source, span_hits) = many1(pair(
//     //     alt((
//     //         code_shorthand_flag_base_span_v42,
//     //         colon_not_followed_by_space,
//     //         escaped_pipe,
//     //         escaped_backslash,
//     //         escaped_backtick,
//     //         single_backtick,
//     //     )),
//     //     not(tag(": ")),
//     // ))
//     // .context("")
//     // .parse(source)?;
//     // let (source, _) = multispace0.context("").parse(source)?;
//     // let spans = span_hits.iter().map(|sh| sh.0.clone()).collect();
//     // let flag = SpanAttr {
//     //     kind: SpanAttrKind::FlagV42 { value: spans },
//     // };
//     // let (source, _) = multispace0.context("").parse(source)?;
//     let (source, _) = structure_empty_until_newline_or_eof
//         .context("")
//         .parse(source)?;
//     let spans = span_hits.iter().map(|sh| sh.0.clone()).collect();
//     Ok((
//         source,
//         SectionAttr {
//             kind: SectionAttrKind::FlagV42 { value: spans },
//         },
//     ))
// }

// pub fn section_flag_attr<'a>(source: &'a str) -> IResult<&'a str, SectionAttr, ErrorTree<&'a str>> {
//     let (source, _) = tag("--").context("").parse(source)?;
//     let (source, _) = space1.context("").parse(source)?;
//     let (souce, spans) =
//     // let (source, parts) = many1(alt((
//     //     is_not(": \n\t"),
//     //     pair(tag(":"), not(alt((multispace1, eof)))).map(|x| x.0),
//     // )))
//     // .context("")
//     // .parse(source)?;
//     let (source, _) = structure_empty_until_newline_or_eof
//         .context("")
//         .parse(source)?;
//     // let flag = parts
//     //     .iter()
//     //     .map(|part| part.to_string())
//     //     .collect::<Vec<String>>()
//     //     .join("");
//     Ok((
//         source,
//         SectionAttr {
//             kind: SectionAttrKind::FlagV42 { value: vec![] },
//         },
//     ))
// }

// pub fn section_flag_attr_old<'a>(
//     source: &'a str,
// ) -> IResult<&'a str, SectionAttr, ErrorTree<&'a str>> {
//     let (source, _) = tag("--").context("").parse(source)?;
//     let (source, _) = space1.context("").parse(source)?;
//     let (source, key) = is_not(":\n").context("").parse(source)?;
//     let (source, _) = structure_empty_until_newline_or_eof
//         .context("")
//         .parse(source)?;
//     Ok((
//         source,
//         SectionAttr {
//             kind: SectionAttrKind::Flag {
//                 flag: key.trim().to_string(),
//             },
//         },
//     ))
// }

#[cfg(test)]
mod test {

    // use super::*;
    // use pretty_assertions::assert_eq;
    // use rstest::rstest;

    // // Valid Flags
    // #[rstest]
    // #[case("alfa")]
    // #[case("bravo-charlie")]
    // #[case("https://www.example.com/")]
    // fn solo_valid_section_flags(#[case] left: &str) {
    //     let source = format!("-- {}", left);
    //     let response = section_flag(source.as_str()).unwrap();
    //     let right = match response.1.kind {
    //         SectionAttrKind::Flag { flag } => flag,
    //         _ => "".to_string(),
    //     };
    //     assert_eq!(left, right);
    //     // let response = section_flag(source.as_str());
    //     // let left = match response.unwrap() {
    //     //     SectionAttrKind::Flag { flag } => flag,
    //     //     _ => "".to_string(),
    //     // };
    //     // if response.is_ok() {
    //     //     assert!(true)
    //     // } else {
    //     //     dbg!(&response);
    //     //     assert!(false);
    //     // }
    // }

    // // TODO: Move to useing section_flag instead of section_flag_v42
    // // Valid Flags
    // #[rstest]
    // #[case("-- alfa")]
    // #[case("-- bravo-charlie")]
    // #[case("-- https://www.example.com/")]
    // fn valid_section_flags(#[case] source: &str) {
    //     let response = section_flag_v42(source);
    //     if response.is_ok() {
    //         assert!(true)
    //     } else {
    //         dbg!(&response);
    //         assert!(false);
    //     }
    // }

    // // TODO: Move to useing section_flag instead of section_flag_v42
    // // Invalid Flags
    // #[rstest]
    // #[case("-- alfa: ")]
    // #[case("-- bravo: charlie")]
    // fn invalid_section_flags(#[case] source: &str) {
    //     let response = section_flag_v42(source);
    //     if response.is_ok() {
    //         dbg!(&response);
    //         assert!(false)
    //     } else {
    //         assert!(true);
    //     }
    // }

    // // Valid Attrs
    // #[rstest]
    // #[case("-- alfa: bravo")]
    // #[case("-- charlie-delta: echo")]
    // #[case("-- foxtrot: ``em|golf``")]
    // #[case("-- url: https://www.example.com/")]
    // fn valid_section_attrs(#[case] source: &str) {
    //     let response = section_key_value_attr(source);
    //     if response.is_ok() {
    //         assert!(true)
    //     } else {
    //         dbg!(&response);
    //         assert!(false);
    //     }
    // }

    // // Invalid Attrs
    // #[rstest]
    // #[case("-- alfa bravo: charlie")]
    // #[ignore]
    // // #[case("-- https://www.example.com/")]
    // fn invalid_section_attrs(#[case] source: &str) {
    //     let response = section_key_value_attr(source);
    //     if response.is_ok() {
    //         dbg!(&response);
    //         assert!(false);
    //     } else {
    //         assert!(true)
    //     }
    // }

    // // TODO: Put this back in:
    // // #[case("colon and hyphen", "-- created: 2024-06-09T14:28:09-04:00", true)]
    // // #[case("type with hyphen", "-- type: home-page", true)]
    // #[rstest]
    // // TODO: Add all the rest of the possible options here
    // #[case("basic key value test", "-- key: value", true)]
    // #[case("pipe", "-- key: some | thing", true)]
    // #[case("escaped pipe", "-- key: some \\| thing", true)]
    // #[case("path with slash", "-- path: /", true)]
    // #[case("flag with doulbe underscore", "-- alfa__bravo", true)]
    // #[ignore]
    // fn section_attr_basic_fixture(
    //     #[case] _description: &str,
    //     #[case] source: &str,
    //     #[case] should_pass: bool,
    // ) {
    //     if should_pass {
    //         assert_eq!("", section_attr(source).unwrap().0);
    //     }
    // }

    // // TODO: Move to useing section_flag instead of section_flag_v42
    // #[test]
    // #[ignore]
    // fn flag_attr_with_colons_in_it() {
    //     let source = "-- https://www.example.com";
    //     assert!(section_flag_v42(source).is_ok());
    // }

    // // TODO: Move to useing section_flag instead of section_flag_v42
    // #[test]
    // #[ignore]
    // fn flag_attr_with_hyphen() {
    //     let source = "-- https://www.exa-mple.com";
    //     assert!(section_flag_v42(source).is_ok());
    // }

    // // TODO: Move to useing section_flag instead of section_flag_v42
    // #[test]
    // #[ignore]
    // fn flag_attr_with_single_underscore() {
    //     let source = "-- alfa_bravo";
    //     assert!(section_flag_v42(source).is_ok());
    // }

    // #[test]
    // #[ignore]
    // fn key_value_spans_basic() {
    //     let source = "-- key: value";
    //     let left = SectionAttr {
    //         kind: SectionAttrKind::KeyValueSpans {
    //             key: "key".to_string(),
    //             spans: vec![Span {
    //                 attrs: vec![],
    //                 kind: SpanKind::PlainText {},
    //                 parsed_text: "value".to_string(),
    //             }],
    //         },
    //     };
    //     let right = section_attr(source).unwrap().1;
    //     assert_eq!(left, right);
    // }

    // TODO: Bring this test back when dash_with_trailing_not_dash is working
    // #[test]
    // #[skip]
    // fn key_value_with_hyphen() {
    //     let source = "-- key: alfa-bravo";
    //     let left = SectionAttr {
    //         kind: SectionAttrKind::KeyValueSpans {
    //             key: "key".to_string(),
    //             spans: vec![
    //                 Span {
    //                     attrs: vec![],
    //                     kind: SpanKind::PlainText {},
    //                     parsed_text: "alfa".to_string(),
    //                 },
    //                 Span {
    //                     attrs: vec![],
    //                     kind: SpanKind::Hyphen {},
    //                     parsed_text: "-".to_string(),
    //                 },
    //                 Span {
    //                     attrs: vec![],
    //                     kind: SpanKind::PlainText {},
    //                     parsed_text: "bravo".to_string(),
    //                 },
    //             ],
    //         },
    //     };
    //     let right = section_attr(source).unwrap().1;
    //     assert_eq!(left, right);
    // }

    // #[test]
    // #[ignore]
    // fn key_value_with_single_underscore() {
    //     let source = "-- key: alfa_bravo";
    //     let left = SectionAttr {
    //         kind: SectionAttrKind::KeyValueSpans {
    //             key: "key".to_string(),
    //             spans: vec![
    //                 Span {
    //                     attrs: vec![],
    //                     kind: SpanKind::PlainText {},
    //                     parsed_text: "alfa".to_string(),
    //                 },
    //                 Span {
    //                     attrs: vec![],
    //                     kind: SpanKind::SingleUnderscore {},
    //                     parsed_text: "_".to_string(),
    //                 },
    //                 Span {
    //                     attrs: vec![],
    //                     kind: SpanKind::PlainText {},
    //                     parsed_text: "bravo".to_string(),
    //                 },
    //             ],
    //         },
    //     };
    //     let right = section_attr(source).unwrap().1;
    //     assert_eq!(left, right);
    // }

    //
}

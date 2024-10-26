#![allow(unused_variables)]
#![allow(unused_imports)]

pub mod flag;
pub mod key_value;

use crate::section_attr_v42::flag::SectionFlagAttrV42;
use crate::section_attr_v42::key_value::SectionKeyValueAttrV42;
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

use self::flag::section_flag_attr_v42;
use self::key_value::section_key_value_attr_v42;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
// #[serde(tag = "type", rename_all = "lowercase")]
pub enum SectionAttrV42 {
    FlagV42(SectionFlagAttrV42),
    KeyValueV42(SectionKeyValueAttrV42),
}

pub fn section_attr_v42<'a>(
    source: &'a str,
) -> IResult<&'a str, SectionAttrV42, ErrorTree<&'a str>> {
    let (source, attr) = alt((section_flag_attr_v42, section_key_value_attr_v42))(source)?;
    Ok((source, attr))
}

#[cfg(test)]
mod test {

    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[test]
    fn section_attr_v42_get_flag() {
        let source = "-- alfa-flag";
        let left = SectionAttrV42::FlagV42(SectionFlagAttrV42 {
            value: "alfa-flag".to_string(),
        });
        let right = section_attr_v42(source);
        assert_eq!(left, right.unwrap().1);
    }

    // #[test]
    // fn section_attr_v42_get_key_value() {
    //     let source = "-- alfa: bravo";
    //     let left = SectionAttrV42::KeyValueV42(SectionKeyValueAttrV42 {
    //         key: "alfa".to_string(),
    //         value: vec![SpanV42 {
    //             attrs: vec![],
    //             kind: SpanKind::PlainText,
    //             parsed_text: "bravo".to_string(),
    //         }],
    //     });
    //     let right = section_attr_v42(source);
    //     assert_eq!(left, right.unwrap().1);
    // }

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

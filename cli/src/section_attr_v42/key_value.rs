use self::any_characters::any_colons;
use self::any_characters::any_pipes;
use self::plaintext_with_spaces_v42::plaintext_with_spaces_v42;
use self::shorthand_code::code_shorthand_span;
use self::shorthand_emphasis::emphasis_shorthand_span;
use self::shorthand_footnote::footnote_shorthand_span;
use self::shorthand_html::html_shorthand_span;
use self::shorthand_image::image_shorthand_span;
use self::shorthand_link::link_shorthand_span;
use self::shorthand_mark::mark_shorthand_span;
use self::shorthand_strikethrough::strikethrough_shorthand_span;
use self::shorthand_strong::strong_shorthand_span;
use self::shorthand_underline::underline_shorthand_span;
use self::space0_line_ending_or_eof::space0_line_ending_or_eof;
use crate::helpers::dump_parser_if_debug;
use crate::section_attr_v42::SectionAttrV42;
use crate::span_v42::*;
use crate::spans_v42::generic_span_attr_content::generic_span_attr_content;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::alphanumeric1;
use nom::character::complete::space1;
use nom::multi::many1;
use nom::sequence::terminated;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SectionKeyValueAttrV42 {
    pub key: String,
    pub value: Vec<SpanV42>,
}

pub fn section_key_value_attr_v42<'a>(
    source: &'a str,
) -> IResult<&'a str, SectionAttrV42, ErrorTree<&'a str>> {
    let debug = false;
    let dbg_header = "SECTION_KEY_VALUE_ATTR";
    dump_parser_if_debug(debug, dbg_header, source, 1);
    let (source, _) = tag("--").context("").parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 2);
    let (source, _) = space1.context("").parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 3);
    let (source, key_parts) = many1(alt((alphanumeric1, tag("-"), tag("_"))))
        .context("")
        .parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 4);
    let (source, _) = tag(":").context("").parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 5);
    let (source, _) = space1.context("").parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 6);
    let (source, value) = terminated(
        many1(alt((
            plaintext_with_spaces_v42,
            any_colons,
            any_pipes,
            code_shorthand_span,
            emphasis_shorthand_span,
            footnote_shorthand_span,
            html_shorthand_span,
            image_shorthand_span,
            link_shorthand_span,
            mark_shorthand_span,
            strikethrough_shorthand_span,
            strong_shorthand_span,
            underline_shorthand_span,
        ))),
        space0_line_ending_or_eof,
    )
    .context("")
    .parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 7);
    Ok((
        source,
        SectionAttrV42::KeyValueV42(SectionKeyValueAttrV42 {
            key: key_parts.join("").to_string(),
            value,
        }),
    ))
}

pub fn spans_of_section_key_value_attr_value<'a>(
    source: &'a str,
) -> IResult<&'a str, Vec<SpanV42>, ErrorTree<&'a str>> {
    let (source, output) = generic_span_attr_content.context("").parse(source)?;
    Ok((source, output))
}

// pub fn section_flag_attr_v42<'a>(
//     source: &'a str,
// ) -> IResult<&'a str, SectionAttrV42, ErrorTree<&'a str>> {
//     let (source, _) = tag("--").context("").parse(source)?;
//     let (source, _) = space1.context("").parse(source)?;
//     let (source, parts) = many1(alt((
//         is_not(": \n\t\\"),
//         pair(tag(":"), not(alt((multispace1, eof)))).map(|x| x.0),
//     )))
//     .context("")
//     .parse(source)?;
//     let (source, _) = structure_empty_until_newline_or_eof
//         .context("")
//         .parse(source)?;
//     let value = parts
//         .iter()
//         .map(|part| part.to_string())
//         .collect::<Vec<String>>()
//         .join("");
//     Ok((
//         source,
//         SectionAttrV42::FlagV42(SectionFlagAttrV42 {
//             value: value.trim().to_string(),
//         }),
//     ))
// }

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("-- alfa: bravo", "")]
    #[case("-- key-hyphens-are-okay: alfa", "")]
    #[case("-- alfa: value-hyphens-are-okay", "")]
    #[case("-- key_underscores_are_okay: alfa", "")]
    #[case("-- alfa: value_underscores_are_okay", "")]
    #[case("--    leading_key_spaces_are_okay: alfa", "")]
    #[case("-- alfa:   leading_value_spaces_are_okay", "")]
    #[case("-- alfa: ``can contain code, etc...``", "")]
    fn valid_cases_with_remainders(#[case] source: &str, #[case] remainder: &str) {
        let results = section_key_value_attr_v42(source);
        match results {
            Ok(result) => {
                if result.0 == remainder {
                    assert!(true);
                } else {
                    assert!(
                        false,
                        "Remainder should have been '{}' but got '{}'",
                        &remainder, result.0
                    );
                }
            }
            Err(e) => {
                assert!(false, "{}", e)
            }
        }
    }

    #[rstest]
    #[case("-- alfa")]
    #[case("-- alfa*bravo: charlie")]
    #[case("-- https://www.example.com/")]
    #[case("-- key-must-connect-to-colon : alfa")]
    fn test_invalid_cases(#[case] source: &str) {
        let results = section_key_value_attr_v42(source);
        match results {
            Ok(_) => {
                assert!(false, "This should not have parsed but it did: {}", source);
            }
            Err(_) => {
                assert!(true);
            }
        }
    }

    #[rstest]
    #[case("-- alfa: bravo charlie ")]
    #[case("-- alfa: bravo charlie\n")]
    #[case("-- alfa: bravo charlie \n")]
    fn section_key_value_attr_make_sure_ending_space_is_trimmed(#[case] source: &str) {
        let left = (
            "",
            SectionAttrV42::KeyValueV42(SectionKeyValueAttrV42 {
                key: "alfa".to_string(),
                value: vec![SpanV42::PlainText(plaintext::PlainTextSpan {
                    text: "bravo charlie".to_string(),
                })],
            }),
        );
        let right = section_key_value_attr_v42(source).unwrap();
        assert_eq!(left, right);
    }

    // #[rstest]
    // #[case("alfa", "bravo")]
    // #[case("charlie", "delta-echo")]
    // #[case("  leading_spaces", "are_ok")]
    // #[case("test_value", "    value_leading_spaces_are_ok")]
    // #[case("test_value", "value_trailing_spaces_are_ok    ")]
    // // #[case("test_value", "<<link|named_span|https://www.example.com/>>")]
    // // #[case("test_value", "``code shorthand``")]
    // // #[case("test_value", "``code shorthand|with_flag``")]
    // // TODO: do a character check of all the stuff that should work

    // // #[case("https://www.example.com/")]
    // // #[case("  leading_spaces_are_okay")]
    // // #[case("trailing_spaces_are_okay   ")]
    // // #[case("these_characters_are_okay:!@#$%^&*()[]<>|-")]
    // fn section_flag_attr_v42_valid_flags(#[case] left_key: &str, #[case] left_value: &str) {
    //     let source = format!("-- {}: {}", left_key, left_value);
    //     let response = section_key_value_attr_v42(source.as_str()).unwrap();
    //     let right = match response.1 {
    //         // NOTE: Only checking keys here which means the
    //         // parsing worked. Specific value checks are done
    //         // in other tests
    //         SectionAttrV42::KeyValueV42(data) => data.key,
    //         _ => "".to_string(),
    //     };
    //     assert_eq!(left_key.trim(), right);
    // }

    // Add to failing case
    // #[case("trailing_spaces_after_key_before_colon_fail   ", "x")]
    // don't allow single backslashes, they must be part of an escape

    // #[rstest]
    // #[case("alfa:")]
    // #[case("bravo: charlie")]
    // #[case("delta: ")]
    // #[case("no_escaped_\\allowed")]
    // fn section_flag_attr_v42_invalid_flags(#[case] left: &str) {
    //     let source = format!("-- {}", left);
    //     let result = section_flag_attr_v42(source.as_str());
    //     if result.is_err() {
    //         assert!(true)
    //     } else {
    //         dbg!(left);
    //         assert!(false)
    //     }
    // }

    //
}

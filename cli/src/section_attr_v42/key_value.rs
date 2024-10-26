#![allow(unused_imports)]
use crate::section_attr_v42::SectionAttrV42;
use crate::span_v42::*;
use crate::spans_v42::basic_content::spans_of_basic_content;
// use crate::section::block::block_of_main_content_v42;
// use crate::section::block::block_of_section_attr_value_v42;
// use crate::span::colon::colon;
// use crate::span::double_underscore::double_underscore;
// use crate::span::escaped_pipe::escaped_pipe;
// use crate::span::hyphen::hyphen;
// use crate::span::pipe::pipe;
// use crate::span::single_underscore::single_underscore;
use crate::helpers::dump_parser_if_debug;
use crate::spans_v42::section_key_value_attr_value::spans_of_section_key_value_attr_value;
// use crate::span::*;
use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::character::complete::anychar;
use nom::character::complete::line_ending;
use nom::character::complete::multispace0;
use nom::character::complete::multispace1;
use nom::character::complete::space0;
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
    let (source, value) = spans_of_section_key_value_attr_value
        .context("")
        .parse(source)?;
    let (source, _) = space0.context("").parse(source)?;
    let (source, _) = alt((line_ending, eof)).context("").parse(source)?;
    dump_parser_if_debug(debug, dbg_header, source, 7);
    let key = key_parts
        .iter()
        .map(|part| part.to_string())
        .collect::<Vec<String>>()
        .join("")
        .trim()
        .to_string();
    Ok((
        source,
        SectionAttrV42::KeyValueV42(SectionKeyValueAttrV42 { key, value }),
    ))
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
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case("alfa", "bravo")]
    #[case("charlie", "delta-echo")]
    #[case("  leading_spaces", "are_ok")]
    #[case("test_value", "    value_leading_spaces_are_ok")]
    #[case("test_value", "value_trailing_spaces_are_ok    ")]
    // #[case("test_value", "<<link|named_span|https://www.example.com/>>")]
    // #[case("test_value", "``code shorthand``")]
    // #[case("test_value", "``code shorthand|with_flag``")]
    // TODO: do a character check of all the stuff that should work

    // #[case("https://www.example.com/")]
    // #[case("  leading_spaces_are_okay")]
    // #[case("trailing_spaces_are_okay   ")]
    // #[case("these_characters_are_okay:!@#$%^&*()[]<>|-")]
    fn section_flag_attr_v42_valid_flags(#[case] left_key: &str, #[case] left_value: &str) {
        let source = format!("-- {}: {}", left_key, left_value);
        let response = section_key_value_attr_v42(source.as_str()).unwrap();
        let right = match response.1 {
            // NOTE: Only checking keys here which means the
            // parsing worked. Specific value checks are done
            // in other tests
            SectionAttrV42::KeyValueV42(data) => data.key,
            _ => "".to_string(),
        };
        assert_eq!(left_key.trim(), right);
    }

    // Add to failing case
    // #[case("trailing_spaces_after_key_before_colon_fail   ", "x")]
    // don't allow single backslashes, they must be part of an escape

    // #[rstest]
    // #[case("alfa:")]
    // #[case("bravo: charlie")]
    // #[case("delta: ")]
    // #[case("no_escaped_\\allowed")]
    // fn solo_section_flag_attr_v42_invalid_flags(#[case] left: &str) {
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

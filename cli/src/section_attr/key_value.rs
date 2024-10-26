#![allow(unused_imports)]
use crate::section_attr::SectionAttrV42;
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

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SectionKeyValueAttrV42 {
    key: String,
    value: Vec<Span>,
}

pub fn section_key_value_attr_v42<'a>(
    source: &'a str,
) -> IResult<&'a str, SectionAttrV42, ErrorTree<&'a str>> {
    Ok((
        source,
        SectionAttrV42::KeyValueV42(SectionKeyValueAttrV42 {
            key: "".to_string(),
            value: vec![],
        }),
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
    // #[case("bravo-charlie")]
    // #[case("https://www.example.com/")]
    // #[case("  leading_spaces_are_okay")]
    // #[case("trailing_spaces_are_okay   ")]
    // #[case("these_characters_are_okay:!@#$%^&*()[]<>|-")]
    fn solo_section_flag_attr_v42_valid_flags(#[case] left_key: &str, #[case] left_value: &str) {
        let source = format!("-- {}: {}", left_key, left_value);
        let _response = section_key_value_attr_v42(source.as_str()).unwrap();
        //let right = match response.1 {
        //    //SectionAttrV42::FlagV42(data) => data.value,
        //    _ => "".to_string(),
        //};
        // assert_eq!(left.trim(), right);
    }

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

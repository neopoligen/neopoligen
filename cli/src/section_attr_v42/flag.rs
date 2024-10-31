use crate::section_attr_v42::SectionAttrV42;
use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::character::complete::multispace1;
use nom::character::complete::space0;
use nom::character::complete::space1;
use nom::combinator::eof;
use nom::combinator::not;
use nom::multi::many1;
use nom::sequence::pair;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SectionFlagAttrV42 {
    pub value: String,
}

pub fn section_flag_attr_v42<'a>(
    source: &'a str,
) -> IResult<&'a str, SectionAttrV42, ErrorTree<&'a str>> {
    let (source, _) = tag("--").context("").parse(source)?;
    let (source, _) = space1.context("").parse(source)?;
    let (source, parts) = many1(alt((
        is_not(": \n\t\\"),
        pair(tag(":"), not(alt((multispace1, eof)))).map(|x| x.0),
    )))
    .context("")
    .parse(source)?;
    let (source, _) = alt((pair(space0, line_ending), pair(space0, eof)))
        .context("")
        .parse(source)?;
    let value = parts
        .iter()
        .map(|part| part.to_string())
        .collect::<Vec<String>>()
        .join("");
    Ok((
        source,
        SectionAttrV42::FlagV42(SectionFlagAttrV42 {
            value: value.trim().to_string(),
        }),
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case("alfa")]
    #[case("bravo-charlie")]
    #[case("https://www.example.com/")]
    #[case("  leading_spaces_are_okay")]
    #[case("trailing_spaces_are_okay   ")]
    #[case("these_characters_are_okay:!@#$%^&*()[]<>|-")]
    fn section_flag_attr_v42_valid_flags(#[case] left: &str) {
        let source = format!("-- {}", left);
        let response = section_flag_attr_v42(source.as_str()).unwrap();
        let right = match response.1 {
            SectionAttrV42::FlagV42(data) => data.value,
            _ => "".to_string(),
        };
        assert_eq!(left.trim(), right);
    }

    #[rstest]
    #[case("alfa:")]
    #[case("bravo: charlie")]
    #[case("delta: ")]
    #[case("no_escaped_\\allowed")]
    fn section_flag_attr_v42_invalid_flags(#[case] left: &str) {
        let source = format!("-- {}", left);
        let result = section_flag_attr_v42(source.as_str());
        if result.is_err() {
            assert!(true)
        } else {
            dbg!(left);
            assert!(false)
        }
    }

    //
}

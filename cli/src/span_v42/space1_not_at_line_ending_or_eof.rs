// use nom::branch::alt;
// use nom::character::complete::line_ending;
use nom::character::complete::multispace1;
use nom::character::complete::space1;
use nom::combinator::eof;
use nom::combinator::not;
use nom::sequence::pair;
use nom::sequence::terminated;
// use nom::sequence::tuple;
use crate::span_v42::PlainTextSpan;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;

use super::SpanV42;

pub fn space1_not_at_line_ending_or_eof(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    let (source, _) = terminated(space1, pair(not(eof), not(multispace1)))
        .context("")
        .parse(source)?;
    Ok((
        source,
        SpanV42::PlainText(PlainTextSpan {
            text: " ".to_string(),
        }),
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(" alfa", "alfa")]
    fn solo_valid_cases_with_remainders(#[case] source: &str, #[case] remainder: &str) {
        let results = space1_not_at_line_ending_or_eof(source);
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
    #[case(" ")]
    #[case(" \n")]
    fn solo_invalid_cases(#[case] source: &str) {
        let results = space1_not_at_line_ending_or_eof(source);
        match results {
            Ok(_) => {
                assert!(false, "This should not have parsed but it did: {}", source);
            }
            Err(_) => {
                assert!(true);
            }
        }
    }
}

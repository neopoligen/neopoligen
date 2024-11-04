use super::any_characters::*;
use super::plaintext_base_v42::plaintext_base_v42;
use super::single_characters::*;
use super::space0_line_ending_or_eof::space0_line_ending_or_eof;
use super::space1_not_at_line_ending_or_eof::space1_not_at_line_ending_or_eof;
use crate::helpers::flatten_spans_v42;
use crate::span_v42::plaintext::PlainTextSpan;
use crate::span_v42::SpanV42;
use nom::branch::alt;
use nom::character::complete::space0;
use nom::combinator::eof;
use nom::multi::many1;
use nom::sequence::terminated;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::parser_ext::ParserExt;

pub fn plaintext_with_spaces_v42(source: &str) -> IResult<&str, SpanV42, ErrorTree<&str>> {
    // " \t\r\n~`*^![]{}<>|:_-\\"
    let (source, spans) = terminated(
        many1(alt((
            plaintext_base_v42,
            any_hyphens,
            any_tabs,
            space1_not_at_line_ending_or_eof,
        ))),
        alt((eof, space0)),
    )
    .context("")
    .parse(source)?;
    let text = flatten_spans_v42(&spans);
    Ok((source, SpanV42::PlainText(PlainTextSpan { text })))
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("alfa", "")]
    #[case("alfa bravo", "")]
    #[case("alfa  bravo", "")]
    #[case("alfa-bravo", "")]
    #[case("alfa--bravo", "")]
    #[case("alfa\tbravo", "")]
    #[case("alfa\t\tbravo", "")]
    #[case("don't caputre newlines\n", "\n")]
    #[case("don't caputre newlines\n\n", "\n\n")]
    #[case("chomp spaces before newline \n", "\n")]
    #[case("chomp spaces before newline    \n", "\n")]
    fn solo_valid_cases_with_remainders(#[case] source: &str, #[case] remainder: &str) {
        let results = plaintext_with_spaces_v42(source);
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

    // #[rstest]
    // fn solo_invalid_cases(#[case] source: &str) {
    //     let results = plaintext_with_spaces_v42(source);
    //     match results {
    //         Ok(_) => {
    //             assert!(false, "This should not have parsed but it did: {}", source);
    //         }
    //         Err(_) => {
    //             assert!(true);
    //         }
    //     }
    // }

    //
}

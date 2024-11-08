use super::generic_base_content_v42::generic_base_content_v42;
use crate::span_v42::any_characters::*;
use crate::span_v42::block_line_ending::block_line_ending;
use crate::span_v42::string_parsers::span_of_escaped_character;
use crate::span_v42::string_parsers::span_of_plain_text_for_block_paragraph;
use crate::span_v42::SpanV42;
use nom::branch::alt;
use nom::multi::many1;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::ParserExt;

pub fn generic_block_content_v42<'a>(
    source: &'a str,
) -> IResult<&'a str, Vec<SpanV42>, ErrorTree<&'a str>> {
    let (source, results) = many1(alt((
        span_of_plain_text_for_block_paragraph,
        span_of_escaped_character,
    )))
    .context("")
    .parse(source)?;
    // let (source, results) = many1(alt((
    //     generic_base_content_v42,
    //     block_line_ending,
    //     any_pipes,
    //     any_colons,
    // )))
    // .context("")
    // .parse(source)?;
    Ok((source, results))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::helpers::get_include_str_file;
    use include_dir::{include_dir, Dir};
    use rstest::rstest;

    const VALID_SAMPLES: Dir = include_dir!("src/spans_v42/samples/generic_block_content/valid");

    // TODO: Test these: \r<>`|_!@#$^&*\\
    #[rstest]
    #[case("alfa", "")]
    #[case("alfa bravo", "")]
    #[case("alfa-bravo", "")]
    #[case("alfa -- bravo", "")]
    #[case("alfa\nbravo", "")]
    #[case("alfa\n\nbravo", "\n\nbravo")]
    #[case("alfa\n\n-- bravo", "\n\n-- bravo")]
    #[case("alfa (bravo) charlie", "")]
    #[case("alfa ((bravo)) charlie", "")]
    #[case("alfa:bravo", "")]
    #[case("alfa::bravo", "")]
    #[case("``alfa``", "")]
    //#[case("``alfa|bravo``", "")]
    //#[case("``alfa|charlie: delta``", "")]

    fn verify_strings(#[case] source: &str, #[case] remainder: &str) {
        match generic_block_content_v42(source) {
            Ok(result) => {
                if result.clone().0 == remainder {
                    assert!(true)
                } else {
                    dbg!(result);
                    assert!(false)
                }
            }
            Err(e) => {
                dbg!(e);
                assert!(false)
            }
        }
    }

    #[rstest]
    #[case("001-plaintext.txt")]
    #[case("002-with-hyphen.txt")]
    #[case("003-named-spans.txt")]
    fn verify_files(#[case] file: &str) {
        let source = get_include_str_file(VALID_SAMPLES, file);
        match generic_block_content_v42(&source) {
            Ok(_) => assert!(true),
            Err(e) => {
                dbg!(e);
                assert!(false)
            }
        }
    }
}

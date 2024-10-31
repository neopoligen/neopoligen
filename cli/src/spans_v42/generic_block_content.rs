use super::generic_base_content::generic_base_content;
use crate::span_v42::block_line_ending::block_line_ending;
use crate::span_v42::single_characters::*;
use crate::span_v42::SpanV42;
use nom::branch::alt;
use nom::multi::many1;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::ParserExt;

pub fn spans_of_generic_block_content<'a>(
    source: &'a str,
) -> IResult<&'a str, Vec<SpanV42>, ErrorTree<&'a str>> {
    let (source, output) = many1(alt((generic_base_content, block_line_ending, single_pipe)))
        .context("")
        .parse(source)?;
    Ok((source, output))
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
        match spans_of_generic_block_content(source) {
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
        match spans_of_generic_block_content(&source) {
            Ok(_) => assert!(true),
            Err(e) => {
                dbg!(e);
                assert!(false)
            }
        }
    }
}

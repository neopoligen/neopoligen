// use crate::span::named_span::named_span_type_span_v42;
use crate::span_v42::any_hyphens::any_hyphens;
use crate::span_v42::block_line_ending::block_line_ending;
use crate::span_v42::named_span::named_span;
use crate::span_v42::plaintext::plaintext;
use crate::span_v42::single_exclamation::single_exclamation;
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
    let (source, output) = many1(alt((
        plaintext,
        any_hyphens,
        single_exclamation,
        block_line_ending,
        named_span,
    )))
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

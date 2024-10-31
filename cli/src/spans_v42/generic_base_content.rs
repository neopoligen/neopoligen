use crate::span_v42::any_colons::any_colons;
use crate::span_v42::any_hyphens::any_hyphens;
use crate::span_v42::any_paren_closes::any_paren_closes;
use crate::span_v42::any_paren_opens::any_paren_opens;
use crate::span_v42::code_shorthand::*;
use crate::span_v42::emphasis_shorthand::*;
use crate::span_v42::footnote_shorthand::footnote_shorthand_span;
use crate::span_v42::named_span::named_span;
use crate::span_v42::plaintext::plaintext;
use crate::span_v42::single_backtick::single_backtick;
use crate::span_v42::single_exclamation::single_exclamation;
use crate::span_v42::single_greater_than::single_greater_than;
use crate::span_v42::single_less_than::single_less_than;
use crate::span_v42::single_underscore::single_underscore;
use crate::span_v42::strong_shorthand::strong_shorthand_span;
use crate::span_v42::SpanV42;
use nom::branch::alt;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::ParserExt;

pub fn generic_base_content<'a>(source: &'a str) -> IResult<&'a str, SpanV42, ErrorTree<&'a str>> {
    let (source, output) = alt((
        // "<>`-|:_!@#$^&*()~\\"
        plaintext,
        single_less_than,
        single_greater_than,
        single_backtick,
        // Reminder: don't do single_pipe here, but in block content instead
        single_underscore,
        single_exclamation,
        named_span,
        code_shorthand_span,
        emphasis_shorthand_span,
        footnote_shorthand_span,
        strong_shorthand_span,
        any_hyphens,
        any_colons,
        any_paren_opens,
        any_paren_closes,
    ))
    .context("")
    .parse(source)?;
    Ok((source, output))
}

#[cfg(test)]
mod test {

    // use super::*;
    // use crate::helpers::get_include_str_file;
    // use include_dir::{include_dir, Dir};
    // use rstest::rstest;

    //const VALID_SAMPLES: Dir = include_dir!("src/spans_v42/samples/generic_block_content/valid");

    //// TODO: Test these: \r<>`|_!@#$^&*\\
    //#[rstest]
    //#[case("alfa", "")]
    //#[case("alfa bravo", "")]
    //#[case("alfa-bravo", "")]
    //#[case("alfa -- bravo", "")]
    //#[case("alfa\nbravo", "")]
    //#[case("alfa\n\nbravo", "\n\nbravo")]
    //#[case("alfa\n\n-- bravo", "\n\n-- bravo")]
    //#[case("alfa (bravo) charlie", "")]
    //#[case("alfa ((bravo)) charlie", "")]
    //#[case("alfa:bravo", "")]
    //#[case("alfa::bravo", "")]
    //#[case("``alfa``", "")]
    ////#[case("``alfa|bravo``", "")]
    ////#[case("``alfa|charlie: delta``", "")]

    // fn verify_strings(#[case] source: &str, #[case] remainder: &str) {
    //     match spans_of_generic_block_content(source) {
    //         Ok(result) => {
    //             if result.clone().0 == remainder {
    //                 assert!(true)
    //             } else {
    //                 dbg!(result);
    //                 assert!(false)
    //             }
    //         }
    //         Err(e) => {
    //             dbg!(e);
    //             assert!(false)
    //         }
    //     }
    // }

    // #[rstest]
    // #[case("001-plaintext.txt")]
    // #[case("002-with-hyphen.txt")]
    // #[case("003-named-spans.txt")]
    // fn verify_files(#[case] file: &str) {
    //     let source = get_include_str_file(VALID_SAMPLES, file);
    //     match spans_of_generic_block_content(&source) {
    //         Ok(_) => assert!(true),
    //         Err(e) => {
    //             dbg!(e);
    //             assert!(false)
    //         }
    //     }
    // }

    //
}

use crate::span_v42::escaped_characters::*;
use crate::span_v42::named_span::named_span;
use crate::span_v42::plaintext::plaintext;
use crate::span_v42::shorthand_code::*;
use crate::span_v42::shorthand_emphasis::*;
use crate::span_v42::shorthand_footnote::*;
use crate::span_v42::shorthand_html::*;
use crate::span_v42::shorthand_image::*;
use crate::span_v42::shorthand_link::*;
use crate::span_v42::shorthand_mark::*;
use crate::span_v42::shorthand_strikethrough::*;
use crate::span_v42::shorthand_strong::*;
use crate::span_v42::shorthand_underline::*;
use crate::span_v42::single_characters::*;
use crate::span_v42::SpanV42;
use nom::branch::alt;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::ParserExt;

pub fn generic_base_content<'a>(source: &'a str) -> IResult<&'a str, SpanV42, ErrorTree<&'a str>> {
    let (source, output) = alt((
        alt((
            plaintext,
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
            named_span,
        )),
        alt((
            // Reminder: don't do single_pipe or single_colon
            // those go in higher level parsers
            single_tilde,
            single_backtick,
            single_asterisk,
            single_caret,
            single_exclamation,
            single_open_bracket,
            single_close_bracket,
            single_open_brace,
            single_close_brace,
            single_underscore,
            single_ampersand,
            single_less_than,
            single_greater_than,
        )),
        alt((
            escaped_tilde,
            escaped_backtick,
            escaped_asterisk,
            escaped_caret,
            escaped_exclamation,
            escaped_open_bracket,
            escaped_close_bracket,
            escaped_open_brace,
            escaped_close_brace,
            escaped_underscore,
            escaped_ampersand,
            escaped_less_than,
            escaped_greater_than,
        )),
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

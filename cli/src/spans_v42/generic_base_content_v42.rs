use crate::span_v42::named_span::named_span;
use crate::span_v42::plaintext_with_spaces_v42::plaintext_with_spaces_v42;
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
use crate::span_v42::SpanV42;
use nom::branch::alt;
use nom::IResult;
use nom::Parser;
use nom_supreme::error::ErrorTree;
use nom_supreme::ParserExt;

pub fn generic_base_content_v42<'a>(
    source: &'a str,
) -> IResult<&'a str, SpanV42, ErrorTree<&'a str>> {
    let (source, output) = alt((
        plaintext_with_spaces_v42,
        named_span,
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
    ))
    .context("")
    .parse(source)?;
    Ok((source, output))
}

#[cfg(test)]
mod test {

    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    // TODO: Test these: \r<>`|_!@#$^&*\\
    #[rstest]
    #[case("alfa", "")]
    #[case("alfa bravo", "")]
    #[case("alfa-bravo", "")]
    #[case("alfa -- bravo", "")]
    #[case("alfa\nbravo", "\nbravo")]
    #[case("alfa\n\nbravo", "\n\nbravo")]
    #[case("alfa\n\n-- bravo", "\n\n-- bravo")]
    #[case("alfa (bravo) charlie", "")]
    #[case("alfa ((bravo)) charlie", "")]
    #[case("``alfa``", "")]

    fn verify_strings(#[case] source: &str, #[case] remainder: &str) {
        match generic_base_content_v42(source) {
            Ok(result) => {
                assert_eq!(result.clone().0, remainder)
            }
            Err(e) => {
                dbg!(e);
                assert!(false)
            }
        }
    }

    //
}

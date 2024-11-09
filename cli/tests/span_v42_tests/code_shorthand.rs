use neopoligengine::span_v42::shorthand_code::code_shorthand_span;

use pretty_assertions::assert_eq;
use rstest::rstest;

#[rstest]
#[case("``alfa``", "")]
#[case("``alfa bravo``", "")]
#[case("``alfa     bravo``", "")]
#[case("``  alfa     bravo  ``", "")]
#[case("``\nalfa bravo``", "")]
#[case("``alfa\nbravo``", "")]
fn solo_code_shorthand_valid_tests(#[case] source: &str, #[case] remainder: &str) {
    let left = remainder;
    let parsed = code_shorthand_span(source).unwrap();
    dbg!(&parsed.1);
    let right = parsed.0;
    assert_eq!(left, right);
}

// #[rstest]
// #[case("alfa", &["alfa"], "")]
// #[case("alfa bravo", &["alfa", " ", "bravo"], "")]
// #[case("alfa    bravo", &["alfa", " ", "bravo"], "")]
// #[case("alfa\nbravo", &["alfa", " ", "bravo"], "")]
// #[case("alfa\n\nbravo", &["alfa"], "\n\nbravo")]
// #[case("alfa~bravo", &["alfa", "~", "bravo"], "")]
// #[case("alfa|bravo", &["alfa", "|", "bravo"], "")]
// #[case("alfa||bravo", &["alfa", "||", "bravo"], "")]
// #[case("alfa:bravo", &["alfa", ":", "bravo"], "")]
// #[case("alfa::bravo", &["alfa", "::", "bravo"], "")]
// #[case("alfa~~bravo~~", &["alfa"], "~~bravo~~")]
// #[case("alfa ~~bravo~~", &["alfa", " "], "~~bravo~~")]
// fn plain_text_span_for_block(#[case] source: &str, #[case] got: &[&str], #[case] remainder: &str) {
//     let matcher = (remainder, got.to_vec());
//     let parsed = plain_text_span_for_block_paragraph(source).unwrap();
//     assert_eq!(matcher, parsed);
// }

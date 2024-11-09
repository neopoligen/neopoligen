use neopoligengine::span_v42::{shorthand_code::code_shorthand_span, SpanV42};

use pretty_assertions::assert_eq;
use rstest::rstest;

#[rstest]
#[case("``alfa``", "", 0, 0)]
#[case("``alfa bravo``", "", 0, 0)]
#[case("``alfa     bravo``", "", 0, 0)]
#[case("``  alfa     bravo  ``", "", 0, 0)]
#[case("``\nalfa bravo``", "", 0, 0)]
#[case("``alfa\nbravo``", "", 0, 0)]
#[case("``alfa <<bravo>>``", "", 0, 0)]
#[case("``alfa ` bravo``", "", 0, 0)]
// #[case("``alfa \\| bravo``", "", 0, 0)]

fn solo_code_shorthand_valid_tests(
    #[case] source: &str,
    #[case] remainder: &str,
    #[case] attrs: usize,
    #[case] flags: usize,
) {
    let parsed = code_shorthand_span(source).unwrap();
    if &remainder == &parsed.0 {
        match &parsed.1 {
            SpanV42::CodeShorthand(span) => {
                assert_eq!(
                    attrs,
                    span.attrs.len(),
                    "Incorrect number of attrs. Expected: {} - Got: {}",
                    &attrs,
                    span.attrs.len()
                );
                assert!(true);
            }
            _ => assert!(false, "Did not get CodeShorthand span"),
        };
    } else {
        assert!(
            false,
            "Incorrect remainder. Expected: '{}', Got: '{}'",
            remainder, &parsed.0
        );
    }
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

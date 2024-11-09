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
#[case("``alfa \\| bravo``", "", 0, 0)]
#[case("``alfa~bravo``", "", 0, 0)]
#[case("``alfa\\|\\|bravo``", "", 0, 0)]
#[case("``alfa:bravo``", "", 0, 0)]
#[case("``alfa::bravo``", "", 0, 0)]
#[case("``alfa~~bravo~~``", "", 0, 0)]
#[case("``alfa|bravo``", "", 1, 0)]
fn solo_code_shorthand_valid_tests(
    #[case] source: &str,
    #[case] remainder: &str,
    #[case] flags: usize,
    #[case] attrs: usize,
) {
    let parsed = code_shorthand_span(source).unwrap();
    assert_eq!(
        &remainder, &parsed.0,
        "Incorrect remainder. Expected: {} - Got: {}",
        &remainder, &parsed.0
    );
    match &parsed.1 {
        SpanV42::CodeShorthand(span) => {
            assert_eq!(
                attrs,
                span.attrs.len(),
                "Incorrect number of attrs. Expected: {} - Got: {}",
                &attrs,
                span.attrs.len()
            );
            assert_eq!(
                flags,
                span.flags.len(),
                "Incorrect number of flags. Expected: {} - Got: {}",
                &flags,
                span.flags.len()
            );
        }
        _ => assert!(false, "Did not get CodeShorthand span"),
    };
}

#[rstest]
#[case("``alfa\n\nbravo``")]
fn solo_code_shorthand_invalid_tests(#[case] source: &str) {
    let parsed = code_shorthand_span(source);
    if parsed.is_err() {
        assert!(true);
    } else {
        dbg!(&parsed);
        assert!(false, "Should not have parsed but did");
    }
}

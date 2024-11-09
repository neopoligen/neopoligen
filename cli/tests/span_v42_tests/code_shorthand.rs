use std::collections::BTreeMap;

use neopoligengine::span_v42::{
    plaintext::PlainTextSpan,
    shorthand_code::{code_shorthand_span, CodeShorthandV42},
    SpanV42,
};
use pretty_assertions::assert_eq;
use rstest::rstest;

#[test]
fn solo_code_shorthand_integration_test_1() {
    let source = "``alfa|bravo|charlie: delta``";
    let mut attrs = BTreeMap::new();
    attrs.insert(
        "charlie".to_string(),
        vec![SpanV42::PlainText(PlainTextSpan {
            kind: "plain-text".to_string(),
            text: "delta".to_string(),
        })],
    );
    let left = SpanV42::CodeShorthand(CodeShorthandV42 {
        attrs,
        flags: vec!["bravo".to_string()],
        kind: "code-shorthand".to_string(),
        text: "alfa".to_string(),
    });
    let right = code_shorthand_span(source).unwrap().1;
    assert_eq!(left, right);
}

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
#[case("``alfa|https://www.example.com/``", "", 1, 0)]
#[case("``alfa|  bravo``", "", 1, 0)]
#[case("``alfa|\nbravo``", "", 1, 0)]
#[case("``alfa|bravo\n``", "", 1, 0)]
#[case("``alfa| \n bravo``", "", 1, 0)]
#[case("``alfa|bravo \n ``", "", 1, 0)]
#[case("``alfa|bravo\\:charlie``", "", 1, 0)]
#[case("``alfa|bravo\\|charlie``", "", 1, 0)]
#[case("``alfa|bravo: charlie``", "", 0, 1)]
#[case("``alfa|\nbravo: charlie``", "", 0, 1)]
#[case("``alfa|bravo: charlie\n``", "", 0, 1)]
#[case("``alfa|  bravo: charlie``", "", 0, 1)]
#[case("``alfa|bravo: charlie  ``", "", 0, 1)]
#[case("``alfa|bravo: <<link|ping>>``", "", 0, 1)]
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
#[case("``alfa|\n\nbravo``")]
#[case("``alfa|bravo\n\n``")]
fn solo_code_shorthand_invalid_tests(#[case] source: &str) {
    let parsed = code_shorthand_span(source);
    if parsed.is_err() {
        assert!(true);
    } else {
        dbg!(&parsed);
        assert!(false, "Should not have parsed but did");
    }
}

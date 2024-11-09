use minijinja::Value;
use neopoligengine::span_v42::{shorthand_code::code_shorthand_span, SpanV42};
use pretty_assertions::assert_eq;
use rstest::rstest;

#[rstest]
#[case(
    "``alfa``",
    r#"{ attrs: {}, flags: [], kind: "code-shorthand", text: "alfa", type: "code-shorthand" }"#
)]
#[case(
    "``alfa|bravo``",
    r#"{ attrs: {}, flags: ["bravo"], kind: "code-shorthand", text: "alfa", type: "code-shorthand" }"#
)]
#[case(
    "``alfa|bravo: charlie``",
    r#"{ attrs: {
        bravo: [
            {
                kind: "plain-text",
                text: "charlie",
                type: "plain-text",
            }
        ]
    }, flags: [], kind: "code-shorthand", text: "alfa", type: "code-shorthand" }"#
)]
#[case(
    "``alfa|bravo|charlie: delta``",
    r#"{ 
    attrs: {
        charlie: [
            {
                kind: "plain-text",
                text: "delta",
                type: "plain-text",
            }
        ]
    }, 
    flags: ["bravo"], 
    kind: "code-shorthand", 
    text: "alfa", 
    type: "code-shorthand" }"#
)]

fn solo_code_integration_tests(#[case] source: &str, #[case] json5_string: &str) {
    let left = serde_json5::from_str::<Value>(json5_string).unwrap();
    let parsed = code_shorthand_span(source);
    let right = Value::from_serialize(&parsed.unwrap().1);
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

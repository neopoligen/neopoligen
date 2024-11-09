use minijinja::Value;
use neopoligengine::span_v42::{shorthand_code::code_shorthand_span, SpanV42};
use pretty_assertions::assert_eq;
use rstest::rstest;

#[rstest]
#[case(
    "Basic Test",
    "``alfa``",
    r#"{ attrs: {}, flags: [], kind: "code-shorthand", text: "alfa", type: "code-shorthand" }"#
)]
#[case(
    "Single Flag",
    "``alfa|bravo``",
    r#"{ attrs: {}, flags: ["bravo"], kind: "code-shorthand", text: "alfa", type: "code-shorthand" }"#
)]
#[case(
    "Single Attr",
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
    "Attr With Single Shorthand Character",
    "``alfa|bravo: charlie ` delta``",
    r#"{ attrs: {
        bravo: [
            {
                kind: "plain-text",
                text: "charlie ` delta",
                type: "plain-text",
            }
        ]
    }, flags: [], kind: "code-shorthand", text: "alfa", type: "code-shorthand" }"#
)]
#[case(
    "Attr With Escaped Shorthand Character",
    "``alfa|bravo: charlie `\\` delta``",
    r#"{ attrs: {
        bravo: [
            {
                kind: "plain-text",
                text: "charlie `",
                type: "plain-text",
            },
            {
                kind: "escaped-character",
                text: "`",
                type: "escaped-character",
            },
            {
                kind: "plain-text",
                text: " delta",
                type: "plain-text",
            },
        ]
    }, flags: [], kind: "code-shorthand", text: "alfa", type: "code-shorthand" }"#
)]
#[case(
    "Attr With Shorthand String That Stays Text",
    "``alfa|bravo: __charlie__``",
    r#"{ attrs: {
        bravo: [
            {
                kind: "plain-text",
                text: "__",
                type: "plain-text",
            },
            {
                kind: "plain-text",
                text: "charlie",
                type: "plain-text",
            },
            {
                kind: "plain-text",
                text: "__",
                type: "plain-text",
            },
        ]
    }, flags: [], kind: "code-shorthand", text: "alfa", type: "code-shorthand" }"#
)]
#[case(
    "Both Flag And Attr",
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
#[case(
    "Attr With Named_Span",
    "``alfa|bravo: charlie <<delta|echo>>``",
    r#"{ 
    attrs: {
        bravo: [
            {
                kind: "plain-text",
                text: "charlie ",
                type: "plain-text",
            },
            {
                attrs: {},
                flags: [],
                kind: "delta",
                spans: [
                    {
                        kind: "plain-text",
                        text: "echo",
                        type: "plain-text",
                    }
                ],
                type: "named-span",
            }
        ]
    }, 
    flags: [], 
    kind: "code-shorthand", 
    text: "alfa", 
    type: "code-shorthand" }"#
)]

fn solo_code_integration_tests(
    #[case] description: &str,
    #[case] source: &str,
    #[case] json5_string: &str,
) {
    let left = serde_json5::from_str::<Value>(json5_string).unwrap();
    let parsed = code_shorthand_span(source);
    let right = Value::from_serialize(&parsed.unwrap().1);
    assert_eq!(left, right, "Test: {}", description);
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
#[case("``alfa\n\n|bravo``")]
#[case("``alfa|\n\nbravo``")]
#[case("``alfa|bravo\n\n``")]
#[case("``alfa\n\n|bravo: charle``")]
#[case("``alfa|\n\nbravo: charle``")]
#[case("``alfa|bravo:\n\ncharle``")]
#[case("``alfa|bravo: charle\n\n``")]
fn solo_code_shorthand_invalid_tests(#[case] source: &str) {
    let parsed = code_shorthand_span(source);
    if parsed.is_err() {
        assert!(true);
    } else {
        dbg!(&parsed);
        assert!(false, "Should not have parsed but did");
    }
}

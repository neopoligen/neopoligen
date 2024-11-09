use minijinja::Value;
use neopoligengine::span_v42::{named_span::named_span, SpanV42};
use pretty_assertions::assert_eq;
use rstest::rstest;

#[rstest]
#[case(
    "Basic Test",
    "<<br>>",
    r#"{ attrs: {}, flags: [], kind: "br", spans: null, type: "named-span" }"#
)]
fn solo_named_span_integration_tests(
    #[case] description: &str,
    #[case] source: &str,
    #[case] json5_string: &str,
) {
    let left = serde_json5::from_str::<Value>(json5_string).unwrap();
    let parsed = named_span(source);
    let right = Value::from_serialize(&parsed.unwrap().1);
    assert_eq!(left, right, "Test: {}", description);
}

#[rstest]
#[case("<<alfa>>", "", 0, 0)]
#[case("<<alfa-bravo>>", "", 0, 0)]
#[case("<<alfa_bravo>>", "", 0, 0)]
#[case("<<em|alfa>>etc...", "etc...", 0, 0)]
#[case("<<em|alfa-bravo>>etc...", "etc...", 0, 0)]
#[case("<<em|charlie -- delta>>etc...", "etc...", 0, 0)]
#[case("<<\nalfa>>", "", 0, 0)]
#[case("<< alfa>>", "", 0, 0)]
#[case("<<alfa\n>>", "", 0, 0)]
#[case("<<alfa >>", "", 0, 0)]
#[case("<<alfa\n|bravo>>", "", 0, 0)]
#[case("<<alfa|\nbravo>>", "", 0, 0)]
#[case("<<alfa|bravo\n>>", "", 0, 0)]
#[case("<<alfa|bravo|charlie>>etc...", "etc...", 1, 0)]
#[case("<<alfa|bravo|charlie-delta>>etc...", "etc...", 1, 0)]
#[case("<<alfa|bravo|charlie--delta>>etc...", "etc...", 1, 0)]
#[case("<<alfa|bravo|charlie_delta>>etc...", "etc...", 1, 0)]
#[case("<<em|Incidentally, if you don't have Dropbox and want to give it a spin, \nyou can [use this referral link][2] to sign up. We'll each get an\nextra 500MB of storage space.>>", "", 0, 0)]
#[case(
    "<<link|text|https://www.example.com/?ping=pong>>etc...",
    "etc...",
    1,
    0
)]
#[case(
    "<<link|text|\nhttps://www.example.com/?ping=pong>>etc...",
    "etc...",
    1,
    0
)]
#[case(
    "<<em|dedicated to <<link|Merlin Mann|https://mastodon.social/@hotdogsladies/>> who put the idea of the clicky sound in my head|class: clicky>>",
    "",
    0,
    1
)]
#[case(
    r#"<<ilink|
I think I'm going to call "YAGINP JS" for "You Ain't Gonna Need
It Philosophy JS"
|2fblbzh9>>"#,
    "",
    1,
    0
)]

// #[case("``alfa``", "", 0, 0)]
// #[case("``alfa bravo``", "", 0, 0)]
// #[case("``alfa     bravo``", "", 0, 0)]
// #[case("``  alfa     bravo  ``", "", 0, 0)]
// #[case("``\nalfa bravo``", "", 0, 0)]
// #[case("``alfa\nbravo``", "", 0, 0)]
// #[case("``alfa <<bravo>>``", "", 0, 0)]
// #[case("``alfa ` bravo``", "", 0, 0)]
// #[case("``alfa \\| bravo``", "", 0, 0)]
// #[case("``alfa~bravo``", "", 0, 0)]
// #[case("``alfa\\|\\|bravo``", "", 0, 0)]
// #[case("``alfa:bravo``", "", 0, 0)]
// #[case("``alfa::bravo``", "", 0, 0)]
// #[case("``alfa~~bravo~~``", "", 0, 0)]
// #[case("``alfa|bravo``", "", 1, 0)]
// #[case("``alfa|https://www.example.com/``", "", 1, 0)]
// #[case("``alfa|  bravo``", "", 1, 0)]
// #[case("``alfa|\nbravo``", "", 1, 0)]
// #[case("``alfa|bravo\n``", "", 1, 0)]
// #[case("``alfa| \n bravo``", "", 1, 0)]
// #[case("``alfa|bravo \n ``", "", 1, 0)]
// #[case("``alfa|bravo\\:charlie``", "", 1, 0)]
// #[case("``alfa|bravo\\|charlie``", "", 1, 0)]
// #[case("``alfa|bravo: charlie``", "", 0, 1)]
// #[case("``alfa|\nbravo: charlie``", "", 0, 1)]
// #[case("``alfa|bravo: charlie\n``", "", 0, 1)]
// #[case("``alfa|  bravo: charlie``", "", 0, 1)]
// #[case("``alfa|bravo: charlie  ``", "", 0, 1)]
// #[case("``alfa|bravo: <<link|ping>>``", "", 0, 1)]

fn solo_named_span_valid_tests(
    #[case] source: &str,
    #[case] remainder: &str,
    #[case] flags: usize,
    #[case] attrs: usize,
) {
    let parsed = named_span(source).unwrap();
    assert_eq!(
        &remainder, &parsed.0,
        "Incorrect remainder. Expected: {} - Got: {}",
        &remainder, &parsed.0
    );
    match &parsed.1 {
        SpanV42::NamedSpan(span) => {
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
        _ => assert!(false, "Did not get Named Span"),
    };
}

#[rstest]
#[case("<<alfa bravo>>", "Can't have spaces in tag names")]
#[case(
    "<<hr:>>",
    "colon not allowed to prevent issues wtih file paths for templates"
)]
#[case(
    "<<hr/>>",
    "forward slaches not allowed to prevent issues wtih file paths for templates"
)]
#[case(
    "<<hr\\>>",
    "back slaches not allowed to prevent issues wtih file paths for templates"
)]
fn solo_named_span_invalid_tests(#[case] source: &str, #[case] description: &str) {
    let parsed = named_span(source);
    if parsed.is_err() {
        assert!(true);
    } else {
        dbg!(&parsed);
        assert!(false, "{}", description);
    }
}

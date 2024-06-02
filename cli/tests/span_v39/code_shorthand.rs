use neopoligengine::site_config::SiteConfig;
use neopoligengine::span_v39::*;
use nom::multi::many1;
use nom::Parser;
use pretty_assertions::assert_eq;

#[test]
fn backtick_at_eof() {
    let source = "`";
    let left = (
        "",
        SpanV39 {
            kind: SpanV39Kind::Backtick {
                source_text: "`".to_string(),
            },
        },
    );
    let right = backtick_v39(source).unwrap();
    assert_eq!(left, right);
}

#[test]
fn backtick_at_infront_of_another_character() {
    let source = "`x";
    let left = (
        "x",
        SpanV39 {
            kind: SpanV39Kind::Backtick {
                source_text: "`".to_string(),
            },
        },
    );
    let right = backtick_v39(source).unwrap();
    assert_eq!(left, right);
}

#[test]
fn backtick_at_infront_of_space() {
    let source = "` ";
    let left = (
        " ",
        SpanV39 {
            kind: SpanV39Kind::Backtick {
                source_text: "`".to_string(),
            },
        },
    );
    let right = backtick_v39(source).unwrap();
    assert_eq!(left, right);
}

#[test]
fn backtick_in_text() {
    let config = SiteConfig::mock1();
    let source = "alfa`bravo";
    let left = (
        "",
        vec![
            SpanV39 {
                kind: SpanV39Kind::WordPart {
                    text: "alfa".to_string(),
                },
            },
            SpanV39 {
                kind: SpanV39Kind::Backtick {
                    source_text: "`".to_string(),
                },
            },
            SpanV39 {
                kind: SpanV39Kind::WordPart {
                    text: "bravo".to_string(),
                },
            },
        ],
    );
    let right = many1(|src| span_v39(src, &config.spans))
        .parse(source)
        .unwrap();
    assert_eq!(left, right);
}

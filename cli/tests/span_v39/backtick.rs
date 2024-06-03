use neopoligengine::site_config::SiteConfig;
use neopoligengine::span_v39::*;
use nom::multi::many1;
use nom::Parser;
use pretty_assertions::assert_eq;

#[test]
fn escaped_backtick() {
    let source = "\\`";
    let left = (
        "",
        SpanV39 {
            attrs: vec![],
            source_text: "\\`".to_string(),
            parsed_text: "`".to_string(),
            kind: SpanV39Kind::EscapedBacktick,
        },
    );
    let right = escaped_backtick_v39(source).unwrap();
    assert_eq!(left, right);
}

#[test]
fn backtick_at_eof() {
    let source = "`";
    let left = (
        "",
        SpanV39 {
            attrs: vec![],
            source_text: "`".to_string(),
            parsed_text: "`".to_string(),
            kind: SpanV39Kind::Backtick,
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
            attrs: vec![],
            source_text: "`".to_string(),
            parsed_text: "`".to_string(),
            kind: SpanV39Kind::Backtick {},
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
            attrs: vec![],
            source_text: "`".to_string(),
            parsed_text: "`".to_string(),
            kind: SpanV39Kind::Backtick {},
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
                attrs: vec![],
                source_text: "alfa".to_string(),
                parsed_text: "alfa".to_string(),
                kind: SpanV39Kind::WordPart {},
            },
            SpanV39 {
                attrs: vec![],
                source_text: "`".to_string(),
                parsed_text: "`".to_string(),
                kind: SpanV39Kind::Backtick,
            },
            SpanV39 {
                attrs: vec![],
                source_text: "bravo".to_string(),
                parsed_text: "bravo".to_string(),
                kind: SpanV39Kind::WordPart {},
            },
        ],
    );
    let right = many1(|src| span_v39(src, &config.spans))
        .parse(source)
        .unwrap();
    assert_eq!(left, right);
}

#[test]
fn escaped_backtick_followed_by_single() {
    let config = SiteConfig::mock1();
    let source = "\\``";
    let left = (
        "",
        vec![
            SpanV39 {
                attrs: vec![],
                source_text: "\\`".to_string(),
                parsed_text: "`".to_string(),
                kind: SpanV39Kind::EscapedBacktick,
            },
            SpanV39 {
                attrs: vec![],
                source_text: "`".to_string(),
                parsed_text: "`".to_string(),
                kind: SpanV39Kind::Backtick,
            },
        ],
    );
    let right = many1(|src| span_v39(src, &config.spans))
        .parse(source)
        .unwrap();
    assert_eq!(left, right);
}

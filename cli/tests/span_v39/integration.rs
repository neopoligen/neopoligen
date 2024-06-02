use neopoligengine::span_v39::{SpanV39, SpanV39Kind};
use neopoligengine::{site_config::SiteConfig, span_v39::span_v39};
use nom::multi::many1;
use nom::Parser;
use pretty_assertions::assert_eq;

#[test]
fn int_1_basic() {
    let config = SiteConfig::mock1();
    let source = "alfa bravo";
    let left = (
        "",
        vec![
            SpanV39 {
                kind: SpanV39Kind::WordPart {
                    text: "alfa".to_string(),
                },
            },
            SpanV39 {
                kind: SpanV39Kind::Space {
                    text: " ".to_string(),
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

#[test]
fn int_2_newline() {
    let config = SiteConfig::mock1();
    let source = "alfa\nbravo";
    let left = (
        "",
        vec![
            SpanV39 {
                kind: SpanV39Kind::WordPart {
                    text: "alfa".to_string(),
                },
            },
            SpanV39 {
                kind: SpanV39Kind::Space {
                    text: " ".to_string(),
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

#[test]
fn int_3_backtick() {
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
                    text: "`".to_string(),
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

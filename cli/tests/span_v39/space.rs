use neopoligengine::site_config::SiteConfig;
use neopoligengine::span_v39::*;
use nom::multi::many1;
use nom::Parser;
use pretty_assertions::assert_eq;

#[test]
fn space_basic() {
    let source = " ";
    let left = (
        "",
        SpanV39 {
            kind: SpanV39Kind::Space {
                source_text: " ".to_string(),
            },
        },
    );
    let right = space_v39(source).unwrap();
    assert_eq!(left, right);
}

#[test]
fn space_in_words() {
    let config = SiteConfig::mock1();
    let source = "alfa bravo";
    let left = (
        "",
        vec![
            SpanV39 {
                kind: SpanV39Kind::WordPart {
                    source_text: "alfa".to_string(),
                },
            },
            SpanV39 {
                kind: SpanV39Kind::Space {
                    source_text: " ".to_string(),
                },
            },
            SpanV39 {
                kind: SpanV39Kind::WordPart {
                    source_text: "bravo".to_string(),
                },
            },
        ],
    );
    let right = many1(|src| span_v39(src, &config.spans))
        .parse(source)
        .unwrap();
    assert_eq!(left, right);
}

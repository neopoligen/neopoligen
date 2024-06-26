use neopoligengine::site_config::SiteConfig;
use neopoligengine::span_v39::*;
use nom::multi::many1;
use nom::Parser;
use pretty_assertions::assert_eq;

#[test]
fn newline_basic() {
    let source = "\n";
    let left = (
        "",
        SpanV39 {
            attrs: vec![],
            source_text: "\n".to_string(),
            parsed_text: "\n".to_string(),
            kind: SpanV39Kind::Newline,
        },
    );
    let right = newline_v39(source).unwrap();
    assert_eq!(left, right);
}

#[test]
fn newline_in_words() {
    let config = SiteConfig::mock1();
    let source = "alfa\nbravo";
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
                source_text: "\n".to_string(),
                parsed_text: "\n".to_string(),
                kind: SpanV39Kind::Newline {},
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

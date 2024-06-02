//use neopoligengine::site_config::SiteConfig;
use neopoligengine::span_v39::*;
// use nom::multi::many1;
// use nom::Parser;
use pretty_assertions::assert_eq;

#[test]
fn word_part_basic() {
    let source = "alfa ";
    let left = (
        " ",
        SpanV39 {
            kind: SpanV39Kind::WordPart {
                source_text: "alfa".to_string(),
            },
        },
    );
    let right = word_part_v39(source).unwrap();
    assert_eq!(left, right);
}

#[test]
fn word_part_not_line_ending() {
    let source = "alfa\n";
    let left = (
        "\n",
        SpanV39 {
            kind: SpanV39Kind::WordPart {
                source_text: "alfa".to_string(),
            },
        },
    );
    let right = word_part_v39(source).unwrap();
    assert_eq!(left, right);
}

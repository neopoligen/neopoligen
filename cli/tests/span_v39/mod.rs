use neopoligengine::{site_config::SiteConfig, span_v39::*};
use pretty_assertions::assert_eq;

#[test]
fn line_ending_or_eof_with_line_ending() {
    let source = "\n";
    let left = ("", "\n");
    let right = line_ending_or_eof(source).unwrap();
    assert_eq!(left, right);
}

#[test]
fn line_ending_or_eof_with_eof() {
    let source = "";
    let left = ("", "");
    let right = line_ending_or_eof(source).unwrap();
    assert_eq!(left, right);
}

#[test]
fn space_basic() {
    let config = SiteConfig::mock1();
    let source = " ";
    let left = (
        "",
        SpanV39 {
            kind: SpanV39Kind::Space {
                text: " ".to_string(),
            },
        },
    );
    let right = span_v39(source, &config.spans).unwrap();
    assert_eq!(left, right);
}

#[test]
fn word_part_basic() {
    let config = SiteConfig::mock1();
    let source = "alfa ";
    let left = (
        " ",
        SpanV39 {
            kind: SpanV39Kind::WordPart {
                text: "alfa".to_string(),
            },
        },
    );
    let right = span_v39(source, &config.spans).unwrap();
    assert_eq!(left, right);
}

#[test]
fn word_part_not_line_ending() {
    let config = SiteConfig::mock1();
    let source = "alfa\n";
    let left = (
        "\n",
        SpanV39 {
            kind: SpanV39Kind::WordPart {
                text: "alfa".to_string(),
            },
        },
    );
    let right = span_v39(source, &config.spans).unwrap();
    assert_eq!(left, right);
}

use neopoligengine::{site_config::SiteConfig, span_v39::*};
use pretty_assertions::assert_eq;

#[test]
fn word_part_basic() {
    let config = SiteConfig::mock1();
    let source = "alfa ";
    let left = (
        " ",
        SpanV39 {
            kind: SpanV39Kind::WordPart { text: "alfa" },
        },
    );
    let right = span_v39(source, &config.spans).unwrap();
    assert_eq!(left, right);
}

#[test]
fn space_basic() {
    let config = SiteConfig::mock1();
    let source = " ";
    let left = (
        "",
        SpanV39 {
            kind: SpanV39Kind::Space { text: " " },
        },
    );
    let right = span_v39(source, &config.spans).unwrap();
    assert_eq!(left, right);
}

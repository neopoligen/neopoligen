use neopoligengine::site_config::SiteConfig;
use neopoligengine::span_v39::*;
use nom::multi::many1;
use nom::Parser;
use pretty_assertions::assert_eq;

#[test]
fn code_shorthand_basic() {
    let source = "``code``";
    let left = (
        "",
        SpanV39 {
            kind: SpanV39Kind::CodeShorthand{
                source_text: "``code``".to_string(),
                parsed_text: "code".to_string(),
            },
        },
    );
    let right = code_shorthand_v39(source).unwrap();
    assert_eq!(left, right);
}

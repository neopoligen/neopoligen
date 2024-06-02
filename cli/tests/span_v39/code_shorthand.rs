// use neopoligengine::site_config::SiteConfig;
use neopoligengine::span_attr_v39::*;
use neopoligengine::span_v39::code_shorthand::*;
use neopoligengine::span_v39::*;
// use nom::multi::many1;
// use nom::Parser;
use pretty_assertions::assert_eq;

#[test]
fn code_shorthand_basic() {
    let source = "``code``";
    let attrs = vec![];
    let left = (
        "",
        SpanV39 {
            kind: SpanV39Kind::CodeShorthand {
                attrs,
                source_text: "``code``".to_string(),
                parsed_text: "code".to_string(),
            },
        },
    );
    let right = code_shorthand_v39(source).unwrap();
    assert_eq!(left, right);
}

#[test]
#[ignore]
fn code_shorthand_with_flag_attr() {
    let source = "``code|rust``";
    let left = (
        "",
        SpanV39 {
            kind: SpanV39Kind::CodeShorthand {
                attrs: vec![SpanAttrV39 {
                    kind: SpanAttrV39Kind::Flag {
                        flag: "rust".to_string(),
                    },
                }],
                source_text: "``code``".to_string(),
                parsed_text: "code".to_string(),
            },
        },
    );
    let right = code_shorthand_v39(source).unwrap();
    assert_eq!(left, right);
}

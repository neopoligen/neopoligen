use neopoligengine::span_attr_v39::*;
use neopoligengine::span_shorthand_token_v39::*;
use neopoligengine::span_v39::code_shorthand::*;
use neopoligengine::span_v39::*;
use pretty_assertions::assert_eq;
// use neopoligengine::site_config::SiteConfig;
// use nom::multi::many1;
// use nom::Parser;

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
fn code_shorthand_with_flag_attr() {
    let source = "``code|rust``";
    let left = (
        "",
        SpanV39 {
            kind: SpanV39Kind::CodeShorthand {
                attrs: vec![SpanAttrV39 {
                    kind: SpanAttrV39Kind::Flag {
                        source_text: "|rust".to_string(),
                        key: "rust".to_string(),
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

#[test]
fn code_shorthand_with_multiple_flag_attrs() {
    let source = "``code|rust|hidden``";
    let left = (
        "",
        SpanV39 {
            kind: SpanV39Kind::CodeShorthand {
                attrs: vec![
                    SpanAttrV39 {
                        kind: SpanAttrV39Kind::Flag {
                            source_text: "|rust".to_string(),
                            key: "rust".to_string(),
                        },
                    },
                    SpanAttrV39 {
                        kind: SpanAttrV39Kind::Flag {
                            source_text: "|hidden".to_string(),
                            key: "hidden".to_string(),
                        },
                    },
                ],
                source_text: "``code``".to_string(),
                parsed_text: "code".to_string(),
            },
        },
    );
    let right = code_shorthand_v39(source).unwrap();
    assert_eq!(left, right);
}

#[test]
fn flag_attr_for_code_basic_end_at_block() {
    let source = "|rust``";
    let left = (
        "``",
        SpanAttrV39 {
            kind: SpanAttrV39Kind::Flag {
                source_text: "|rust".to_string(),
                key: "rust".to_string(),
            },
        },
    );
    let right = code_shorthand_flag_attr_v39(source).unwrap();
    assert_eq!(left, right);
}

#[test]
fn flag_attr_for_code_with_pipe_escape() {
    let source = "|rust\\|here``";
    let left = (
        "``",
        SpanAttrV39 {
            kind: SpanAttrV39Kind::Flag {
                source_text: "|rust\\|here".to_string(),
                key: "rust|here".to_string(),
            },
        },
    );
    let right = code_shorthand_flag_attr_v39_dev(source).unwrap();
    assert_eq!(left, right);
}

#[test]
fn token_for_code_shorthand_word_part() {
    let source = "alfa``";
    let left = (
        "``",
        SpanShorthandTokenV39 {
            source_text: "alfa".to_string(),
            parsed_text: "alfa".to_string(),
            kind: SpanShorthandTokenV39Kind::WordPart,
        },
    );
    let right = code_shorthand_token_word_part_v39(source).unwrap();
    assert_eq!(left, right);
}

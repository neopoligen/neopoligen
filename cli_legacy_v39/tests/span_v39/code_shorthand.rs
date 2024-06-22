use neopoligengine::site_config::SiteConfig;
use neopoligengine::span_attr_v39::*;
use neopoligengine::span_shorthand_token_v39::*;
use neopoligengine::span_v39::code_shorthand::*;
use neopoligengine::span_v39::*;
use nom::multi::many1;
use nom::Parser;
use pretty_assertions::assert_eq;

#[test]
fn code_shorthand_basic() {
    let source = "``ping``";
    let left = (
        "",
        SpanV39 {
            attrs: vec![],
            source_text: "``ping``".to_string(),
            parsed_text: "ping".to_string(),
            kind: SpanV39Kind::CodeShorthand,
        },
    );
    let right = code_shorthand_v39(source).unwrap();
    assert_eq!(left, right);
}

#[test]
fn code_shorthand_with_escaped_pipe() {
    let source = "``ping\\|ping``";
    let left = (
        "",
        SpanV39 {
            attrs: vec![],
            source_text: "``ping\\|ping``".to_string(),
            parsed_text: "ping|ping".to_string(),
            kind: SpanV39Kind::CodeShorthand,
        },
    );
    let right = code_shorthand_v39(source).unwrap();
    assert_eq!(left, right);
}

#[test]
fn code_shorthand_with_escaped_backslash() {
    let source = "``ping\\\\ping``";
    let left = (
        "",
        SpanV39 {
            attrs: vec![],
            source_text: "``ping\\\\ping``".to_string(),
            parsed_text: "ping\\ping".to_string(),
            kind: SpanV39Kind::CodeShorthand,
        },
    );
    let right = code_shorthand_v39(source).unwrap();
    assert_eq!(left, right);
}

#[test]
fn code_shorthand_with_escaped_backtick() {
    let source = "``ping\\`ping``";
    let left = (
        "",
        SpanV39 {
            attrs: vec![],
            source_text: "``ping\\`ping``".to_string(),
            parsed_text: "ping`ping".to_string(),
            kind: SpanV39Kind::CodeShorthand,
        },
    );
    let right = code_shorthand_v39(source).unwrap();
    assert_eq!(left, right);
}

#[test]
fn code_shorthand_with_single_backslash() {
    let source = "``ping\\ping``";
    let left = (
        "",
        SpanV39 {
            attrs: vec![],
            source_text: "``ping\\ping``".to_string(),
            parsed_text: "ping\\ping".to_string(),
            kind: SpanV39Kind::CodeShorthand,
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
            attrs: vec![SpanAttrV39 {
                source_text: "|rust".to_string(),
                kind: SpanAttrV39Kind::Flag {
                    value: "rust".to_string(),
                },
            }],
            source_text: "``code|rust``".to_string(),
            parsed_text: "code".to_string(),
            kind: SpanV39Kind::CodeShorthand {},
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
            attrs: vec![
                SpanAttrV39 {
                    source_text: "|rust".to_string(),
                    kind: SpanAttrV39Kind::Flag {
                        value: "rust".to_string(),
                    },
                },
                SpanAttrV39 {
                    source_text: "|hidden".to_string(),
                    kind: SpanAttrV39Kind::Flag {
                        value: "hidden".to_string(),
                    },
                },
            ],
            source_text: "``code|rust|hidden``".to_string(),
            parsed_text: "code".to_string(),
            kind: SpanV39Kind::CodeShorthand,
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
            source_text: "|rust".to_string(),
            kind: SpanAttrV39Kind::Flag {
                value: "rust".to_string(),
            },
        },
    );
    let right = code_shorthand_flag_attr_v39(source).unwrap();
    assert_eq!(left, right);
}

#[test]
fn flag_attr_for_code_with_escaped_colon() {
    let source = "|rust\\:``";
    let left = (
        "``",
        SpanAttrV39 {
            source_text: "|rust\\:".to_string(),
            kind: SpanAttrV39Kind::Flag {
                value: "rust:".to_string(),
            },
        },
    );
    let right = code_shorthand_flag_attr_v39(source).unwrap();
    assert_eq!(left, right);
}

#[test]
fn flag_attr_for_code_url() {
    let source = "|https://www.example.com``";
    let left = (
        "``",
        SpanAttrV39 {
            source_text: "|https://www.example.com".to_string(),
            kind: SpanAttrV39Kind::Flag {
                value: "https://www.example.com".to_string(),
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
            source_text: "|rust\\|here".to_string(),
            kind: SpanAttrV39Kind::Flag {
                value: "rust|here".to_string(),
            },
        },
    );
    let right = code_shorthand_flag_attr_v39(source).unwrap();
    assert_eq!(left, right);
}

#[test]
fn token_for_code_shorthand_word_part() {
    let source = "alfa``";
    let left = (
        "``",
        SpanShorthandTokenV39 {
            kind: SpanShorthandTokenV39Kind::WordPart,
            parsed_text: "alfa".to_string(),
            source_text: "alfa".to_string(),
        },
    );
    let right = code_shorthand_token_word_part_v39(source).unwrap();
    assert_eq!(left, right);
}

#[test]
fn integration_1_basic() {
    let config = SiteConfig::mock1();
    let source = "alfa ``bravo`` charlie";
    let left = (
        "",
        vec![
            SpanV39 {
                attrs: vec![],
                kind: SpanV39Kind::WordPart,
                parsed_text: "alfa".to_string(),
                source_text: "alfa".to_string(),
            },
            SpanV39 {
                attrs: vec![],
                kind: SpanV39Kind::Space,
                parsed_text: " ".to_string(),
                source_text: " ".to_string(),
            },
            SpanV39 {
                attrs: vec![],
                kind: SpanV39Kind::CodeShorthand,
                parsed_text: "bravo".to_string(),
                source_text: "``bravo``".to_string(),
            },
            SpanV39 {
                attrs: vec![],
                kind: SpanV39Kind::Space,
                parsed_text: " ".to_string(),
                source_text: " ".to_string(),
            },
            SpanV39 {
                attrs: vec![],
                kind: SpanV39Kind::WordPart,
                parsed_text: "charlie".to_string(),
                source_text: "charlie".to_string(),
            },
        ],
    );
    let right = many1(|src| span_v39(src, &config.spans))
        .parse(source)
        .unwrap();
    assert_eq!(left, right);
}

#[test]
fn integration_2_escaped_pipe() {
    let config = SiteConfig::mock1();
    let source = r#"alfa ``bravo\|delta`` charlie"#;
    let left = (
        "",
        vec![
            SpanV39 {
                attrs: vec![],
                kind: SpanV39Kind::WordPart,
                parsed_text: "alfa".to_string(),
                source_text: "alfa".to_string(),
            },
            SpanV39 {
                attrs: vec![],
                kind: SpanV39Kind::Space,
                parsed_text: " ".to_string(),
                source_text: " ".to_string(),
            },
            SpanV39 {
                attrs: vec![],
                kind: SpanV39Kind::CodeShorthand,
                parsed_text: "bravo|delta".to_string(),
                source_text: "``bravo\\|delta``".to_string(),
            },
            SpanV39 {
                attrs: vec![],
                kind: SpanV39Kind::Space,
                parsed_text: " ".to_string(),
                source_text: " ".to_string(),
            },
            SpanV39 {
                attrs: vec![],
                kind: SpanV39Kind::WordPart,
                parsed_text: "charlie".to_string(),
                source_text: "charlie".to_string(),
            },
        ],
    );
    let right = many1(|src| span_v39(src, &config.spans))
        .parse(source)
        .unwrap();
    assert_eq!(left, right);
}

#[test]
fn integration_2_escaped_single_backtick() {
    let config = SiteConfig::mock1();
    let source = r#"alfa ``bravo`delta`` charlie"#;
    let left = (
        "",
        vec![
            SpanV39 {
                attrs: vec![],
                kind: SpanV39Kind::WordPart,
                parsed_text: "alfa".to_string(),
                source_text: "alfa".to_string(),
            },
            SpanV39 {
                attrs: vec![],
                kind: SpanV39Kind::Space,
                parsed_text: " ".to_string(),
                source_text: " ".to_string(),
            },
            SpanV39 {
                attrs: vec![],
                kind: SpanV39Kind::CodeShorthand,
                parsed_text: "bravo`delta".to_string(),
                source_text: "``bravo`delta``".to_string(),
            },
            SpanV39 {
                attrs: vec![],
                kind: SpanV39Kind::Space,
                parsed_text: " ".to_string(),
                source_text: " ".to_string(),
            },
            SpanV39 {
                attrs: vec![],
                kind: SpanV39Kind::WordPart,
                parsed_text: "charlie".to_string(),
                source_text: "charlie".to_string(),
            },
        ],
    );
    let right = many1(|src| span_v39(src, &config.spans))
        .parse(source)
        .unwrap();
    assert_eq!(left, right);
}

#[test]
fn code_shorthand_key_value_attrs_basic_key_value_attr() {
    let source = "|class: green";
    let left = (
        "",
        SpanAttrV39 {
            source_text: "|class: green".to_string(),
            kind: SpanAttrV39Kind::KeyValue {
                key: "class".to_string(),
                value: "green".to_string(),
            },
        },
    );
    let right = code_shorthand_key_value_attr_v39(source).unwrap();
    assert_eq!(left, right);
}

#[test]
fn code_shorthand_basic_with_key_value() {
    let source = "``ping|class: blue``";
    let left = (
        "",
        SpanV39 {
            attrs: vec![SpanAttrV39 {
                source_text: "|class: blue".to_string(),
                kind: SpanAttrV39Kind::KeyValue {
                    key: "class".to_string(),
                    value: "blue".to_string(),
                },
            }],
            source_text: "``ping|class: blue``".to_string(),
            parsed_text: "ping".to_string(),
            kind: SpanV39Kind::CodeShorthand {},
        },
    );
    let right = code_shorthand_v39(source).unwrap();
    assert_eq!(left, right);
}

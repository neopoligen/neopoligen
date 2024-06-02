use neopoligengine::section_v39::basic::*;
use neopoligengine::{
    section_attr_v39::{SectionAttrV39, SectionAttrV39Kind},
    section_v39::*,
    site_config::SiteConfig,
    span_v39::{SpanV39, SpanV39Kind},
};
use pretty_assertions::assert_eq;

#[test]
fn basic_section_full_core_test() {
    let config = SiteConfig::mock1();
    let source = "-- title\n\nHello World";
    let left = (
        "",
        SectionV39 {
            attrs: vec![],
            bounds: SectionV39Bounds::Full,
            kind: SectionV39Kind::Basic {
                children: vec![SectionV39 {
                    attrs: vec![],
                    bounds: SectionV39Bounds::Full,
                    kind: SectionV39Kind::Block {
                        spans: vec![
                            SpanV39 {
                                attrs: vec![],
                                source_text: "Hello".to_string(),
                                parsed_text: "Hello".to_string(),
                                kind: SpanV39Kind::WordPart,
                            },
                            SpanV39 {
                                attrs: vec![],
                                source_text: " ".to_string(),
                                parsed_text: " ".to_string(),
                                kind: SpanV39Kind::Space,
                            },
                            SpanV39 {
                                attrs: vec![],
                                source_text: "World".to_string(),
                                parsed_text: "World".to_string(),
                                kind: SpanV39Kind::WordPart,
                            },
                        ],
                    },
                    r#type: "block-of-text".to_string(),
                }],
            },
            r#type: "title".to_string(),
        },
    );
    let right = basic_section_full_v39(source, &config.sections, &config.spans).unwrap();
    assert_eq!(left, right);
}

#[test]
fn basic_section_full_core_test_with_following_section() {
    let config = SiteConfig::mock1();
    let source = "-- title\n\nHello World\n\n-- div";
    let left = (
        "-- div",
        SectionV39 {
            attrs: vec![],
            bounds: SectionV39Bounds::Full,
            kind: SectionV39Kind::Basic {
                children: vec![SectionV39 {
                    attrs: vec![],
                    bounds: SectionV39Bounds::Full,
                    kind: SectionV39Kind::Block {
                        spans: vec![
                            SpanV39 {
                                attrs: vec![],
                                source_text: "Hello".to_string(),
                                parsed_text: "Hello".to_string(),
                                kind: SpanV39Kind::WordPart,
                            },
                            SpanV39 {
                                attrs: vec![],
                                source_text: " ".to_string(),
                                parsed_text: " ".to_string(),
                                kind: SpanV39Kind::Space,
                            },
                            SpanV39 {
                                attrs: vec![],
                                source_text: "World".to_string(),
                                parsed_text: "World".to_string(),
                                kind: SpanV39Kind::WordPart,
                            },
                        ],
                    },
                    r#type: "block-of-text".to_string(),
                }],
            },
            r#type: "title".to_string(),
        },
    );
    let right = basic_section_full_v39(source, &config.sections, &config.spans).unwrap();
    assert_eq!(left, right);
}

#[test]
fn basic_section_full_with_attrs() {
    let config = SiteConfig::mock1();
    let source = "-- div\n-- class: green\n\nalfa bravo";
    let left = (
        "",
        SectionV39 {
            attrs: vec![SectionAttrV39 {
                kind: SectionAttrV39Kind::KeyValue {
                    key: "class".to_string(),
                    value: "green".to_string(),
                },
            }],
            bounds: SectionV39Bounds::Full,
            kind: SectionV39Kind::Basic {
                children: vec![SectionV39 {
                    attrs: vec![],
                    bounds: SectionV39Bounds::Full,
                    kind: SectionV39Kind::Block {
                        spans: vec![
                            SpanV39 {
                                attrs: vec![],
                                source_text: "alfa".to_string(),
                                parsed_text: "alfa".to_string(),
                                kind: SpanV39Kind::WordPart,
                            },
                            SpanV39 {
                                attrs: vec![],
                                source_text: " ".to_string(),
                                parsed_text: " ".to_string(),
                                kind: SpanV39Kind::Space,
                            },
                            SpanV39 {
                                attrs: vec![],
                                source_text: "bravo".to_string(),
                                parsed_text: "bravo".to_string(),
                                kind: SpanV39Kind::WordPart,
                            },
                        ],
                    },
                    r#type: "block-of-text".to_string(),
                }],
            },
            r#type: "div".to_string(),
        },
    );
    let right = start_or_full_section_v39(source, &config.sections, &config.spans).unwrap();
    assert_eq!(left, right);
}

#[test]
fn basic_section_start_end_core_test() {
    let config = SiteConfig::mock1();
    let source = "-- note/\n\nHello World\n\n-- /note";
    let left = (
        "",
        SectionV39 {
            attrs: vec![],
            bounds: SectionV39Bounds::Start,
            kind: SectionV39Kind::Basic {
                children: vec![
                    SectionV39 {
                        attrs: vec![],
                        bounds: SectionV39Bounds::Full,
                        kind: SectionV39Kind::Block {
                            spans: vec![
                                SpanV39 {
                                    attrs: vec![],
                                    source_text: "Hello".to_string(),
                                    parsed_text: "Hello".to_string(),
                                    kind: SpanV39Kind::WordPart {},
                                },
                                SpanV39 {
                                    attrs: vec![],
                                    source_text: " ".to_string(),
                                    parsed_text: " ".to_string(),
                                    kind: SpanV39Kind::Space {},
                                },
                                SpanV39 {
                                    attrs: vec![],
                                    source_text: "World".to_string(),
                                    parsed_text: "World".to_string(),
                                    kind: SpanV39Kind::WordPart {},
                                },
                            ],
                        },
                        r#type: "block-of-text".to_string(),
                    },
                    SectionV39 {
                        attrs: vec![],
                        bounds: SectionV39Bounds::End,
                        kind: SectionV39Kind::Basic { children: vec![] },
                        r#type: "note".to_string(),
                    },
                ],
            },
            r#type: "note".to_string(),
        },
    );
    let right = basic_section_start_v39(source, &config.sections, &config.spans).unwrap();
    assert_eq!(left, right);
}

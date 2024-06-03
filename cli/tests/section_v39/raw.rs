use neopoligengine::section_v39::raw::*;
use neopoligengine::span_v39::{SpanV39, SpanV39Kind};
use neopoligengine::{
    section_attr_v39::{SectionAttrV39, SectionAttrV39Kind},
    section_v39::*,
    site_config::SiteConfig,
};
use pretty_assertions::assert_eq;

#[test]
fn raw_section_full_core_test_no_attrs() {
    let config = SiteConfig::mock1();
    let source = "-- code\n\nHello World";
    let left = (
        "",
        SectionV39 {
            attrs: vec![],
            bounds: SectionV39Bounds::Full,
            details: None,
            kind: SectionV39Kind::Raw {
                children: vec![],
                text: Some("Hello World".to_string()),
            },
            r#type: "code".to_string(),
        },
    );
    let right = raw_section_full_v39(source, &config.sections, &config.spans).unwrap();
    assert_eq!(left, right);
}

#[test]
fn raw_section_full_core_test_with_attrs() {
    let config = SiteConfig::mock1();
    let source = "-- code\n-- class: green\n\nHello World";
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
            details: None,
            kind: SectionV39Kind::Raw {
                children: vec![],
                text: Some("Hello World".to_string()),
            },
            r#type: "code".to_string(),
        },
    );
    let right = raw_section_full_v39(source, &config.sections, &config.spans).unwrap();
    assert_eq!(left, right);
}

#[test]
fn raw_section_start_no_attrs() {
    let config = SiteConfig::mock1();
    let source = "-- code/\n\nHello World\n\n-- /code";
    let left = (
        "",
        SectionV39 {
            attrs: vec![],
            bounds: SectionV39Bounds::Start,
            details: None,
            kind: SectionV39Kind::Raw {
                children: vec![SectionV39 {
                    attrs: vec![],
                    bounds: SectionV39Bounds::End,
                    details: None,
                    kind: SectionV39Kind::Basic { children: vec![] },
                    r#type: "code".to_string(),
                }],
                text: Some("Hello World".to_string()),
            },
            r#type: "code".to_string(),
        },
    );
    let right = raw_section_start_v39(source, &config.sections, &config.spans).unwrap();
    assert_eq!(left, right);
}

#[test]
fn raw_section_start_no_attrs_with_more_stuff() {
    let config = SiteConfig::mock1();
    let source = "-- code/\n\nHello World\n\n-- /code\n\nmore lines";
    let left = (
        "",
        SectionV39 {
            attrs: vec![],
            bounds: SectionV39Bounds::Start,
            details: None,
            kind: SectionV39Kind::Raw {
                children: vec![SectionV39 {
                    attrs: vec![],
                    bounds: SectionV39Bounds::End,
                    details: None,
                    kind: SectionV39Kind::Basic {
                        children: vec![SectionV39 {
                            attrs: vec![],
                            bounds: SectionV39Bounds::Full,
                            details: None,
                            kind: SectionV39Kind::Block {
                                spans: vec![
                                    SpanV39 {
                                        attrs: vec![],
                                        source_text: "more".to_string(),
                                        parsed_text: "more".to_string(),
                                        kind: SpanV39Kind::WordPart,
                                    },
                                    SpanV39 {
                                        attrs: vec![],
                                        source_text: " ".to_string(),
                                        parsed_text: " ".to_string(),
                                        kind: SpanV39Kind::Space {},
                                    },
                                    SpanV39 {
                                        attrs: vec![],
                                        source_text: "lines".to_string(),
                                        parsed_text: "lines".to_string(),
                                        kind: SpanV39Kind::WordPart {},
                                    },
                                ],
                            },
                            r#type: "block-of-text".to_string(),
                        }],
                    },
                    r#type: "code".to_string(),
                }],
                text: Some("Hello World".to_string()),
            },
            r#type: "code".to_string(),
        },
    );
    let right = raw_section_start_v39(source, &config.sections, &config.spans).unwrap();
    assert_eq!(left, right);
}

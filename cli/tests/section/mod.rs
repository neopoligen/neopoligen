use neopoligengine::block::*;
use neopoligengine::section::*;
use neopoligengine::section_attr::SectionAttr;
use neopoligengine::site_config::SiteConfig;
use neopoligengine::span::*;
use pretty_assertions::assert_eq;
use serde_json::Value;

#[test]
fn basic_section() {
    let source = "-- p\n\nyankee romeo";
    let config = SiteConfig::mock1();
    let left = Section::Basic {
        attrs: vec![],
        bounds: SectionBounds::Full,
        content: vec![Block::Paragraph {
            spans: vec![
                Span::WordPart {
                    text: "yankee".to_string(),
                },
                Span::Space {
                    text: " ".to_string(),
                },
                Span::WordPart {
                    text: "romeo".to_string(),
                },
            ],
        }],
        source: "-- p\n\nyankee romeo".to_string(),
        r#type: "p".to_string(),
    };
    let right = section(&source, &config.sections).unwrap().1;
    assert_eq!(left, right);
}

#[test]
fn empty_section_no_attrs() {
    let source = "-- p\n\n";
    let config = SiteConfig::mock1();
    let left = Section::Basic {
        attrs: vec![],
        bounds: SectionBounds::Full,
        content: vec![],
        source: "-- p\n\n".to_string(),
        r#type: "p".to_string(),
    };
    let right = section(source, &config.sections).unwrap().1;
    assert_eq!(left, right);
}

#[test]
fn empty_section_no_attrs_only_one_newline() {
    let source = "-- p\n";
    let config = SiteConfig::mock1();
    let left = Section::Basic {
        attrs: vec![],
        bounds: SectionBounds::Full,
        content: vec![],
        source: "-- p\n".to_string(),
        r#type: "p".to_string(),
    };
    let right = section(source, &config.sections).unwrap().1;
    assert_eq!(left, right);
}

#[test]
fn multiple_paragraphs() {
    let source = "-- div\n\nsierra tango\n\nindia lima\n\n";
    let config = SiteConfig::mock1();
    let left = Section::Basic {
        attrs: vec![],
        bounds: SectionBounds::Full,
        content: vec![
            Block::Paragraph {
                spans: vec![
                    Span::WordPart {
                        text: "sierra".to_string(),
                    },
                    Span::Space {
                        text: " ".to_string(),
                    },
                    Span::WordPart {
                        text: "tango".to_string(),
                    },
                ],
            },
            Block::Paragraph {
                spans: vec![
                    Span::WordPart {
                        text: "india".to_string(),
                    },
                    Span::Space {
                        text: " ".to_string(),
                    },
                    Span::WordPart {
                        text: "lima".to_string(),
                    },
                ],
            },
        ],
        source: "-- div\n\nsierra tango\n\nindia lima\n\n".to_string(),
        r#type: "div".to_string(),
    };
    let right = section(source, &config.sections).unwrap().1;
    assert_eq!(left, right);
}

#[test]
fn flag_attribute() {
    let source = "-- title\n-- flag-attr\n\nhotel papa";
    let config = SiteConfig::mock1();
    let left = Section::Basic {
        attrs: vec![SectionAttr::Flag {
            key: "flag-attr".to_string(),
        }],
        bounds: SectionBounds::Full,
        content: vec![Block::Paragraph {
            spans: vec![
                Span::WordPart {
                    text: "hotel".to_string(),
                },
                Span::Space {
                    text: " ".to_string(),
                },
                Span::WordPart {
                    text: "papa".to_string(),
                },
            ],
        }],
        source: "-- title\n-- flag-attr\n\nhotel papa".to_string(),
        r#type: "title".to_string(),
    };
    let right = section(source, &config.sections).unwrap().1;
    assert_eq!(left, right);
}

#[test]
fn kv_attr_test() {
    let source = "-- title\n-- key: value\n\nhotel papa";
    let config = SiteConfig::mock1();
    let left = Section::Basic {
        attrs: vec![SectionAttr::KeyValue {
            key: "key".to_string(),
            value: "value".to_string(),
        }],
        bounds: SectionBounds::Full,
        content: vec![Block::Paragraph {
            spans: vec![
                Span::WordPart {
                    text: "hotel".to_string(),
                },
                Span::Space {
                    text: " ".to_string(),
                },
                Span::WordPart {
                    text: "papa".to_string(),
                },
            ],
        }],
        source: "-- title\n-- key: value\n\nhotel papa".to_string(),
        r#type: "title".to_string(),
    };
    let right = section(source, &config.sections).unwrap().1;
    assert_eq!(left, right);
}

#[test]
fn json_section_without_data() {
    let source = "-- metadata\n-- id: someid";
    let config = SiteConfig::mock1();
    let left = Section::Json {
        attrs: vec![SectionAttr::KeyValue {
            key: "id".to_string(),
            value: "someid".to_string(),
        }],
        bounds: SectionBounds::Full,
        source: "-- metadata\n-- id: someid".to_string(),
        data: None,
        r#type: "metadata".to_string(),
    };
    let right = section(source, &config.sections).unwrap().1;
    assert_eq!(left, right);
}

#[test]
fn json_section_with_data() {
    let source = r#"-- metadata
-- id: someid

{ "echo": "delta" }

-- p"#;
    let config = SiteConfig::mock1();
    let left = Section::Json {
        attrs: vec![SectionAttr::KeyValue {
            key: "id".to_string(),
            value: "someid".to_string(),
        }],
        bounds: SectionBounds::Full,
        source: "-- metadata\n-- id: someid\n\n{ \"echo\": \"delta\" }\n\n".to_string(),
        data: Some(serde_json::from_str::<Value>(r#"{ "echo": "delta" }"#).unwrap()),
        r#type: "metadata".to_string(),
    };
    let right = section(source, &config.sections).unwrap().1;
    assert_eq!(left, right);
}

#[test]
fn basic_list() {
    let source = "-- list\n\n- alfa\n\n- bravo";
    let config = SiteConfig::mock1();
    let left = Section::List {
        attrs: vec![],
        bounds: SectionBounds::Full,
        items: vec![],
        source: "-- list\n\n- alfa\n\n- bravo".to_string(),
        r#type: "list".to_string(),
    };
    let right = section(source, &config.sections).unwrap().1;
    assert_eq!(left, right);
}

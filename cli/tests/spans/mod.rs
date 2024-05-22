use neopoligengine::{site_config::SiteConfig, span::*};
use nom::multi::many1;
use pretty_assertions::assert_eq;
use std::collections::BTreeMap;

#[test]
fn basic_word() {
    let config = SiteConfig::mock1();
    let source = "alfa";
    let left = vec![Span::WordPart {
        text: "alfa".to_string(),
        r#type: "wordpart".to_string(),
    }];
    let right = many1(|src| span_finder(src, &config.spans))(source)
        .unwrap()
        .1;
    assert_eq!(left, right);
}

#[test]
fn two_words() {
    let config = SiteConfig::mock1();
    let source = "alfa bravo";
    let left = vec![
        Span::WordPart {
            text: "alfa".to_string(),
            r#type: "wordpart".to_string(),
        },
        Span::Space {
            text: " ".to_string(),
            r#type: "space".to_string(),
        },
        Span::WordPart {
            text: "bravo".to_string(),
            r#type: "wordpart".to_string(),
        },
    ];
    let right = many1(|src| span_finder(src, &config.spans))(source)
        .unwrap()
        .1;
    assert_eq!(left, right);
}

#[test]
fn footnote() {
    let config = SiteConfig::mock1();
    let source = "^^1^^";
    let left = vec![Span::Footnote {
        attrs: BTreeMap::new(),
        flags: vec![],
        text: "1".to_string(),
        r#type: "footnote".to_string(),
    }];
    let right = many1(|src| span_finder(src, &config.spans))(source)
        .unwrap()
        .1;
    assert_eq!(left, right);
}

#[test]
fn footnote_connected_to_word() {
    let config = SiteConfig::mock1();
    let source = "alfa^^1^^";
    let left = vec![
        Span::WordPart {
            text: "alfa".to_string(),
            r#type: "wordpart".to_string(),
        },
        Span::Footnote {
            attrs: BTreeMap::new(),
            flags: vec![],
            text: "1".to_string(),
            r#type: "footnote".to_string(),
        },
    ];
    let right = many1(|src| span_finder(src, &config.spans))(source)
        .unwrap()
        .1;
    assert_eq!(left, right);
}

#[test]
fn link_basic() {
    let config = SiteConfig::mock1();
    let source = "<<link|text|https://www.example.com/>>";
    let left = vec![Span::KnownSpan {
        attrs: BTreeMap::new(),
        flags: vec!["https://www.example.com/".to_string()],
        spans: vec![Span::WordPart {
            text: "text".to_string(),
            r#type: "wordpart".to_string(),
        }],
        r#type: "link".to_string(),
    }];
    let right = many1(|src| span_finder(src, &config.spans))(source)
        .unwrap()
        .1;
    assert_eq!(left, right);
}

#[test]
fn link_shorthand() {
    let config = SiteConfig::mock1();
    let source = "[[text|https://www.example.com/]]";
    let left = vec![Span::KnownSpan {
        attrs: BTreeMap::new(),
        flags: vec!["https://www.example.com/".to_string()],
        spans: vec![Span::WordPart {
            text: "text".to_string(),
            r#type: "wordpart".to_string(),
        }],
        r#type: "link".to_string(),
    }];
    let right = many1(|src| span_finder(src, &config.spans))(source)
        .unwrap()
        .1;
    assert_eq!(left, right);
}

#[test]
fn em_basic_span() {
    let config = SiteConfig::mock1();
    let source = "of <<em|1s>> and 0s.";
    let left = vec![
        Span::WordPart {
            text: "of".to_string(),
            r#type: "wordpart".to_string(),
        },
        Span::Space {
            text: " ".to_string(),
            r#type: "space".to_string(),
        },
        Span::KnownSpan {
            attrs: BTreeMap::new(),
            flags: vec![],
            spans: vec![Span::WordPart {
                text: "1s".to_string(),
                r#type: "wordpart".to_string(),
            }],
            r#type: "em".to_string(),
        },
        Span::Space {
            text: " ".to_string(),
            r#type: "space".to_string(),
        },
        Span::WordPart {
            text: "and".to_string(),
            r#type: "wordpart".to_string(),
        },
        Span::Space {
            text: " ".to_string(),
            r#type: "space".to_string(),
        },
        Span::WordPart {
            text: "0s.".to_string(),
            r#type: "wordpart".to_string(),
        },
    ];
    let right = many1(|src| span_finder(src, &config.spans))(source)
        .unwrap()
        .1;
    assert_eq!(left, right);
}

#[test]
fn open_brace() {
    let config = SiteConfig::mock1();
    let source = "[x";
    let left = vec![
        Span::RawText {
            text: "[".to_string(),
            r#type: "open-brace".to_string(),
        },
        Span::WordPart {
            text: "x".to_string(),
            r#type: "wordpart".to_string(),
        },
    ];
    let right = many1(|src| span_finder(src, &config.spans))(source)
        .unwrap()
        .1;
    assert_eq!(left, right);
}

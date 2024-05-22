use neopoligengine::section::Section;
use neopoligengine::{block::block_of_anything, site_config::SiteConfig, span::*};
use nom::Parser;
use nom_supreme::ParserExt;
use pretty_assertions::assert_eq;
use std::collections::BTreeMap;

#[test]
fn block_with_em() {
    let config = SiteConfig::mock1();
    let source = "of <<em|1s>> and 0s.\n\n";
    let left = Section::Block {
        bounds: "full".to_string(),
        spans: vec![
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
        ],
        r#type: "basic-block".to_string(),
    };
    let right = (|src| block_of_anything(src, &config.spans))(source)
        .unwrap()
        .1;
    assert_eq!(left, right);
}

#[test]
fn solo_block_with_multiple_ems() {
    let config = SiteConfig::mock1();
    let source = "<<em|1s>> and <<em|0s>>.\n\n";
    let left = Section::Block {
        bounds: "full".to_string(),
        spans: vec![
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
            Span::KnownSpan {
                attrs: BTreeMap::new(),
                flags: vec![],
                spans: vec![Span::WordPart {
                    text: "0s".to_string(),
                    r#type: "wordpart".to_string(),
                }],
                r#type: "em".to_string(),
            },
            Span::WordPart {
                text: ".".to_string(),
                r#type: "wordpart".to_string(),
            },
        ],
        r#type: "basic-block".to_string(),
    };
    let right = (|src| block_of_anything(src, &config.spans))
        .context("")
        .parse(source)
        .unwrap()
        .1;
    assert_eq!(left, right);
}

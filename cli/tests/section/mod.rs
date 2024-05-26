use neopoligengine::section::basic::basic_section_full;
use neopoligengine::section::Section;
use neopoligengine::{site_config::SiteConfig, span::*};
use nom::Parser;
use nom_supreme::ParserExt;
use pretty_assertions::assert_eq;
use std::collections::BTreeMap;

#[test]
#[ignore]
fn section_with_em() {
    let config = SiteConfig::mock1();
    let source =
        "-- warning\n\nThis is still a draft\n\nof <<em|1s>> and 0s.\n\nThis is still a draft\n\n";
    let left = Section::Basic {
        attrs: BTreeMap::new(),
        flags: vec![],
        bounds: "full".to_string(),
        children: vec![Section::Block {
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
        }],
        r#type: "warning".to_string(),
    };
    let right = (|src| basic_section_full(src, &config.sections, &config.spans))
        .context("")
        .parse(source)
        .unwrap()
        .1;
    assert_eq!(left, right);
}

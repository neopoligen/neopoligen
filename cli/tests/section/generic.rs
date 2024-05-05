use neopoligengine::block::*;
use neopoligengine::section::*;
use neopoligengine::section_attr::SectionAttr;
use neopoligengine::site_config::SiteConfig;
use neopoligengine::span::*;
use pretty_assertions::assert_eq;

#[test]
fn generic_section() {
    let source = "-- unknown-tag\n\njuliet tango";
    let config = SiteConfig::mock1();
    let left = Section::Basic {
        attrs: vec![],
        bounds: SectionBounds::Full,
        content: vec![Block::Paragraph {
            spans: vec![
                Span::WordPart {
                    text: "juliet".to_string(),
                },
                Span::Space {
                    text: " ".to_string(),
                },
                Span::WordPart {
                    text: "tango".to_string(),
                },
            ],
        }],
        source: "-- unknown-tag\n\njuliet tango".to_string(),
        r#type: "unknown-tag".to_string(),
    };
    let right = section(&source, &config.sections).unwrap().1;
    assert_eq!(left, right);
}

use neopoligengine::block::*;
use neopoligengine::section::*;
use neopoligengine::section_attr::SectionAttr;
use neopoligengine::site_config::SiteConfig;
use neopoligengine::span::*;
use pretty_assertions::assert_eq;
use serde_json::Value;

#[test]
fn basic_list() {
    let source = "-- list\n\n- alfa\n\n- bravo";
    let config = SiteConfig::mock1();
    let left = Section::List {
        attrs: vec![],
        bounds: SectionBounds::Full,
        items: vec![
            Section::ListItem {
                bounds: SectionBounds::Full,
                content: vec![Block::Paragraph {
                    spans: vec![Span::WordPart {
                        text: "alfa".to_string(),
                    }],
                }],
                source: "- alfa\n\n".to_string(),
                r#type: "list_item".to_string(),
            },
            Section::ListItem {
                bounds: SectionBounds::Full,
                content: vec![Block::Paragraph {
                    spans: vec![Span::WordPart {
                        text: "bravo".to_string(),
                    }],
                }],
                source: "- bravo".to_string(),
                r#type: "list_item".to_string(),
            },
        ],
        source: "-- list\n\n- alfa\n\n- bravo".to_string(),
        r#type: "list".to_string(),
    };
    let right = section(source, &config.sections).unwrap().1;
    assert_eq!(left, right);
}

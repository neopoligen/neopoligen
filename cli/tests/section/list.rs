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

#[test]
fn empty_section() {
    let source = "-- list\n\n\n\n\n";
    let config = SiteConfig::mock1();
    let left = Section::List {
        attrs: vec![],
        bounds: SectionBounds::Full,
        items: vec![],
        source: "-- list\n\n\n\n\n".to_string(),
        r#type: "list".to_string(),
    };
    let right = section(source, &config.sections).unwrap().1;
    assert_eq!(left, right);
}

#[test]
fn start_list() {
    let source = "-- list/\n\n- whiskey\n\n- tango";
    let config = SiteConfig::mock1();
    let left = Section::List {
        attrs: vec![],
        bounds: SectionBounds::Start,
        items: vec![
            Section::ListItem {
                bounds: SectionBounds::Full,
                content: vec![Block::Paragraph {
                    spans: vec![Span::WordPart {
                        text: "whiskey".to_string(),
                    }],
                }],
                source: "- whiskey\n\n".to_string(),
                r#type: "list_item".to_string(),
            },
            Section::ListItem {
                bounds: SectionBounds::Full,
                content: vec![Block::Paragraph {
                    spans: vec![Span::WordPart {
                        text: "tango".to_string(),
                    }],
                }],
                source: "- tango".to_string(),
                r#type: "list_item".to_string(),
            },
        ],
        source: "-- list/\n\n- whiskey\n\n- tango".to_string(),
        r#type: "list".to_string(),
    };
    let right = section(source, &config.sections).unwrap().1;
    assert_eq!(left, right);
}

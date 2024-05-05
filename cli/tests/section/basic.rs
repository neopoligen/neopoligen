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

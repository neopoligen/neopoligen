use neopoligengine::block::*;
use neopoligengine::section::*;
use neopoligengine::section_attr::SectionAttr;
use neopoligengine::site_config::SiteConfig;
use neopoligengine::span::*;
use pretty_assertions::assert_eq;

#[test]
fn basic_start_section() {
    let source = "-- div/\n\noscar sierra\n\n-- /div";
    let config = SiteConfig::mock1();
    let left = Section::Basic {
        attrs: vec![],
        bounds: SectionBounds::Start,
        content: vec![Block::Paragraph {
            spans: vec![
                Span::WordPart {
                    text: "oscar".to_string(),
                },
                Span::Space {
                    text: " ".to_string(),
                },
                Span::WordPart {
                    text: "sierra".to_string(),
                },
            ],
        }],
        source: "-- div/\n\noscar sierra\n\n".to_string(),
        r#type: "div".to_string(),
    };
    let right = section(&source, &config.sections).unwrap().1;
    assert_eq!(left, right);
}

#[test]
fn basic_end_section() {
    let source = "-- /div\n\nhotel delta\n\n";
    let config = SiteConfig::mock1();
    let left = Section::Basic {
        attrs: vec![],
        bounds: SectionBounds::End,
        content: vec![Block::Paragraph {
            spans: vec![
                Span::WordPart {
                    text: "hotel".to_string(),
                },
                Span::Space {
                    text: " ".to_string(),
                },
                Span::WordPart {
                    text: "delta".to_string(),
                },
            ],
        }],
        source: "-- /div\n\nhotel delta\n\n".to_string(),
        r#type: "div".to_string(),
    };
    let right = section(&source, &config.sections).unwrap().1;
    assert_eq!(left, right);
}

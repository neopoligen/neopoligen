use neopoligengine::ast::*;
use neopoligengine::block::*;
use neopoligengine::section::*;
use neopoligengine::site_sections::*;
use neopoligengine::span::*;
use pretty_assertions::assert_eq;

#[test]
fn basic_page_parse() {
    let source = "-- title\n\necho foxtrot";
    let sections = SiteSections::mock1();
    let left = vec![Section::Basic {
        attrs: vec![],
        content: vec![Block::Paragraph {
            spans: vec![
                Span::WordPart {
                    text: "echo".to_string(),
                },
                Span::Space {
                    text: " ".to_string(),
                },
                Span::WordPart {
                    text: "foxtrot".to_string(),
                },
            ],
        }],
        source: "-- title\n\necho foxtrot".to_string(),
        r#type: "title".to_string(),
    }];
    let right = ast(source, &sections).unwrap().1;
    assert_eq!(left, right);
}

#[test]
fn multiple_paragraphs() {
    let source = "-- p\n\nwhiskey tango\n\npapa juliet";
    let sections = SiteSections::mock1();
    let left = vec![Section::Basic {
        attrs: vec![],
        content: vec![
            Block::Paragraph {
                spans: vec![
                    Span::WordPart {
                        text: "whiskey".to_string(),
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
                        text: "papa".to_string(),
                    },
                    Span::Space {
                        text: " ".to_string(),
                    },
                    Span::WordPart {
                        text: "juliet".to_string(),
                    },
                ],
            },
        ],
        source: "-- p\n\nwhiskey tango\n\npapa juliet".to_string(),
        r#type: "p".to_string(),
    }];
    let right = ast(source, &sections).unwrap().1;
    assert_eq!(left, right);
}

#[test]
fn multiple_sections() {
    let source = "-- note\n\ntango echo\n\n-- div\n\ndelta alfa";
    let sections = SiteSections::mock1();
    let left = vec![
        Section::Basic {
            attrs: vec![],
            content: vec![Block::Paragraph {
                spans: vec![
                    Span::WordPart {
                        text: "tango".to_string(),
                    },
                    Span::Space {
                        text: " ".to_string(),
                    },
                    Span::WordPart {
                        text: "echo".to_string(),
                    },
                ],
            }],
            source: "-- note\n\ntango echo\n\n".to_string(),
            r#type: "note".to_string(),
        },
        Section::Basic {
            attrs: vec![],
            content: vec![Block::Paragraph {
                spans: vec![
                    Span::WordPart {
                        text: "delta".to_string(),
                    },
                    Span::Space {
                        text: " ".to_string(),
                    },
                    Span::WordPart {
                        text: "alfa".to_string(),
                    },
                ],
            }],
            source: "-- div\n\ndelta alfa".to_string(),
            r#type: "div".to_string(),
        },
    ];
    let right = ast(source, &sections).unwrap().1;
    assert_eq!(left, right);
}

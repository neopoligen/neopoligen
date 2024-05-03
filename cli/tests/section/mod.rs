use neopoligengine::block::*;
use neopoligengine::section::*;
use neopoligengine::span::*;
use pretty_assertions::assert_eq;

#[test]
fn basic_section() {
    let source = "-- p\n\nyankee romeo";
    let left = Section::Standard {
        attrs: vec![],
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
        r#type: "p".to_string(),
    };
    let right = section(source).unwrap().1;
    assert_eq!(left, right);
}

#[test]
fn multiple_paragraphs() {
    let source = "-- div\n\nsierra tango\n\nindia lima\n\n";
    let left = Section::Standard {
        attrs: vec![],
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
        r#type: "div".to_string(),
    };
    let right = section(source).unwrap().1;
    assert_eq!(left, right);
}

#[test]
fn flag_attribute() {
    let source = "-- title\n-- flag-attr\n\nhotel papa";
    let left = Section::Standard {
        attrs: vec![],
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
        r#type: "title".to_string(),
    };
    let right = section(source).unwrap().1;
    assert_eq!(left, right);
}

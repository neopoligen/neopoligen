use neopoligengine::block::*;
use neopoligengine::page::*;
use neopoligengine::section::*;
use neopoligengine::span::*;
use pretty_assertions::assert_eq;

#[test]
fn basic_page_parse() {
    let source = "-- title\n\necho foxtrot";
    let left = vec![Section::Standard {
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
        r#type: "title".to_string(),
    }];
    let right = page(source).unwrap().1;
    assert_eq!(left, right);
}

#[test]
fn multiple_paragraphs() {
    let source = "-- p\n\nwhiskey tango\n\npapa juliet";
    let left = vec![Section::Standard {
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
        r#type: "p".to_string(),
    }];
    let right = page(source).unwrap().1;
    assert_eq!(left, right);
}

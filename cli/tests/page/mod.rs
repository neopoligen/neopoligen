use neopoligengine::block::*;
use neopoligengine::page::*;
use neopoligengine::section::*;
use neopoligengine::span::*;
use pretty_assertions::assert_eq;

#[test]
fn basic_page_parse() {
    let source = r#"-- title

echo foxtrot"#;
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

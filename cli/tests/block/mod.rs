use neopoligengine::block::*;
use neopoligengine::span::*;
use pretty_assertions::assert_eq;

#[test]
fn basic_block() {
    let source = "alfa bravo";
    let left = Block::Paragraph {
        spans: vec![
            Span::WordPart {
                text: "alfa".to_string(),
            },
            Span::Space {
                text: " ".to_string(),
            },
            Span::WordPart {
                text: "bravo".to_string(),
            },
        ],
    };
    let right = block(source).unwrap().1;
    assert_eq!(left, right);
}

#[test]
fn basic_block_stops() {
    let source = "alfa bravo\n\n";
    let left = Block::Paragraph {
        spans: vec![
            Span::WordPart {
                text: "alfa".to_string(),
            },
            Span::Space {
                text: " ".to_string(),
            },
            Span::WordPart {
                text: "bravo".to_string(),
            },
        ],
    };
    let right = block(source).unwrap().1;
    assert_eq!(left, right);
}

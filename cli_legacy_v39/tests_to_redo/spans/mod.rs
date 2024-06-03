use neopoligengine::span::*;
use pretty_assertions::assert_eq;

#[test]
fn basic_word() {
    let source = "alfa bravo";
    let left = vec![
        Span::WordPart {
            text: "alfa".to_string(),
        },
        Span::Space {
            text: " ".to_string(),
        },
        Span::WordPart {
            text: "bravo".to_string(),
        },
    ];
    let right = spans(source).unwrap().1;
    assert_eq!(left, right);
}

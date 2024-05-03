use neopoligengine::span::*;
use pretty_assertions::assert_eq;

#[test]
fn basic_word() {
    let source = "alfa";
    let left = Span::WordPart {
        text: "alfa".to_string(),
    };
    let right = span(source).unwrap().1;
    assert_eq!(left, right);
}

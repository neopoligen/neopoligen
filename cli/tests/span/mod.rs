use neopoligengine::span::*;
use pretty_assertions::assert_eq;

#[test]
fn basic_word_part() {
    let source = "alfa";
    let left = Span::WordPart {
        text: "alfa".to_string(),
    };
    let right = span(source).unwrap().1;
    assert_eq!(left, right);
}

#[test]
fn single_space() {
    let source = " ";
    let left = Span::Space {
        text: " ".to_string(),
    };
    let right = span(source).unwrap().1;
    assert_eq!(left, right);
}

#[test]
fn single_newline() {
    let source = "\n";
    let left = Span::Newline {
        text: "\n".to_string(),
    };
    let right = span(source).unwrap().1;
    assert_eq!(left, right);
}

use neopoligengine::span_v39::*;
use pretty_assertions::assert_eq;

#[test]
fn word_part_basic() {
    let source = "alfa ";
    let left = (
        " ",
        SpanV39 {
            attrs: vec![],
            source_text: "alfa".to_string(),
            parsed_text: "alfa".to_string(),
            kind: SpanV39Kind::WordPart,
        },
    );
    let right = word_part_v39(source).unwrap();
    assert_eq!(left, right);
}

#[test]
fn word_part_not_line_ending() {
    let source = "alfa\n";
    let left = (
        "\n",
        SpanV39 {
            attrs: vec![],
            source_text: "alfa".to_string(),
            parsed_text: "alfa".to_string(),
            kind: SpanV39Kind::WordPart,
        },
    );
    let right = word_part_v39(source).unwrap();
    assert_eq!(left, right);
}

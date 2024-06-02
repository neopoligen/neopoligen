pub mod code_shorthand;
pub mod integration;

use neopoligengine::span_v39::*;
use pretty_assertions::assert_eq;

#[test]
fn newline_basic() {
    let source = "\n";
    let left = (
        "",
        SpanV39 {
            kind: SpanV39Kind::Space {
                text: " ".to_string(),
            },
        },
    );
    let right = newline_v39(source).unwrap();
    assert_eq!(left, right);
}

#[test]
fn structure_empty_until_newline_or_eof_basic() {
    let source = "\n";
    let left = ("", "");
    let right = structure_empty_until_newline_or_eof(source).unwrap();
    assert_eq!(left, right);
}

#[test]
fn structure_empty_until_newline_or_eof_with_leading_spaces() {
    let source = "   \n";
    let left = ("", "");
    let right = structure_empty_until_newline_or_eof(source).unwrap();
    assert_eq!(left, right);
}

#[test]
fn structure_empty_until_newline_or_eof_at_eof() {
    let source = "";
    let left = ("", "");
    let right = structure_empty_until_newline_or_eof(source).unwrap();
    assert_eq!(left, right);
}

#[test]
fn space_basic() {
    let source = " ";
    let left = (
        "",
        SpanV39 {
            kind: SpanV39Kind::Space {
                text: " ".to_string(),
            },
        },
    );
    let right = space_v39(source).unwrap();
    assert_eq!(left, right);
}

#[test]
fn word_part_basic() {
    let source = "alfa ";
    let left = (
        " ",
        SpanV39 {
            kind: SpanV39Kind::WordPart {
                text: "alfa".to_string(),
            },
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
            kind: SpanV39Kind::WordPart {
                text: "alfa".to_string(),
            },
        },
    );
    let right = word_part_v39(source).unwrap();
    assert_eq!(left, right);
}

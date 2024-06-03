use neopoligengine::span_v39::*;
use pretty_assertions::assert_eq;

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

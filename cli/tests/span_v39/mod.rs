pub mod backtick;
pub mod code_shorthand;
pub mod newline;
pub mod space;
pub mod structure;
pub mod word_part;

use minijinja::Value;
use neopoligengine::span_v39::SpanV39;
use pretty_assertions::assert_eq;

#[test]
fn classes_test_add_a_class() {
    let span = SpanV39::mock1_code_shorthand_without_attrs();
    let left = vec!["new-class"];
    let right = span.classes(&[Value::from("new-class")]);
    assert_eq!(left, right);
}

#[test]
fn classes_test() {
    let span = SpanV39::mock2_code_shorthand_with_attrs_and_language();
    let left = vec!["green"];
    let right = span.classes(&[]);
    assert_eq!(left, right);
}

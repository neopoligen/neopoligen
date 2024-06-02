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
fn classes_test() {
    let span = SpanV39::mock1_code_shorthand_with_attrs();
    let left = vec!["green"];
    let right = span.classes(&[Value::from(None::<String>)]);
    assert_eq!(left, right);
}

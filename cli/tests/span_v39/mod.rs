pub mod backtick;
pub mod code_shorthand;
pub mod link_shorthand;
pub mod newline;
pub mod space;
pub mod structure;
pub mod word_part;

use minijinja::Value;
//use neopoligengine::span_attr_v39::*;
use neopoligengine::span_v39::SpanV39;
use pretty_assertions::assert_eq;

// NOTE: There's not a simple way to do this because of the
// way minijinja wraps errors. This is tested in the
// outputs instead
// #[test]
// fn attrs_basic() {
//     let span = SpanV39::mock2_code_shorthand_with_attrs_and_language();
//     let attrs = vec![
//         SpanAttrV39 {
//             source_text: "|class: green".to_string(),
//             kind: neopoligengine::span_attr_v39::SpanAttrV39Kind::KeyValue {
//                 key: "class".to_string(),
//                 value: "green".to_string(),
//             },
//         },
//         SpanAttrV39 {
//             source_text: "|id: alfa".to_string(),
//             kind: neopoligengine::span_attr_v39::SpanAttrV39Kind::KeyValue {
//                 key: "id".to_string(),
//                 value: "alfa".to_string(),
//             },
//         },
//     ];
//     let left = Ok(Value::make_object_iterable(attrs.clone(), |attr_set| {
//         Box::new(attr_set.iter().cloned().filter_map(|attr| match attr.kind {
//             SpanAttrV39Kind::KeyValue { .. } => Some(Value::from_object(attr)),
//             _ => None,
//         }))
//     }));
//     let right = span.attrs(&[]);
//     assert_eq!(left, right);
// }

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

use neopoligengine::section_attr_v39::{self, *};
use pretty_assertions::assert_eq;

#[test]
fn key_value_basic_attr_with_newline() {
    let source = "-- alfa: bravo\n";
    let left = (
        "",
        SectionAttrV39 {
            kind: SectionAttrV39Kind::KeyValue {
                key: "alfa".to_string(),
                value: "bravo".to_string(),
            },
        },
    );
    let right = section_attr_v39(source).unwrap();
    assert_eq!(left, right);
}

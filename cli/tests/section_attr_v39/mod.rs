use neopoligengine::section_attr_v39::*;
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

#[test]
fn key_value_basic_attr_without_newline() {
    let source = "-- charlie: delta";
    let left = (
        "",
        SectionAttrV39 {
            kind: SectionAttrV39Kind::KeyValue {
                key: "charlie".to_string(),
                value: "delta".to_string(),
            },
        },
    );
    let right = section_attr_v39(source).unwrap();
    assert_eq!(left, right);
}

#[test]
fn flag_basic_attr_with_newline() {
    let source = "-- foxtrot\n";
    let left = (
        "",
        SectionAttrV39 {
            kind: SectionAttrV39Kind::Flag {
                flag: "foxtrot".to_string(),
            },
        },
    );
    let right = section_attr_v39(source).unwrap();
    assert_eq!(left, right);
}

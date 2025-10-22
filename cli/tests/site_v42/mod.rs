use minijinja::Value;
use neopoligengine::bound::*;
use neopoligengine::section_key_value_attr_payload_v42::*;
use neopoligengine::section_v42::basic::*;
use neopoligengine::site_config::*;
use neopoligengine::site_v42::*;
use neopoligengine::span_v42::plaintext::PlainTextSpan;
use neopoligengine::span_v42::SpanV42;
use pretty_assertions::assert_eq;
use std::collections::BTreeMap;

#[test]
fn get_section_attrs_empty_test() {
    let site = SiteV42b {
        config: SiteConfigV42::mock1_basic(),
        generated_at: None,
        images: BTreeMap::new(),
        pages: BTreeMap::new(),
        pages_without_ids: BTreeMap::new(),
    };
    let section = Value::from_serialize(BasicSectionV42 {
        attrs: BTreeMap::new(),
        bound: Bound::Full,
        children: vec![],
        end_section: None,
        flags: vec![],
        kind: "div".to_string(),
        neo_body: "".to_string(),
        neo_source: "".to_string(),
        template_list: vec![],
    });
    let expected = Value::from(r#" class="div-section""#);
    let got = site.get_section_attrs(&[section]).unwrap();
    assert_eq!(expected, got);
}

#[test]
#[ignore]
fn get_section_attrs_with_id() {
    let site = SiteV42b {
        config: SiteConfigV42::mock1_basic(),
        generated_at: None,
        images: BTreeMap::new(),
        pages: BTreeMap::new(),
        pages_without_ids: BTreeMap::new(),
    };
    let mut attrs: BTreeMap<String, Vec<SectionKeyValueAttrPayloadV42>> = BTreeMap::new();
    attrs.insert(
        "id".to_string(),
        vec![SectionKeyValueAttrPayloadV42 {
            text: "alfa".to_string(),
            spans: vec![SpanV42::PlainText(PlainTextSpan {
                kind: "plain-text".to_string(),
                text: "alfa".to_string(),
            })],
        }],
    );
    let section = Value::from_serialize(BasicSectionV42 {
        attrs,
        bound: Bound::Full,
        children: vec![],
        end_section: None,
        flags: vec![],
        kind: "div-test".to_string(),
        neo_body: "".to_string(),
        neo_source: "".to_string(),
        template_list: vec![],
    });
    let left = Value::from(r#" class="div-test-section" id="alfa""#);
    let right = site.get_section_attrs(&[section]).unwrap();
    assert_eq!(left, right);
}

// #[test]
// fn get_section_attrs_with_attr_class() {
//     let site = SiteV42b {
//         config: SiteConfigV42::mock1_basic(),
//         generated_at: None,
//         pages: BTreeMap::new(),
//         pages_without_ids: BTreeMap::new(),
//     };
//     let mut attrs: BTreeMap<String, SectionKeyValueAttrPayloadV42> = BTreeMap::new();
//     attrs.insert(
//         "class".to_string(),
//         SectionKeyValueAttrPayloadV42 {
//             text: "bravo".to_string(),
//             spans: vec![SpanV42::PlainText(PlainTextSpan {
//                 kind: "plain-text".to_string(),
//                 text: "bravo".to_string(),
//             })],
//         },
//     );
//     let section = Value::from_serialize(BasicSectionV42 {
//         attrs,
//         bound: Bound::Full,
//         children: vec![],
//         end_section: None,
//         flags: vec![],
//         kind: "title".to_string(),
//         template_list: vec![],
//     });
//     let left = Value::from(r#" class="title-section bravo""#);
//     let right = site.get_section_attrs(&[section]).unwrap();
//     assert_eq!(left, right);
// }

// #[test]
// fn get_section_attrs_with_extra_class() {
//     let site = SiteV42b {
//         config: SiteConfigV42::mock1_basic(),
//         generated_at: None,
//         pages: BTreeMap::new(),
//         pages_without_ids: BTreeMap::new(),
//     };
//     let attrs: BTreeMap<String, SectionKeyValueAttrPayloadV42> = BTreeMap::new();
//     let section = Value::from_serialize(BasicSectionV42 {
//         attrs,
//         bound: Bound::Full,
//         children: vec![],
//         end_section: None,
//         flags: vec![],
//         kind: "p".to_string(),
//         template_list: vec![],
//     });
//     let left = Value::from(r#" class="p-section charlie""#);
//     let right = site
//         .get_section_attrs(&[section, Value::from("charlie")])
//         .unwrap();
//     assert_eq!(left, right);
// }

// #[test]
// fn get_section_attrs_with_attr_class_and_extra_class_and_id() {
//     let site = SiteV42b {
//         config: SiteConfigV42::mock1_basic(),
//         generated_at: None,
//         pages: BTreeMap::new(),
//         pages_without_ids: BTreeMap::new(),
//     };
//     let mut attrs: BTreeMap<String, SectionKeyValueAttrPayloadV42> = BTreeMap::new();
//     attrs.insert(
//         "id".to_string(),
//         SectionKeyValueAttrPayloadV42 {
//             text: "delta".to_string(),
//             spans: vec![SpanV42::PlainText(PlainTextSpan {
//                 kind: "plain-text".to_string(),
//                 text: "delta".to_string(),
//             })],
//         },
//     );
//     attrs.insert(
//         "class".to_string(),
//         SectionKeyValueAttrPayloadV42 {
//             text: "echo".to_string(),
//             spans: vec![SpanV42::PlainText(PlainTextSpan {
//                 kind: "plain-text".to_string(),
//                 text: "echo".to_string(),
//             })],
//         },
//     );
//     let section = Value::from_serialize(BasicSectionV42 {
//         attrs,
//         bound: Bound::Full,
//         children: vec![],
//         end_section: None,
//         flags: vec![],
//         kind: "aside".to_string(),
//         template_list: vec![],
//     });
//     let left = Value::from(r#" class="aside-section echo foxtrot" id="delta""#);
//     let right = site
//         .get_section_attrs(&[section, Value::from("foxtrot")])
//         .unwrap();
//     assert_eq!(left, right);
// }

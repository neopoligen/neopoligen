use neopoligengine::section_v39::raw::*;
use neopoligengine::{
    section_attr_v39::{SectionAttrV39, SectionAttrV39Kind},
    section_v39::*,
    site_config::SiteConfig,
    span_v39::{SpanV39, SpanV39Kind},
};
use pretty_assertions::assert_eq;

#[test]
fn raw_section_full_core_test() {
    let config = SiteConfig::mock1();
    let source = "-- code\n\nHello World";
    let left = (
        "",
        SectionV39 {
            attrs: vec![],
            bounds: SectionV39Bounds::Full,
            kind: SectionV39Kind::Raw {
                text: Some("Hello World".to_string()),
            },
            r#type: "code".to_string(),
        },
    );
    let right = raw_section_full_v39(source, &config.sections, &config.spans).unwrap();
    assert_eq!(left, right);
}

// #[test]
// fn basic_section_full_with_attrs() {
//     let config = SiteConfig::mock1();
//     let source = "-- div\n-- class: green\n\nalfa bravo";
//     let left = (
//         "",
//         SectionV39 {
//             attrs: vec![SectionAttrV39 {
//                 kind: SectionAttrV39Kind::KeyValue {
//                     key: "class".to_string(),
//                     value: "green".to_string(),
//                 },
//             }],
//             bounds: SectionV39Bounds::Full,
//             kind: SectionV39Kind::Basic {
//                 children: vec![SectionV39 {
//                     attrs: vec![],
//                     bounds: SectionV39Bounds::Full,
//                     kind: SectionV39Kind::Block {
//                         spans: vec![
//                             SpanV39 {
//                                 kind: SpanV39Kind::WordPart {
//                                     text: "alfa".to_string(),
//                                 },
//                             },
//                             SpanV39 {
//                                 kind: SpanV39Kind::Space {
//                                     text: " ".to_string(),
//                                 },
//                             },
//                             SpanV39 {
//                                 kind: SpanV39Kind::WordPart {
//                                     text: "bravo".to_string(),
//                                 },
//                             },
//                         ],
//                     },
//                     r#type: "block-of-text".to_string(),
//                 }],
//             },
//             r#type: "div".to_string(),
//         },
//     );
//     let right = start_or_full_section_v39(source, &config.sections, &config.spans).unwrap();
//     assert_eq!(left, right);
// }

// #[test]
// fn basic_section_start_end_core_test() {
//     let config = SiteConfig::mock1();
//     let source = "-- note/\n\nHello World\n\n-- /note";
//     let left = (
//         "",
//         SectionV39 {
//             attrs: vec![],
//             bounds: SectionV39Bounds::Start,
//             kind: SectionV39Kind::Basic {
//                 children: vec![
//                     SectionV39 {
//                         attrs: vec![],
//                         bounds: SectionV39Bounds::Full,
//                         kind: SectionV39Kind::Block {
//                             spans: vec![
//                                 SpanV39 {
//                                     kind: SpanV39Kind::WordPart {
//                                         text: "Hello".to_string(),
//                                     },
//                                 },
//                                 SpanV39 {
//                                     kind: SpanV39Kind::Space {
//                                         text: " ".to_string(),
//                                     },
//                                 },
//                                 SpanV39 {
//                                     kind: SpanV39Kind::WordPart {
//                                         text: "World".to_string(),
//                                     },
//                                 },
//                             ],
//                         },
//                         r#type: "block-of-text".to_string(),
//                     },
//                     SectionV39 {
//                         attrs: vec![],
//                         bounds: SectionV39Bounds::End,
//                         kind: SectionV39Kind::Basic { children: vec![] },
//                         r#type: "note".to_string(),
//                     },
//                 ],
//             },
//             r#type: "note".to_string(),
//         },
//     );
//     let right = basic_section_start_v39(source, &config.sections, &config.spans).unwrap();
//     assert_eq!(left, right);
// }

use neopoligengine::{
    ast_v39,
    section_attr_v39::{SectionAttrV39, SectionAttrV39Kind},
    section_v39::{SectionV39, SectionV39Bounds, SectionV39Kind},
    site_config::SiteConfig,
    span_v39::{SpanV39, SpanV39Kind},
};
use pretty_assertions::assert_eq;

#[test]
fn ast_basic_test() {
    let config = SiteConfig::mock1();
    let source =
        "-- title\n\nalfa\n\n-- metadata\n-- id: bravo123\n-- created: 2024-05-31T14:13:40-04:00";
    let left = vec![
        SectionV39 {
            attrs: vec![],
            bounds: SectionV39Bounds::Full,
            kind: SectionV39Kind::Basic {
                children: vec![SectionV39 {
                    attrs: vec![],
                    bounds: SectionV39Bounds::Full,
                    kind: SectionV39Kind::Block {
                        spans: vec![SpanV39 {
                            source_text: "alfa".to_string(),
                            parsed_text: "alfa".to_string(),
                            kind: SpanV39Kind::WordPart,
                        }],
                    },
                    r#type: "block-of-text".to_string(),
                }],
            },
            r#type: "title".to_string(),
        },
        SectionV39 {
            attrs: vec![
                SectionAttrV39 {
                    kind: SectionAttrV39Kind::KeyValue {
                        key: "id".to_string(),
                        value: "bravo123".to_string(),
                    },
                },
                SectionAttrV39 {
                    kind: SectionAttrV39Kind::KeyValue {
                        key: "created".to_string(),
                        value: "2024-05-31T14:13:40-04:00".to_string(),
                    },
                },
            ],
            bounds: SectionV39Bounds::Full,
            kind: SectionV39Kind::Yaml {},
            r#type: "metadata".to_string(),
        },
    ];
    let right = ast_v39::parse(source, &config.sections, &config.spans).unwrap();
    assert_eq!(left, right);
}

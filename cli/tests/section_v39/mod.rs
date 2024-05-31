use neopoligengine::{
    // section_attr_v39::SectionAttrV39Kind,
    section_v39::*,
    site_config::SiteConfig,
    span_v39::{SpanV39, SpanV39Kind},
};
use pretty_assertions::assert_eq;

#[test]
fn basic_section_basic_test() {
    let config = SiteConfig::mock1();
    let source = "-- title\n\nHello World";
    let left = (
        "",
        SectionV39 {
            attrs: vec![],
            bounds: SectionV39Bounds::Full,
            kind: SectionV39Kind::Basic {
                children: vec![SectionV39 {
                    attrs: vec![],
                    bounds: SectionV39Bounds::Full,
                    kind: SectionV39Kind::Block {
                        spans: vec![
                            SpanV39 {
                                kind: SpanV39Kind::WordPart { text: "Hello" },
                            },
                            SpanV39 {
                                kind: SpanV39Kind::Space { text: " " },
                            },
                            SpanV39 {
                                kind: SpanV39Kind::WordPart { text: "World" },
                            },
                        ],
                    },
                }],
                r#type: "title".to_string(),
            },
        },
    );
    let right = start_or_full_section_v39(source, &config.sections, &config.spans).unwrap();
    assert_eq!(left, right);
}

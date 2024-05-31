use neopoligengine::section_attr_v39::{SectionAttrV39, SectionAttrV39Kind};
use neopoligengine::section_v39::yaml::*;
use neopoligengine::{section_v39::*, site_config::SiteConfig};
use pretty_assertions::assert_eq;

#[test]
fn yaml_basic() {
    let config = SiteConfig::mock1();
    let source = "-- metadata\n-- id: alfa1234\n";
    let left = (
        "",
        SectionV39 {
            attrs: vec![SectionAttrV39 {
                kind: SectionAttrV39Kind::KeyValue {
                    key: "id".to_string(),
                    value: "alfa1234".to_string(),
                },
            }],
            bounds: SectionV39Bounds::Full,
            kind: SectionV39Kind::Yaml {},
            r#type: "metadata".to_string(),
        },
    );
    let right = yaml_section_full_v39(source, &config.sections, &config.spans).unwrap();
    assert_eq!(left, right);
}

use neopoligengine::block::*;
use neopoligengine::section::*;
use neopoligengine::section_attr::SectionAttr;
use neopoligengine::site_config::SiteConfig;
use neopoligengine::span::*;
use pretty_assertions::assert_eq;
use serde_json::Value;

#[test]
fn json_section_without_data() {
    let source = "-- metadata\n-- id: someid";
    let config = SiteConfig::mock1();
    let left = Section::Json {
        attrs: vec![SectionAttr::KeyValue {
            key: "id".to_string(),
            value: "someid".to_string(),
        }],
        bounds: SectionBounds::Full,
        source: "-- metadata\n-- id: someid".to_string(),
        data: None,
        r#type: "metadata".to_string(),
    };
    let right = section(source, &config.sections).unwrap().1;
    assert_eq!(left, right);
}

#[test]
fn json_section_with_data() {
    let source = r#"-- metadata
-- id: someid

{ "echo": "delta" }

-- p"#;
    let config = SiteConfig::mock1();
    let left = Section::Json {
        attrs: vec![SectionAttr::KeyValue {
            key: "id".to_string(),
            value: "someid".to_string(),
        }],
        bounds: SectionBounds::Full,
        source: "-- metadata\n-- id: someid\n\n{ \"echo\": \"delta\" }\n\n".to_string(),
        data: Some(serde_json::from_str::<Value>(r#"{ "echo": "delta" }"#).unwrap()),
        r#type: "metadata".to_string(),
    };
    let right = section(source, &config.sections).unwrap().1;
    assert_eq!(left, right);
}

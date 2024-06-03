use neopoligengine::section::{start_or_full_section, Section};
use neopoligengine::site_config::SiteConfig;
use nom::multi::many1;
use pretty_assertions::assert_eq;
use std::collections::BTreeMap;

#[test]
fn code_followed_by_result() {
    let config = SiteConfig::mock1();
    let source = "-- code\n\nsome code\n\n-- results/\n\nsome results\n\n-- /results\n\n\n\n";
    let left = vec![
        Section::Raw {
            attrs: BTreeMap::new(),
            children: vec![],
            flags: vec![],
            bounds: "full".to_string(),
            text: Some("some code".to_string()),
            r#type: "code".to_string(),
        },
        Section::Raw {
            attrs: BTreeMap::new(),
            children: vec![Section::Basic {
                attrs: BTreeMap::new(),
                attr_list: vec![],
                bounds: "end".to_string(),
                children: vec![],
                flags: vec![],
                r#type: "results".to_string(),
            }],
            flags: vec![],
            bounds: "start".to_string(),
            text: Some("some results".to_string()),
            r#type: "results".to_string(),
        },
    ];
    let right = many1(|src| start_or_full_section(src, &config.sections, &config.spans))(source)
        .unwrap()
        .1;
    assert_eq!(left, right);
}

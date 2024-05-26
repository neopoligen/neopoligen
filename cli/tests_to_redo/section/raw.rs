use neopoligengine::section::*;
use neopoligengine::site_config::SiteConfig;
use pretty_assertions::assert_eq;

#[test]
fn raw_full_section() {
    let source = "-- pre\n\nsome text";
    let config = SiteConfig::mock1();
    let left = Section::Raw {
        attrs: vec![],
        bounds: SectionBounds::Full,
        text: "some text".to_string(),
        source: "-- pre\n\nsome text".to_string(),
        r#type: "pre".to_string(),
    };
    let right = section(&source, &config.sections).unwrap().1;
    assert_eq!(left, right);
}

#[test]
fn raw_start_section() {
    let source = "-- pre/\n\nfoxtrot victor\n\n-- /pre";
    let config = SiteConfig::mock1();
    let left = Section::Raw {
        attrs: vec![],
        bounds: SectionBounds::Start,
        text: "foxtrot victor".to_string(),
        source: "-- pre/\n\nfoxtrot victor\n\n".to_string(),
        r#type: "pre".to_string(),
    };
    let right = section(&source, &config.sections).unwrap().1;
    assert_eq!(left, right);
}

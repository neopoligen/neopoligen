use neopoligengine::{section_v39::*, site_config::SiteConfig};
use pretty_assertions::assert_eq;

#[test]
fn basic_section_basic_test() {
    let config = SiteConfig::mock1();
    let source = "-- title\n\nHello World";
    let left = ("", SectionV39 {});
    let right = start_or_full_section_v39(source, &config.sections, &config.spans).unwrap();
    assert_eq!(left, right);
}

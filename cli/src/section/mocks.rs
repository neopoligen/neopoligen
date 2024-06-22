use super::start_or_full_section;
use crate::{section::Section, site_config::SiteConfig};

impl Section {
    pub fn mock1_basic_title_section_no_attrs() -> Section {
        let config = SiteConfig::mock1_basic();
        let source = "-- title\n\nHello World\n\nThis is a title section";
        start_or_full_section(source, &config.sections).unwrap().1
    }

    pub fn mock2_div_with_title_and_template_attrs() -> Section {
        let config = SiteConfig::mock1_basic();
        let source = "-- div\n-- title: Title From Attr\n-- template: template-from-attr\n\nAlfa bravo charlie\n\nDelta echo foxtrot\n\n";
        start_or_full_section(source, &config.sections).unwrap().1
    }

    pub fn mock3_image_with_flag_and_multiple_attrs_with_same_key() -> Section {
        let config = SiteConfig::mock1_basic();
        let source = "-- image\n-- some-image-name\n-- alt: alfa bravo\n-- alt: charlie delta";
        start_or_full_section(source, &config.sections).unwrap().1
    }

    pub fn mock4_youtube_with_tags_and_classes() -> Section {
        let config = SiteConfig::mock1_basic();
        let source = "-- youtube\n-- NPJ1qQraMZI\n-- tag: minecraft\n-- tag: how-to\n-- class: class1 class2\n-- class: class3";
        start_or_full_section(source, &config.sections).unwrap().1
    }

    pub fn mock5_div_with_id() -> Section {
        let config = SiteConfig::mock1_basic();
        let source = "-- div\n-- id: attr-id";
        start_or_full_section(source, &config.sections).unwrap().1
    }

    pub fn mock6_div_with_created_and_updated_and_status() -> Section {
        let config = SiteConfig::mock1_basic();
        let source =
            "-- div\n-- created: 2024-01-01T00:00:00-04:00\n-- updated: 2024-01-02T00:00:00-04:00\n-- status: section-status-example";
        start_or_full_section(source, &config.sections).unwrap().1
    }

    pub fn mock7_div_with_title_and_subtitle() -> Section {
        let config = SiteConfig::mock1_basic();
        let source = "-- div\n-- title: Title From Attr\n-- subtitle: Subtitle From Attr\n-- subtitle: Another Subtitle Line\n\nWhiskey tango\n\n";
        start_or_full_section(source, &config.sections).unwrap().1
    }

    pub fn mock8_metadata_basic() -> Section {
        let config = SiteConfig::mock1_basic();
        let source = "-- metadata\n-- created: 2024-06-10T15:03:01-04:00\n-- id: id_from_metadata";
        start_or_full_section(source, &config.sections).unwrap().1
    }

    pub fn mock9_aria_data() -> Section {
        let config = SiteConfig::mock1_basic();
        let source = "-- div\n-- aria-description: alfa bravo\n-- aria-description: charlie delta\n\nalfa bravo";
        start_or_full_section(source, &config.sections).unwrap().1
    }

    pub fn mock10_data_attrs() -> Section {
        let config = SiteConfig::mock1_basic();
        let source = "-- div\n-- data-test: alfa bravo\n-- data-test: charlie delta\n\nalfa bravo";
        start_or_full_section(source, &config.sections).unwrap().1
    }

    pub fn mock10_attr_and_custom_attr_test() -> Section {
        let config = SiteConfig::mock1_basic();
        let source = "-- div\n-- src: valid-attr-src\n-- url: custom-attr-url\n\nalfa bravo";
        start_or_full_section(source, &config.sections).unwrap().1
    }

    //
}

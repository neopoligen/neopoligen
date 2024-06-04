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
}


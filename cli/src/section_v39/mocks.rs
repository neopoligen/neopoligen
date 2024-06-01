use crate::section_v39::basic_section_full_v39;
use crate::section_v39::SectionV39;
use crate::site_config::SiteConfig;

use super::SectionV39Kind;

impl SectionV39 {
    pub fn mock1_basic_full() -> SectionV39 {
        let config = SiteConfig::mock1();
        let source = "-- title\n\nHello World";
        basic_section_full_v39(source, &config.sections, &config.spans)
            .unwrap()
            .1
    }

    // pub fn mock2_basic_full_attrs() -> SectionV39 {
    //     let config = SiteConfig::mock1();
    //     let source = "-- title\n-- template: show\n-- alt: line 1\n-- alt: line2\n\nHello World";
    //     basic_section_full_v39(source, &config.sections, &config.spans)
    //         .unwrap()
    //         .1
    // }

    //
}

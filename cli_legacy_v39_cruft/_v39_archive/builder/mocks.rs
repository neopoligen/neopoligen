use std::collections::BTreeMap;

use crate::builder::Builder;
use crate::site_config::SiteConfig;

impl Builder {
    pub fn mock1() -> Builder {
        let config = SiteConfig::mock1();
        let pages = BTreeMap::new();
        let issues = vec![];
        Builder {
            config,
            issues,
            pages,
        }
    }
}
